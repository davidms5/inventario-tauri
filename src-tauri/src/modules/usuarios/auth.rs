use diesel::prelude::*;
use diesel::OptionalExtension;
use crate::config::db;
use crate::schema::users::password_hash;
use serde::{Serialize, Deserialize};
use bcrypt::verify;
use crate::schema::users::dsl::{users, username, rol};
#[derive(Serialize, Deserialize)]
pub struct  UserInfo {
    pub username: String,
    pub rol: Option<String>,
}

#[derive(Queryable)]
struct DbUser {
    //pub id: Option<i32>,
    pub username: String,
    pub password_hash: String,
    pub rol: Option<String>,
}
//esto por el momento es con conexion a sqlite, luego se cambiara a postgres
pub fn check_login(input_username: String, input_password: String) -> Result<Option<UserInfo>, String> {
    let mut conn = db::get_conn();

    let db_user: Option<DbUser> = users
        .select((username, password_hash, rol))
        .filter(username.eq(&input_username))
        .first::<DbUser>(&mut conn)
        .optional()
        .map_err(|e| e.to_string())?;

    let db_user = match db_user {
        None => return Ok(None),
        Some(u) => u,
    };

    let stored = &db_user.password_hash;

    // Detectar si es bcrypt (hash comienza con `$2` y tiene longitud ~60)
    let is_hashed = stored.starts_with("$2");

    let valid = if is_hashed {
        // Comparación usando bcrypt
        verify(&input_password, stored).unwrap_or(false)
    } else {
        // Comparación directa para legacy
        input_password == *stored
    };

    if valid {
        // Si estaba en texto plano, re-hashear y actualizar en la base
        if !is_hashed {
            if let Ok(new_hash) = bcrypt::hash(&input_password, bcrypt::DEFAULT_COST) {
                // Actualizar el hash en la base
                diesel::update(users.filter(username.eq(&input_username)))
                    .set(password_hash.eq(new_hash))
                    .execute(&mut conn)
                    .map_err(|e| e.to_string())?;
            }
        }

        Ok(Some(UserInfo {
            username: db_user.username,
            rol: db_user.rol,
        }))
    } else {
        Ok(None)
    }
}