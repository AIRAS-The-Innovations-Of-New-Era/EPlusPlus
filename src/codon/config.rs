//! Codon configuration module
//! 
//! Handles loading and saving Codon configuration settings

use std::path::PathBuf;
use std::fs;
use serde::{Deserialize, Serialize};
use crate::codon::{CodonConfig, OptimizationLevel, CodonError};

const CONFIG_FILE_NAME: &str = "codon.toml";
const CONFIG_DIR_NAME: &str = ".eppx";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodonProjectConfig {
    pub codon: CodonConfig,
    pub project: ProjectConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub dependencies: Vec<String>,
    pub build_target: String,
    pub output_dir: String,
}

impl Default for CodonProjectConfig {
    fn default() -> Self {
        Self {
            codon: CodonConfig::default(),
            project: ProjectConfig {
                name: "eppx-project".to_string(),
                version: "0.1.0".to_string(),
                description: None,
                dependencies: Vec::new(),
                build_target: "native".to_string(),
                output_dir: "target".to_string(),
            },
        }
    }
}

pub struct ConfigManager {
    config_dir: PathBuf,
    project_config: Option<CodonProjectConfig>,
}

impl ConfigManager {
    pub fn new() -> Result<Self, CodonError> {
        let config_dir = get_config_dir()?;
        Ok(Self {
            config_dir,
            project_config: None,
        })
    }

    pub fn load_project_config(&mut self, project_dir: &PathBuf) -> Result<CodonProjectConfig, CodonError> {
        let config_path = project_dir.join(CONFIG_FILE_NAME);
        
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)
                .map_err(|e| CodonError::ConfigError(e.to_string()))?;
            
            let config: CodonProjectConfig = toml::from_str(&content)
                .map_err(|e| CodonError::ConfigError(e.to_string()))?;
            
            self.project_config = Some(config.clone());
            Ok(config)
        } else {
            // Create default config
            let config = CodonProjectConfig::default();
            self.save_project_config(project_dir, &config)?;
            self.project_config = Some(config.clone());
            Ok(config)
        }
    }

    pub fn save_project_config(&self, project_dir: &PathBuf, config: &CodonProjectConfig) -> Result<(), CodonError> {
        let config_path = project_dir.join(CONFIG_FILE_NAME);
        
        let content = toml::to_string_pretty(config)
            .map_err(|e| CodonError::ConfigError(e.to_string()))?;
        
        fs::write(&config_path, content)
            .map_err(|e| CodonError::ConfigError(e.to_string()))?;
        
        Ok(())
    }

    pub fn get_global_config(&self) -> Result<CodonConfig, CodonError> {
        let global_config_path = self.config_dir.join("global.toml");
        
        if global_config_path.exists() {
            let content = fs::read_to_string(&global_config_path)
                .map_err(|e| CodonError::ConfigError(e.to_string()))?;
            
            let config: CodonConfig = toml::from_str(&content)
                .map_err(|e| CodonError::ConfigError(e.to_string()))?;
            
            Ok(config)
        } else {
            // Create default global config
            let config = CodonConfig::default();
            self.save_global_config(&config)?;
            Ok(config)
        }
    }

    pub fn save_global_config(&self, config: &CodonConfig) -> Result<(), CodonError> {
        let global_config_path = self.config_dir.join("global.toml");
        
        // Ensure config directory exists
        fs::create_dir_all(&self.config_dir)
            .map_err(|e| CodonError::ConfigError(e.to_string()))?;
        
        let content = toml::to_string_pretty(config)
            .map_err(|e| CodonError::ConfigError(e.to_string()))?;
        
        fs::write(&global_config_path, content)
            .map_err(|e| CodonError::ConfigError(e.to_string()))?;
        
        Ok(())
    }

    pub fn update_project_config<F>(&mut self, project_dir: &PathBuf, updater: F) -> Result<(), CodonError>
    where
        F: FnOnce(&mut CodonProjectConfig),
    {
        let mut config = self.load_project_config(project_dir)?;
        updater(&mut config);
        self.save_project_config(project_dir, &config)?;
        self.project_config = Some(config);
        Ok(())
    }

    pub fn add_dependency(&mut self, project_dir: &PathBuf, dependency: &str) -> Result<(), CodonError> {
        self.update_project_config(project_dir, |config| {
            if !config.project.dependencies.contains(&dependency.to_string()) {
                config.project.dependencies.push(dependency.to_string());
            }
        })
    }

    pub fn remove_dependency(&mut self, project_dir: &PathBuf, dependency: &str) -> Result<(), CodonError> {
        self.update_project_config(project_dir, |config| {
            config.project.dependencies.retain(|d| d != dependency);
        })
    }

    pub fn set_optimization_level(&mut self, project_dir: &PathBuf, level: OptimizationLevel) -> Result<(), CodonError> {
        self.update_project_config(project_dir, |config| {
            config.codon.optimization_level = level;
        })
    }

    pub fn enable_gpu(&mut self, project_dir: &PathBuf, enable: bool) -> Result<(), CodonError> {
        self.update_project_config(project_dir, |config| {
            config.codon.enable_gpu = enable;
        })
    }

    pub fn enable_parallel(&mut self, project_dir: &PathBuf, enable: bool) -> Result<(), CodonError> {
        self.update_project_config(project_dir, |config| {
            config.codon.enable_parallel = enable;
        })
    }

    pub fn set_target_arch(&mut self, project_dir: &PathBuf, arch: &str) -> Result<(), CodonError> {
        self.update_project_config(project_dir, |config| {
            config.codon.target_arch = arch.to_string();
        })
    }

    pub fn get_current_config(&self) -> Option<&CodonProjectConfig> {
        self.project_config.as_ref()
    }

    pub fn create_new_project(&self, project_dir: &PathBuf, name: &str, description: Option<&str>) -> Result<(), CodonError> {
        // Ensure project directory exists
        fs::create_dir_all(project_dir)
            .map_err(|e| CodonError::ConfigError(e.to_string()))?;
        
        let mut config = CodonProjectConfig::default();
        config.project.name = name.to_string();
        config.project.description = description.map(|s| s.to_string());
        
        self.save_project_config(project_dir, &config)?;
        
        // Create basic project structure
        let src_dir = project_dir.join("src");
        fs::create_dir_all(&src_dir)
            .map_err(|e| CodonError::ConfigError(e.to_string()))?;
        
        let main_file = src_dir.join("main.eppx");
        if !main_file.exists() {
            let main_content = r#"# E++ main file
print("Hello, E++ with Codon!")

# Example function
def fibonacci(n):
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)

# Test the function
result = fibonacci(10)
print(f"Fibonacci(10) = {result}")
"#;
            fs::write(&main_file, main_content)
                .map_err(|e| CodonError::ConfigError(e.to_string()))?;
        }
        
        Ok(())
    }
}

fn get_config_dir() -> Result<PathBuf, CodonError> {
    let home_dir = dirs::home_dir()
        .ok_or_else(|| CodonError::ConfigError("Could not find home directory".to_string()))?;
    
    Ok(home_dir.join(CONFIG_DIR_NAME))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_config_creation() {
        let temp_dir = TempDir::new().unwrap();
        let config_manager = ConfigManager::new().unwrap();
        
        let project_name = "test-project";
        let description = "A test project";
        
        config_manager.create_new_project(&temp_dir.path().to_path_buf(), project_name, Some(description)).unwrap();
        
        let config_path = temp_dir.path().join(CONFIG_FILE_NAME);
        assert!(config_path.exists());
        
        let mut config_manager = ConfigManager::new().unwrap();
        let config = config_manager.load_project_config(&temp_dir.path().to_path_buf()).unwrap();
        
        assert_eq!(config.project.name, project_name);
        assert_eq!(config.project.description, Some(description.to_string()));
    }

    #[test]
    fn test_dependency_management() {
        let temp_dir = TempDir::new().unwrap();
        let mut config_manager = ConfigManager::new().unwrap();
        
        config_manager.create_new_project(&temp_dir.path().to_path_buf(), "test", None).unwrap();
        
        // Add dependency
        config_manager.add_dependency(&temp_dir.path().to_path_buf(), "numpy").unwrap();
        
        let config = config_manager.load_project_config(&temp_dir.path().to_path_buf()).unwrap();
        assert!(config.project.dependencies.contains(&"numpy".to_string()));
        
        // Remove dependency
        config_manager.remove_dependency(&temp_dir.path().to_path_buf(), "numpy").unwrap();
        
        let config = config_manager.load_project_config(&temp_dir.path().to_path_buf()).unwrap();
        assert!(!config.project.dependencies.contains(&"numpy".to_string()));
    }
} 