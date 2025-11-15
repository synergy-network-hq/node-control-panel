pub mod types;
pub mod init;
pub mod installer;
pub mod config;
pub mod process;
pub mod status;
pub mod logs;

use std::sync::Arc;
use tokio::sync::Mutex;
use types::*;

pub struct NodeManager {
    pub process_handle: Option<tokio::process::Child>,
    pub node_info: NodeInfo,
}

impl NodeManager {
    pub fn new() -> Self {
        Self {
            process_handle: None,
            node_info: NodeInfo::default(),
        }
    }
}

// Re-export command handlers with pub use
pub use init::{init_node_environment, check_initialization};  // Add check_initialization here
pub use installer::install_node_binaries;
pub use process::{start_node, stop_node, restart_node};
pub use status::get_node_status;
pub use logs::{stream_logs, read_log_file};