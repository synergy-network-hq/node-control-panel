#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod node_manager;

use node_manager::{
    NodeManager,
    init_node_environment,
    check_initialization,  // Add this
    install_node_binaries,
    start_node,
    stop_node,
    restart_node,
    get_node_status,
    stream_logs,
    read_log_file,
};
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let node_manager = Arc::new(Mutex::new(NodeManager::new()));

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(node_manager)
        .invoke_handler(tauri::generate_handler![
            init_node_environment,
            check_initialization,  // Add this
            install_node_binaries,
            start_node,
            stop_node,
            restart_node,
            get_node_status,
            stream_logs,
            read_log_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}