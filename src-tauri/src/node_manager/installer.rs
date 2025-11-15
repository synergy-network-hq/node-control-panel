use super::types::*;
use super::NodeManager;
use std::sync::Arc;
use tauri::{State, Emitter, AppHandle};
use tokio::sync::Mutex;
use std::fs::{self, File};
use std::io::Write;

#[tauri::command]
pub async fn install_node_binaries(
    app: AppHandle,
    state: State<'_, Arc<Mutex<NodeManager>>>,
) -> Result<String, String> {
    let manager = state.lock().await;
    let binary_path = manager.node_info.binary_path.clone();
    drop(manager);
    
    // Emit progress: Starting
    let _ = app.emit("install-progress", InstallProgress {
        step: "starting".to_string(),
        progress: 0,
        message: "Preparing to install node binaries...".to_string(),
    });
    
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    
    // Emit progress: Downloading
    let _ = app.emit("install-progress", InstallProgress {
        step: "downloading".to_string(),
        progress: 25,
        message: "Downloading Synergy node binary...".to_string(),
    });
    
    // Simulate download (in production, use reqwest to download from GitHub releases)
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    
    // Create a mock binary for demonstration
    let mock_binary = create_mock_binary();
    
    // Emit progress: Installing
    let _ = app.emit("install-progress", InstallProgress {
        step: "installing".to_string(),
        progress: 75,
        message: "Installing binary...".to_string(),
    });
    
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    
    // Write binary
    let mut file = File::create(&binary_path)
        .map_err(|e| format!("Failed to create binary file: {}", e))?;
    
    file.write_all(&mock_binary)
        .map_err(|e| format!("Failed to write binary: {}", e))?;
    
    // Make executable on Unix systems
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&binary_path)
            .map_err(|e| format!("Failed to get metadata: {}", e))?
            .permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&binary_path, perms)
            .map_err(|e| format!("Failed to set permissions: {}", e))?;
    }
    
    // Emit progress: Complete
    let _ = app.emit("install-progress", InstallProgress {
        step: "complete".to_string(),
        progress: 100,
        message: "Installation complete!".to_string(),
    });
    
    Ok("Node binaries installed successfully".to_string())
}

fn create_mock_binary() -> Vec<u8> {
    // This is a mock binary for demonstration
    // In production, you would download the actual node binary
    let script_content = if cfg!(target_os = "windows") {
        // Windows batch script
        br#"@echo off
echo Synergy Node v1.0.0
echo Starting node...
timeout /t 1 /nobreak >nul
echo Node running on port 8545
echo Press Ctrl+C to stop
:loop
timeout /t 5 /nobreak >nul
echo [INFO] Block #%RANDOM% synced
goto loop
"#.to_vec()
    } else {
        // Unix shell script
        br#"#!/bin/bash
echo "Synergy Node v1.0.0"
echo "Starting node..."
sleep 1
echo "Node running on port 8545"
echo "Press Ctrl+C to stop"
while true; do
    sleep 5
    echo "[INFO] Block #$RANDOM synced"
done
"#.to_vec()
    };
    
    script_content
}