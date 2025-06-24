// CLI module placeholder

use clap::{Parser, Subcommand};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use thiserror::Error;

use crate::parser::parse_eppx_file;
use crate::codegen::generate_cpp_code;

#[derive(Parser, Debug)]
#[clap(name = "eppx", version = "0.1.0", about = "E++ Compiler and Tools")]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    New { project_name: String },
    Build {
        file: PathBuf,
        #[clap(short, long)]
        output: Option<String>,
    },
    Run { file: PathBuf },
    Install { package: String },
    Test,
}

#[derive(Error, Debug)]
pub enum CliError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parser error: {0}")]
    Parser(String),
    #[error("Codegen error: {0}")]
    Codegen(String),
    #[error("Compilation error: {0}")]
    Compilation(String),
    #[error("Execution error: {0}")]
    Execution(String),
    #[error("File not found: {0}")]
    FileNotFound(PathBuf),
    #[error("Feature not yet implemented: {0}")]
    NotImplemented(String),
    #[error("Failed to create project directory '{0}': {1}")]
    ProjectCreation(String, std::io::Error),
}

pub fn handle_new_project(project_name: &str) -> Result<String, CliError> {
    let project_path = Path::new(project_name);
    if project_path.exists() {
        return Err(CliError::ProjectCreation(
            project_name.to_string(),
            std::io::Error::new(std::io::ErrorKind::AlreadyExists, "Project directory already exists"),
        ));
    }
    fs::create_dir_all(project_path.join("src"))
        .map_err(|e| CliError::ProjectCreation(project_name.to_string(), e))?;
    fs::write(
        project_path.join("src/main.eppx"),
        "print(\"Hello from new E++ project!\")\n",
    )
    .map_err(|e| CliError::ProjectCreation(project_name.to_string(), e))?;
    fs::write(
        project_path.join(".eppx_ignore"),
        ".eppx_build/\n.eppx_packages/\n",
    )
    .map_err(|e| CliError::ProjectCreation(project_name.to_string(), e))?;
    Ok(format!("Successfully created E++ project '{}'", project_name))
}

pub fn handle_build(file_path: &Path, output_name: Option<&str>) -> Result<String, CliError> {
    if !file_path.exists() {
        return Err(CliError::FileNotFound(file_path.to_path_buf()));
    }    println!("Building {}...", file_path.display());
    let ast = parse_eppx_file(file_path).map_err(CliError::Parser)?;
    // DEBUG: Print the AST to verify method bodies
    println!("{:#?}", ast);
    let cpp_code = generate_cpp_code(&ast).map_err(CliError::Codegen)?;
    let build_dir = Path::new(".eppx_build");
    fs::create_dir_all(build_dir)?;
    let exec_name = output_name.unwrap_or_else(|| {
        file_path.file_stem().map_or("a.out", |s| s.to_str().unwrap_or("a.out"))
    });
    let exec_path = build_dir.join(exec_name);
    let cpp_file_path = build_dir.join(format!("{}.cpp", exec_name));
    fs::write(&cpp_file_path, cpp_code)?;
    println!("Generated C++ source: {}", cpp_file_path.display());
    let compiler_output = Command::new("g++")
        .arg(&cpp_file_path)
        .arg("-o")
        .arg(&exec_path)
        .arg("-std=c++17")
        .output();
    let compiler_output = match compiler_output {
        Ok(output) if output.status.success() => output,
        _ => {
            println!("g++ failed or not found, trying clang++...");
            Command::new("clang++")
                .arg(&cpp_file_path)
                .arg("-o")
                .arg(&exec_path)
                .arg("-std=c++17")
                .output()?
        }
    };
    if !compiler_output.status.success() {
        let stderr = String::from_utf8_lossy(&compiler_output.stderr);
        return Err(CliError::Compilation(format!(
            "C++ compilation failed:\n{}",
            stderr
        )));
    }
    Ok(format!(
        "Successfully built: {}",
        exec_path.display()
    ))
}

pub fn handle_run(file_path: &Path) -> Result<String, CliError> {
    if !file_path.exists() {
        return Err(CliError::FileNotFound(file_path.to_path_buf()));
    }
    println!("Running {}...", file_path.display());
    let build_dir = Path::new(".eppx_build");
    let exec_name = file_path.file_stem().map_or("a.out", |s| s.to_str().unwrap_or("a.out"));
    let exec_path = build_dir.join(exec_name);
    handle_build(file_path, Some(exec_name))?;
    println!("Executing {}...", exec_path.display());
    let run_output = Command::new(&exec_path).output()?;
    if !run_output.status.success() {
        let stderr = String::from_utf8_lossy(&run_output.stderr);
        return Err(CliError::Execution(format!(
            "Execution failed with code {:?}:\n{}",
            run_output.status.code(),
            stderr
        )));
    }
    let stdout = String::from_utf8_lossy(&run_output.stdout);
    Ok(format!("Output:\n{}", stdout.trim_end()))
}

pub fn handle_install(package_name: &str) -> Result<String, CliError> {
    println!(
        "Attempting to install '{}' using uv (placeholder)...",
        package_name
    );
    let uv_output = Command::new("uv")
        .arg("pip")
        .arg("install")
        .arg(package_name)
        .output();
    match uv_output {
        Ok(output) => {
            if output.status.success() {
                Ok(format!(
                    "Package '{}' installation command sent to uv.\nStdout: {}\nStderr: {}",
                    package_name,
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr)
                ))
            } else {
                Err(CliError::NotImplemented(format!(
                    "uv command failed for package '{}'. Is uv installed and in PATH?\nStderr: {}",
                    package_name,
                    String::from_utf8_lossy(&output.stderr)
                )))
            }
        }
        Err(e) => Err(CliError::NotImplemented(format!(
            "Failed to run uv for package '{}'. Is uv installed and in PATH? Error: {}",
            package_name, e
        ))),
    }
}

pub fn handle_test() -> Result<String, CliError> {
    Err(CliError::NotImplemented(
        "Test runner not yet implemented.".to_string(),
    ))
}
