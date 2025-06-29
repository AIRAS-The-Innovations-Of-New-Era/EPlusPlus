//! Codon installer module
//! 
//! Handles automatic download and installation of Codon

use std::path::PathBuf;
use std::process::Command;
use std::env;
use tempfile::TempDir;
use crate::codon::CodonError;

pub async fn install_codon(version: &str) -> Result<(), CodonError> {
    println!("Installing Codon version: {}", version);
    
    // Check if we're on a supported platform
    let platform = get_platform()?;
    
    // Download Codon
    let download_url = get_download_url(version, &platform)?;
    let temp_dir = TempDir::new()
        .map_err(|e| CodonError::InstallationFailed(e.to_string()))?;
    
    let download_path = temp_dir.path().join("codon_install.sh");
    
    download_file(&download_url, &download_path).await?;
    
    // Make the installer executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&download_path)
            .map_err(|e| CodonError::InstallationFailed(e.to_string()))?
            .permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&download_path, perms)
            .map_err(|e| CodonError::InstallationFailed(e.to_string()))?;
    }
    
    // Run the installer
    let output = Command::new(&download_path)
        .output()
        .map_err(|e| CodonError::InstallationFailed(e.to_string()))?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(CodonError::InstallationFailed(format!(
            "Codon installer failed: {}", stderr
        )));
    }
    
    println!("Codon installed successfully!");
    Ok(())
}

fn get_platform() -> Result<String, CodonError> {
    let os = env::consts::OS;
    let arch = env::consts::ARCH;
    
    match (os, arch) {
        ("linux", "x86_64") => Ok("linux-x86_64".to_string()),
        ("linux", "aarch64") => Ok("linux-aarch64".to_string()),
        ("macos", "x86_64") => Ok("macos-x86_64".to_string()),
        ("macos", "aarch64") => Ok("macos-aarch64".to_string()),
        ("windows", "x86_64") => Ok("windows-x86_64".to_string()),
        _ => Err(CodonError::InstallationFailed(format!(
            "Unsupported platform: {} {}", os, arch
        ))),
    }
}

fn get_download_url(version: &str, platform: &str) -> Result<String, CodonError> {
    if version == "latest" {
        Ok(format!(
            "https://exaloop.io/install.sh"
        ))
    } else {
        Ok(format!(
            "https://github.com/exaloop/codon/releases/download/v{}/codon-{}-{}.tar.gz",
            version, version, platform
        ))
    }
}

async fn download_file(url: &str, path: &PathBuf) -> Result<(), CodonError> {
    println!("Downloading Codon from: {}", url);
    
    let response = reqwest::get(url)
        .await
        .map_err(|e| CodonError::InstallationFailed(e.to_string()))?;
    
    if !response.status().is_success() {
        return Err(CodonError::InstallationFailed(format!(
            "Failed to download Codon: HTTP {}", response.status()
        )));
    }
    
    let bytes = response.bytes()
        .await
        .map_err(|e| CodonError::InstallationFailed(e.to_string()))?;
    
    std::fs::write(path, bytes)
        .map_err(|e| CodonError::InstallationFailed(e.to_string()))?;
    
    println!("Download completed!");
    Ok(())
}

pub fn get_codon_install_path() -> Result<PathBuf, CodonError> {
    // Try to find Codon in common installation locations
    let possible_paths = vec![
        PathBuf::from("/usr/local/bin/codon"),
        PathBuf::from("/opt/codon/bin/codon"),
        PathBuf::from(format!("{}/.local/bin/codon", env::var("HOME").unwrap_or_default())),
        PathBuf::from(format!("{}/.cargo/bin/codon", env::var("HOME").unwrap_or_default())),
    ];
    
    for path in possible_paths {
        if path.exists() {
            return Ok(path);
        }
    }
    
    Err(CodonError::CodonNotFound)
}

pub fn verify_codon_installation() -> Result<(), CodonError> {
    let codon_path = get_codon_install_path()?;
    
    // Test if Codon works
    let output = Command::new(&codon_path)
        .arg("--version")
        .output()
        .map_err(|e| CodonError::InstallationFailed(e.to_string()))?;
    
    if !output.status.success() {
        return Err(CodonError::InstallationFailed(
            "Codon installation verification failed".to_string()
        ));
    }
    
    let version = String::from_utf8_lossy(&output.stdout);
    println!("Codon version: {}", version.trim());
    
    Ok(())
} 