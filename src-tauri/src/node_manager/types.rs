use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    pub sandbox_path: PathBuf,
    pub binary_path: PathBuf,
    pub config_path: PathBuf,
    pub logs_path: PathBuf,
    pub is_initialized: bool,
    pub is_running: bool,
    pub pid: Option<u32>,
}

impl Default for NodeInfo {
    fn default() -> Self {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let sandbox = home.join(".synergy").join("node");
        
        Self {
            sandbox_path: sandbox.clone(),
            binary_path: sandbox.join("bin").join(get_binary_name()),
            config_path: sandbox.join("config").join("node.json"),
            logs_path: sandbox.join("logs").join("node.log"),
            is_initialized: false,
            is_running: false,
            pid: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeStatus {
    pub is_running: bool,
    pub pid: Option<u32>,
    pub uptime: Option<u64>,
    pub version: Option<String>,
    pub block_height: Option<u64>,
    pub peer_count: Option<u32>,
    pub sync_status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallProgress {
    pub step: String,
    pub progress: u8,
    pub message: String,
}

#[cfg(target_os = "windows")]
pub fn get_binary_name() -> String {
    "synergy-node.exe".to_string()
}

#[cfg(not(target_os = "windows"))]
pub fn get_binary_name() -> String {
    "synergy-node".to_string()
}