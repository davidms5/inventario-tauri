// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod modules;
pub mod config;
pub mod schema;
use modules::usuarios::auth;
use modules::usuarios::usuarios::*;
use modules::inventario::inventario::*;
use modules::ventas::historial_ventas::*;
use modules::ventas::venta::*;
use modules::inventario::combos::*;
use modules::cierres::cierres::*;
use crate::modules::usuarios::auth::UserInfo;
use tauri_plugin_dialog::init as dialog_plugin;
use tauri_plugin_fs::init as fs_plugin;
pub mod migrations;
use tauri::Manager;

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
        .setup(|app| {

            // 1) Carpeta de datos de la app (per-user) + archivo DB
            // (En v2 puedes usar `app.path().app_data_dir()`; en v1 era `app.path_resolver().app_data_dir()`).
            // Si tu template es v2, esta línea es correcta:
            let app_dir = app
            .path()
            .app_data_dir()
            .map_err(|e| format!("No se pudo resolver app_data_dir: {e}"))?;
            std::fs::create_dir_all(&app_dir)?;

            //LA UBICACION DE LA DB ES EN APPDATA/ROAMING/COM.INVENTARIO.APP/INVENTORY.DB
            let db_path = app_dir.join("inventory.db");

            // 2) Pool Diesel
            crate::config::db::init_pool(&db_path)?;

            {
                let mut conn = crate::config::db::get_conn(); // PooledConnection
                crate::migrations::run(&mut conn)?;
            }

            // 4) (Opcional) sembrar admin si no existe
            //#[cfg(feature = "bootstrap-admin")]
            //crate::modules::usuarios::bootstrap::ensure_default_admin()
            //    .map_err(|e| format!("bootstrap admin: {e}"))?;

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(dialog_plugin())
        .plugin(fs_plugin())
        .invoke_handler(tauri::generate_handler![
            greet,
            check_login,
            create_user,
            delete_user,
            list_users,
            update_user,
            list_products_paginated,
            create_product,
            update_product,
            delete_product,
            export_table_to_csv,
            list_sales_paginated,
            list_combos,
            create_combo,
            update_combo,
            delete_combo,
            list_active_combos,
            list_products_in_stock,
            create_sale,
            get_sale,
            update_sale_status,
            delete_sale_soft,
            get_combo_with_items,
            create_combo_with_items,
            update_combo_with_items,
            search_products_in_stock,
            preview_daily_closure,
            create_daily_closure,
            list_daily_closures,
            is_date_closed,
            get_today_sales_summary
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
