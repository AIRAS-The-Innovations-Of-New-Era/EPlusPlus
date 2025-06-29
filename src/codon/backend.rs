//! Codon backend module
//! 
//! Provides the core backend functionality for integrating E++ with Codon

use std::path::PathBuf;
use std::process::Command;
use std::collections::HashMap;
use crate::codon::{CodonError, CodonConfig, OptimizationLevel};

pub struct CodonBackendCore {
    config: CodonConfig,
    codon_path: Option<PathBuf>,
    cache_dir: PathBuf,
    compiled_cache: HashMap<String, PathBuf>,
}

impl CodonBackendCore {
    pub fn new(config: CodonConfig) -> Self {
        let cache_dir = std::env::temp_dir().join("eppx-codon-cache");
        
        Self {
            config,
            codon_path: None,
            cache_dir,
            compiled_cache: HashMap::new(),
        }
    }

    pub fn initialize(&mut self) -> Result<(), CodonError> {
        // Create cache directory
        std::fs::create_dir_all(&self.cache_dir)
            .map_err(|e| CodonError::ConfigError(e.to_string()))?;
        
        // Find Codon
        self.find_codon()?;
        
        Ok(())
    }

    pub fn find_codon(&mut self) -> Result<(), CodonError> {
        if let Some(path) = &self.codon_path {
            if path.exists() {
                return Ok(());
            }
        }

        // Try to find codon in PATH
        match which::which("codon") {
            Ok(path) => {
                self.codon_path = Some(path);
                Ok(())
            }
            Err(_) => Err(CodonError::CodonNotFound),
        }
    }

    pub fn compile_eppx_file(&mut self, input_file: &PathBuf) -> Result<PathBuf, CodonError> {
        // Check cache first
        let cache_key = self.get_cache_key(input_file);
        if let Some(cached_path) = self.compiled_cache.get(&cache_key) {
            if cached_path.exists() {
                return Ok(cached_path.clone());
            }
        }

        // Convert E++ to Python-compatible code
        let python_code = self.convert_eppx_to_python(input_file)?;
        
        // Create temporary Python file
        let temp_python_file = self.cache_dir.join(format!("{}.py", cache_key));
        std::fs::write(&temp_python_file, python_code)?;
        
        // Compile with Codon
        let output_file = self.cache_dir.join(format!("{}_compiled", cache_key));
        self.compile_with_codon(&temp_python_file, &output_file)?;
        
        // Cache the result
        self.compiled_cache.insert(cache_key, output_file.clone());
        
        Ok(output_file)
    }

    pub fn compile_with_codon(&self, input_file: &PathBuf, output_file: &PathBuf) -> Result<(), CodonError> {
        let codon_path = self.codon_path.as_ref()
            .ok_or_else(|| CodonError::ConfigError("Codon path not set".to_string()))?;

        let mut cmd = Command::new(codon_path);
        cmd.arg("build");

        // Add optimization flags
        match self.config.optimization_level {
            OptimizationLevel::Debug => {
                cmd.arg("--debug");
            }
            OptimizationLevel::Release => {
                cmd.arg("--release");
            }
            OptimizationLevel::Optimized => {
                cmd.arg("--release");
                cmd.arg("--optimize");
            }
        }

        // Add target architecture
        if self.config.target_arch != "native" {
            cmd.arg("--target").arg(&self.config.target_arch);
        }

        // Add parallel support
        if self.config.enable_parallel {
            cmd.arg("--parallel");
        }

        // Add GPU support
        if self.config.enable_gpu {
            cmd.arg("--gpu");
        }

        cmd.arg("-o").arg(output_file);
        cmd.arg(input_file);

        let output = cmd.output()
            .map_err(|e| CodonError::CompilationFailed(e.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(CodonError::CompilationFailed(stderr.to_string()));
        }

        Ok(())
    }

    pub fn run_compiled_file(&self, compiled_file: &PathBuf) -> Result<(), CodonError> {
        let output = Command::new(compiled_file)
            .output()
            .map_err(|e| CodonError::ExecutionFailed(e.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(CodonError::ExecutionFailed(stderr.to_string()));
        }

        // Print stdout
        let stdout = String::from_utf8_lossy(&output.stdout);
        if !stdout.is_empty() {
            print!("{}", stdout);
        }

        Ok(())
    }

    pub fn generate_llvm_ir(&self, input_file: &PathBuf, output_file: &PathBuf) -> Result<(), CodonError> {
        let codon_path = self.codon_path.as_ref()
            .ok_or_else(|| CodonError::ConfigError("Codon path not set".to_string()))?;

        let mut cmd = Command::new(codon_path);
        cmd.arg("build")
            .arg("--llvm")
            .arg("-o")
            .arg(output_file)
            .arg(input_file);

        let output = cmd.output()
            .map_err(|e| CodonError::CompilationFailed(e.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(CodonError::CompilationFailed(stderr.to_string()));
        }

        Ok(())
    }

    pub fn get_codon_version(&self) -> Result<String, CodonError> {
        let codon_path = self.codon_path.as_ref()
            .ok_or_else(|| CodonError::ConfigError("Codon path not set".to_string()))?;

        let output = Command::new(codon_path)
            .arg("--version")
            .output()
            .map_err(|e| CodonError::ConfigError(e.to_string()))?;

        if !output.status.success() {
            return Err(CodonError::ConfigError("Failed to get Codon version".to_string()));
        }

        let version = String::from_utf8_lossy(&output.stdout);
        Ok(version.trim().to_string())
    }

    pub fn list_available_targets(&self) -> Result<Vec<String>, CodonError> {
        let codon_path = self.codon_path.as_ref()
            .ok_or_else(|| CodonError::ConfigError("Codon path not set".to_string()))?;

        let output = Command::new(codon_path)
            .arg("build")
            .arg("--list-targets")
            .output()
            .map_err(|e| CodonError::ConfigError(e.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(CodonError::ConfigError(stderr.to_string()));
        }

        let targets = String::from_utf8_lossy(&output.stdout);
        let target_list: Vec<String> = targets
            .lines()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        Ok(target_list)
    }

    pub fn clear_cache(&mut self) -> Result<(), CodonError> {
        if self.cache_dir.exists() {
            std::fs::remove_dir_all(&self.cache_dir)
                .map_err(|e| CodonError::ConfigError(e.to_string()))?;
        }
        
        std::fs::create_dir_all(&self.cache_dir)
            .map_err(|e| CodonError::ConfigError(e.to_string()))?;
        
        self.compiled_cache.clear();
        
        Ok(())
    }

    pub fn get_cache_info(&self) -> CacheInfo {
        let cache_size = if self.cache_dir.exists() {
            calculate_dir_size(&self.cache_dir).unwrap_or(0)
        } else {
            0
        };

        CacheInfo {
            cache_dir: self.cache_dir.clone(),
            cache_size,
            cached_files: self.compiled_cache.len(),
        }
    }

    fn get_cache_key(&self, input_file: &PathBuf) -> String {
        // Create a hash based on file path and modification time
        let (meta_str, mod_str) = match std::fs::metadata(input_file) {
            Ok(metadata) => {
                let modified = metadata.modified().unwrap_or(std::time::SystemTime::UNIX_EPOCH);
                (format!("{:?}", metadata), format!("{:?}", modified))
            }
            Err(_) => ("no-metadata".to_string(), "no-modified".to_string()),
        };
        format!("{:x}", md5::compute(format!("{}{}{}", input_file.to_string_lossy(), meta_str, mod_str)))
    }

    fn convert_eppx_to_python(&self, input_file: &PathBuf) -> Result<String, CodonError> {
        // Read the E++ file
        let content = std::fs::read_to_string(input_file)?;
        
        // For now, we'll do a simple conversion
        // In a full implementation, this would use the E++ parser and AST
        let python_code = content.clone();
        
        // Convert .eppx specific syntax to Python
        // This is a simplified conversion - in practice, you'd use your parser
        
        // Remove .eppx specific constructs if any
        // For now, we assume the syntax is already Python-compatible
        
        Ok(python_code)
    }
}

#[derive(Debug)]
pub struct CacheInfo {
    pub cache_dir: PathBuf,
    pub cache_size: u64,
    pub cached_files: usize,
}

impl CacheInfo {
    pub fn print_summary(&self) {
        println!("Codon Cache Info:");
        println!("  Cache directory: {}", self.cache_dir.display());
        println!("  Cache size: {} bytes", self.cache_size);
        println!("  Cached files: {}", self.cached_files);
    }
}

fn calculate_dir_size(path: &PathBuf) -> Result<u64, std::io::Error> {
    let mut total_size = 0;
    
    if path.is_file() {
        return Ok(path.metadata()?.len());
    }
    
    if path.is_dir() {
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            total_size += calculate_dir_size(&entry_path)?;
        }
    }
    
    Ok(total_size)
}

// Add md5 dependency to Cargo.toml if not already present
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_cache_key_generation() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.eppx");
        std::fs::write(&test_file, "print('test')").unwrap();
        
        let config = CodonConfig::default();
        let backend = CodonBackendCore::new(config);
        
        let cache_key = backend.get_cache_key(&test_file);
        assert!(!cache_key.is_empty());
    }

    #[test]
    fn test_cache_info() {
        let config = CodonConfig::default();
        let backend = CodonBackendCore::new(config);
        
        let cache_info = backend.get_cache_info();
        assert_eq!(cache_info.cached_files, 0);
    }
} 