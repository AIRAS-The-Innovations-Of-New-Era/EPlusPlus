//! Codon integration module for E++
//! 
//! This module provides integration with Codon, a high-performance Python implementation
//! that compiles to native machine code. It allows E++ to use Codon as a backend
//! for compilation and execution.

pub mod backend;
pub mod installer;
pub mod runner;
pub mod config;

use std::path::PathBuf;
use std::process::Command;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CodonError {
    #[error("Codon not found in PATH")]
    CodonNotFound,
    #[error("Codon installation failed: {0}")]
    InstallationFailed(String),
    #[error("Codon compilation failed: {0}")]
    CompilationFailed(String),
    #[error("Codon execution failed: {0}")]
    ExecutionFailed(String),
    #[error("Invalid Codon configuration: {0}")]
    ConfigError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodonConfig {
    pub version: String,
    pub optimization_level: OptimizationLevel,
    pub target_arch: String,
    pub enable_parallel: bool,
    pub enable_gpu: bool,
    pub python_interop: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationLevel {
    Debug,
    Release,
    Optimized,
}

impl Default for CodonConfig {
    fn default() -> Self {
        Self {
            version: "latest".to_string(),
            optimization_level: OptimizationLevel::Release,
            target_arch: "native".to_string(),
            enable_parallel: true,
            enable_gpu: false,
            python_interop: true,
        }
    }
}

pub struct CodonBackend {
    config: CodonConfig,
    codon_path: Option<PathBuf>,
}

impl CodonBackend {
    pub fn new(config: CodonConfig) -> Self {
        Self {
            config,
            codon_path: None,
        }
    }

    pub fn with_codon_path(mut self, path: PathBuf) -> Self {
        self.codon_path = Some(path);
        self
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

    pub fn compile_eppx_to_codon(&self, input_file: &PathBuf, output_file: &PathBuf) -> Result<(), CodonError> {
        // Convert E++ syntax to Python-compatible syntax for Codon
        let python_code = self.convert_eppx_to_python(input_file)?;
        
        // Write the converted code to a temporary Python file
        std::fs::write(output_file, python_code)?;
        
        Ok(())
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

    pub fn run_with_codon(&self, input_file: &PathBuf) -> Result<(), CodonError> {
        let codon_path = self.codon_path.as_ref()
            .ok_or_else(|| CodonError::ConfigError("Codon path not set".to_string()))?;

        let mut cmd = Command::new(codon_path);
        cmd.arg("run");

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

        // Do NOT add --parallel or --gpu for run
        // These are only valid for build

        cmd.arg(input_file);

        let output = cmd.output()
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
        // Implementation would clear Codon's cache
        Ok(())
    }

    pub fn get_cache_info(&self) -> backend::CacheInfo {
        backend::CacheInfo {
            cache_dir: PathBuf::from("/tmp/codon-cache"),
            cache_size: 0,
            cached_files: 0,
        }
    }
}

pub struct CodonManager {
    pub backend: CodonBackend,
    installed_packages: HashMap<String, String>,
}

impl CodonManager {
    pub fn new() -> Self {
        let config = CodonConfig::default();
        let backend = CodonBackend::new(config);
        
        Self {
            backend,
            installed_packages: HashMap::new(),
        }
    }

    pub fn with_config(config: CodonConfig) -> Self {
        let backend = CodonBackend::new(config);
        
        Self {
            backend,
            installed_packages: HashMap::new(),
        }
    }

    pub fn initialize(&mut self) -> Result<(), CodonError> {
        // Find or install Codon
        if self.backend.find_codon().is_err() {
            // Try to install Codon
            self.install_codon()?;
        }
        
        Ok(())
    }

    pub fn install_codon(&self) -> Result<(), CodonError> {
        // For now, just return success - actual installation would be async
        Ok(())
    }

    pub fn compile_file(&mut self, input_file: &PathBuf, output_file: &PathBuf) -> Result<(), CodonError> {
        self.backend.compile_eppx_to_codon(input_file, output_file)?;
        self.backend.compile_with_codon(output_file, output_file)
    }

    pub fn run_file(&mut self, input_file: &PathBuf) -> Result<(), CodonError> {
        self.backend.run_with_codon(input_file)
    }

    pub fn install_package(&mut self, package_name: &str) -> Result<(), CodonError> {
        // Use Codon's package management capabilities
        let codon_path = self.backend.codon_path.as_ref()
            .ok_or_else(|| CodonError::ConfigError("Codon path not set".to_string()))?;

        let mut cmd = Command::new(codon_path);
        cmd.arg("install").arg(package_name);

        let output = cmd.output()
            .map_err(|e| CodonError::InstallationFailed(e.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(CodonError::InstallationFailed(stderr.to_string()));
        }

        // Store the installed package
        self.installed_packages.insert(package_name.to_string(), "latest".to_string());
        
        Ok(())
    }
} 