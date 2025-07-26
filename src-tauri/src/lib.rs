// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod modules;
pub mod config;
use modules::usuarios::auth;
use modules::usuarios::usuarios::*;
use modules::inventario::inventario::*;
use crate::modules::usuarios::auth::UserInfo;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command(rename_all = "snake_case")]
fn check_login(username: String, password_hash: String) -> Result<Option<UserInfo>, String> {
    auth::check_login(username, password_hash)
        .map_err(|e| format!("Error checking login: {}", e))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            check_login,
            create_user,
            delete_user,
            list_users,
            update_user,
            list_products,
            create_product,
            update_product,
            delete_product,
            export_products_csv
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
