//! Codon runner module
//! 
//! Handles execution of E++ files through Codon backend

use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::io::Write;
use tokio::process::Command as TokioCommand;
use crate::codon::{CodonError, CodonBackend, OptimizationLevel};

pub struct CodonRunner {
    backend: CodonBackend,
    interactive: bool,
    debug_mode: bool,
}

impl CodonRunner {
    pub fn new(backend: CodonBackend) -> Self {
        Self {
            backend,
            interactive: false,
            debug_mode: false,
        }
    }

    pub fn interactive(mut self) -> Self {
        self.interactive = true;
        self
    }

    pub fn debug(mut self) -> Self {
        self.debug_mode = true;
        self
    }

    pub fn run_file(&self, input_file: &PathBuf) -> Result<(), CodonError> {
        if self.interactive {
            self.run_interactive(input_file)
        } else {
            self.run_batch(input_file)
        }
    }

    fn run_batch(&self, input_file: &PathBuf) -> Result<(), CodonError> {
        let codon_path = self.backend.codon_path.as_ref()
            .ok_or_else(|| CodonError::ConfigError("Codon path not set".to_string()))?;

        let mut cmd = Command::new(codon_path);
        cmd.arg("run");

        // Add optimization flags
        match self.backend.config.optimization_level {
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

        // Add parallel support
        if self.backend.config.enable_parallel {
            cmd.arg("--parallel");
        }

        // Add GPU support
        if self.backend.config.enable_gpu {
            cmd.arg("--gpu");
        }

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

    fn run_interactive(&self, input_file: &PathBuf) -> Result<(), CodonError> {
        let codon_path = self.backend.codon_path.as_ref()
            .ok_or_else(|| CodonError::ConfigError("Codon path not set".to_string()))?;

        let mut cmd = Command::new(codon_path);
        cmd.arg("run")
            .arg("--interactive")
            .arg(input_file)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());

        let status = cmd.status()
            .map_err(|e| CodonError::ExecutionFailed(e.to_string()))?;

        if !status.success() {
            return Err(CodonError::ExecutionFailed(
                format!("Process exited with code: {}", status.code().unwrap_or(-1))
            ));
        }

        Ok(())
    }

    pub async fn run_async(&self, input_file: &PathBuf) -> Result<(), CodonError> {
        let codon_path = self.backend.codon_path.as_ref()
            .ok_or_else(|| CodonError::ConfigError("Codon path not set".to_string()))?;

        let mut cmd = TokioCommand::new(codon_path);
        cmd.arg("run");

        // Add optimization flags
        match self.backend.config.optimization_level {
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

        // Add parallel support
        if self.backend.config.enable_parallel {
            cmd.arg("--parallel");
        }

        // Add GPU support
        if self.backend.config.enable_gpu {
            cmd.arg("--gpu");
        }

        cmd.arg(input_file);

        let output = cmd.output()
            .await
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

    pub fn run_with_input(&self, input_file: &PathBuf, input: &str) -> Result<String, CodonError> {
        let codon_path = self.backend.codon_path.as_ref()
            .ok_or_else(|| CodonError::ConfigError("Codon path not set".to_string()))?;

        let mut cmd = Command::new(codon_path);
        cmd.arg("run")
            .arg(input_file)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let mut child = cmd.spawn()
            .map_err(|e| CodonError::ExecutionFailed(e.to_string()))?;

        // Write input to stdin
        if let Some(stdin) = child.stdin.as_mut() {
            stdin.write_all(input.as_bytes())
                .map_err(|e| CodonError::ExecutionFailed(e.to_string()))?;
        }

        let output = child.wait_with_output()
            .map_err(|e| CodonError::ExecutionFailed(e.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(CodonError::ExecutionFailed(stderr.to_string()));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.to_string())
    }

    pub fn run_repl(&self) -> Result<(), CodonError> {
        let codon_path = self.backend.codon_path.as_ref()
            .ok_or_else(|| CodonError::ConfigError("Codon path not set".to_string()))?;

        let mut cmd = Command::new(codon_path);
        cmd.arg("repl")
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());

        let status = cmd.status()
            .map_err(|e| CodonError::ExecutionFailed(e.to_string()))?;

        if !status.success() {
            return Err(CodonError::ExecutionFailed(
                format!("REPL exited with code: {}", status.code().unwrap_or(-1))
            ));
        }

        Ok(())
    }

    pub fn benchmark(&self, input_file: &PathBuf, iterations: usize) -> Result<BenchmarkResult, CodonError> {
        let mut times = Vec::new();

        for i in 0..iterations {
            let start = std::time::Instant::now();
            self.run_file(input_file)?;
            let duration = start.elapsed();
            times.push(duration);

            if self.debug_mode {
                println!("Run {}: {:?}", i + 1, duration);
            }
        }

        let avg_time = times.iter().sum::<std::time::Duration>() / times.len() as u32;
        let min_time = times.iter().min().unwrap();
        let max_time = times.iter().max().unwrap();

        Ok(BenchmarkResult {
            iterations,
            average_time: avg_time,
            min_time: *min_time,
            max_time: *max_time,
            times,
        })
    }
}

#[derive(Debug)]
pub struct BenchmarkResult {
    pub iterations: usize,
    pub average_time: std::time::Duration,
    pub min_time: std::time::Duration,
    pub max_time: std::time::Duration,
    pub times: Vec<std::time::Duration>,
}

impl BenchmarkResult {
    pub fn print_summary(&self) {
        println!("Benchmark Results:");
        println!("  Iterations: {}", self.iterations);
        println!("  Average time: {:?}", self.average_time);
        println!("  Min time: {:?}", self.min_time);
        println!("  Max time: {:?}", self.max_time);
        println!("  Standard deviation: {:?}", self.standard_deviation());
    }

    fn standard_deviation(&self) -> std::time::Duration {
        let avg_nanos = self.average_time.as_nanos() as f64;
        let variance = self.times.iter()
            .map(|t| {
                let diff = t.as_nanos() as f64 - avg_nanos;
                diff * diff
            })
            .sum::<f64>() / self.times.len() as f64;
        
        let std_dev_nanos = variance.sqrt() as u128;
        std::time::Duration::from_nanos(std_dev_nanos as u64)
    }
} 