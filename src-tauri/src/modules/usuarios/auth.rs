use rusqlite::{params, OptionalExtension, Result};
use crate::config::db;

#[derive(serde::Serialize)]
pub struct  UserInfo {
    pub username: String,
    pub rol: String,
}

//esto por el momento es con conexion a sqlite, luego se cambiara a postgres
pub fn check_login(username: String, password_hash: String) -> Result<Option<UserInfo>, String> {
    let conn = db::get_connection();

    let mut stmt = conn
        .prepare("SELECT username, rol FROM users WHERE username = ?1 AND password_hash = ?2")
        .map_err(|e| e.to_string())?;

    let user = stmt
        .query_row(params![username, password_hash], |row| {
            Ok(UserInfo{
                username: row.get(0)?,
                rol: row.get(1)?
            })
        })
        .optional()
        .map_err(|e| e.to_string())?;

    Ok(user)
}