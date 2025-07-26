use rusqlite::{params, Result};
use serde::{Serialize, Deserialize};
use crate::config::db;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub rol: String,
}

// listar usuarios
#[tauri::command(rename_all = "snake_case")]
pub fn list_users() -> Result<Vec<User>, String> {
    let conn = db::get_connection();

    let mut stmt = conn.prepare("SELECT id, username, rol FROM users")
    .map_err(|e| e.to_string())?;

    let users = stmt
        .query_map([], |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                rol: row.get(2)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<User>, rusqlite::Error>>()
        .map_err(|e| e.to_string())?;

    Ok(users)
}

// Crear usuario
#[tauri::command(rename_all = "snake_case")]
pub fn create_user(username: String, password_hash: String, role: String) -> Result<(), String> {
    let conn = db::get_connection();
    conn.execute(
        "INSERT INTO users (username, password_hash, rol) VALUES (?1, ?2, ?3)",
        params![username, password_hash, role],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

// Eliminar usuario
#[tauri::command(rename_all = "snake_case")]
pub fn delete_user(id: i32) -> Result<(), String> {
    let conn = db::get_connection();
    conn.execute("DELETE FROM users WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

//editar usuario
#[tauri::command(rename_all = "snake_case")]
pub fn update_user(id: i32, password_hash: String, rol: String) -> Result<(), String> {
    let conn = db::get_connection();

    conn.execute(
        "UPDATE users SET password_hash = ?1, rol = ?2 WHERE id = ?3",
        params![password_hash, rol, id],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}
