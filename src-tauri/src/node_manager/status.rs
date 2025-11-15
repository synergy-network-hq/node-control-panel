use super::types::*;
use super::NodeManager;
use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;

#[tauri::command]
pub async fn get_node_status(
    state: State<'_, Arc<Mutex<NodeManager>>>,
) -> Result<NodeStatus, String> {
    let manager = state.lock().await;
    
    // In a real implementation, you would query the node's RPC API
    // For demonstration, we'll return mock status
    let status = NodeStatus {
        is_running: manager.node_info.is_running,
        pid: manager.node_info.pid,
        uptime: if manager.node_info.is_running {
            Some(3600) // Mock 1 hour uptime
        } else {
            None
        },
        version: Some("1.0.0".to_string()),
        block_height: if manager.node_info.is_running {
            Some(12345)
        } else {
            None
        },
        peer_count: if manager.node_info.is_running {
            Some(8)
        } else {
            None
        },
        sync_status: if manager.node_info.is_running {
            Some("Synced".to_string())
        } else {
            Some("Stopped".to_string())
        },
    };
    
    Ok(status)
}