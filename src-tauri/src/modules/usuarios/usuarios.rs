use std::usize;

use diesel::prelude::*;
use rusqlite::{Result};
use serde::{Serialize, Deserialize};
use crate::config::db;
use crate::schema::users::dsl::*;
use crate::schema::users;
use bcrypt::{hash, DEFAULT_COST};

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub rol: Option<String>,
    pub enabled_add_products: bool,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
struct  NewUser <'a> {
    pub username: &'a str,
    pub password_hash: &'a str,
    pub rol: &'a str,
    pub enabled_add_products: bool,
}

#[derive(AsChangeset)]
#[diesel(table_name = users)]
struct UserChanges<'a> {
    pub password_hash: Option<&'a str>,
    pub rol: &'a str,
    pub enabled_add_products: bool,
}

// listar usuarios
#[tauri::command(rename_all = "snake_case")]
pub fn list_users() -> Result<Vec<User>, String> {
    let mut conn = db::get_conn();

    users
        .select((id, username, rol, enabled_add_products))
        //.filter(enabled_add_products.eq(true))
        .load::<User>(&mut conn)
        .map_err(|e| e.to_string())
}

// Crear usuario
#[tauri::command(rename_all = "snake_case")]
pub fn create_user(new_username: String, new_password_hash: String, new_rol: String, new_enabled_add_products: bool) -> Result<usize, String> {
    let mut conn = db::get_conn();
    let hashed = hash(&new_password_hash, DEFAULT_COST).map_err(|e| e.to_string())?;

    let new_user = NewUser {
        username: &new_username,
        password_hash: &hashed,
        rol: &new_rol,
        enabled_add_products: new_enabled_add_products,
    };

    let rows_inserted = diesel::insert_into(users)
    .values(new_user)
    .execute(&mut conn)
    .map_err(|e| e.to_string())?;

    Ok(rows_inserted) // Retorna el número de filas afectadas

}

// Eliminar usuario
#[tauri::command(rename_all = "snake_case")]
pub fn delete_user(target_id: i32) -> Result<usize, String> {
    let mut conn = db::get_conn();
    let rows_deleted = diesel::delete(users.filter(id.eq(target_id)))
        .execute(&mut conn)
        .map_err(|e| e.to_string())?;

    Ok(rows_deleted)
}

//editar usuario
#[tauri::command(rename_all = "snake_case")]
pub fn update_user(target_id: i32, plain_password: String, new_rol: String, new_enabled_add_products: bool) -> Result<usize, String> {
    let mut conn = db::get_conn();

        // Opcional: solo hashear contraseña si se ingresó una no vacía
    let pass_hash_option = if !plain_password.trim().is_empty() {
        Some(
            hash(&plain_password, DEFAULT_COST).map_err(|e| e.to_string())?
        )
    } else {
        None
    };

    let changes = UserChanges {
        password_hash: pass_hash_option.as_deref(),
        rol: &new_rol,
        enabled_add_products: new_enabled_add_products,
    };
    
    diesel::update(users.filter(id.eq(target_id)))
        .set(changes)
        .execute(&mut conn)
        .map_err(|e| e.to_string())?;

    Ok(1) // Retorna 1 si la actualización fue exitosa
}
