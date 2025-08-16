use diesel::prelude::*;
use diesel::OptionalExtension;
use crate::config::db;
use crate::schema::intentos_fallidos;
use crate::schema::users::password_hash;
use serde::{Serialize, Deserialize};
use bcrypt::verify;
use crate::schema::users::dsl::{users, username, rol, id as user_id};
use crate::schema::intentos_fallidos::dsl as IF;
#[derive(Serialize, Deserialize)]
pub struct  UserInfo {
    pub username: String,
    pub rol: Option<String>,
    pub id: i32
}

#[derive(Queryable)]
struct DbUser {
    pub user_id: i32,
    pub username: String,
    pub password_hash: String,
    pub rol: Option<String>,
}


#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = intentos_fallidos)]
pub struct FailedAttempt {
    pub username: String,
    pub intentos: i32,
    pub fecha: String, // YYYY-MM-DD (texto)
}

#[derive(Insertable)]
#[diesel(table_name = intentos_fallidos)]
pub struct NewFailedAttempt<'a> {
    pub username: &'a str,
    pub intentos: i32,         // normalmente 1 en el insert inicial
    // no mandamos `fecha`: la pone el DEFAULT (date('now','localtime'))
}

fn log_failed_attempt(conn: &mut SqliteConnection, user: &str) -> Result<(), String> {
    diesel::insert_into(IF::intentos_fallidos)
        // insert básico: username + intentos=1, la fecha la pone el DEFAULT (hoy)
        .values((IF::username.eq(user), IF::intentos.eq(1)))
        // conflicto en la PK compuesta (username, fecha)
        .on_conflict((IF::username, IF::fecha))
        // si choca, sumamos 1 al contador
        .do_update()
        .set(IF::intentos.eq(IF::intentos + 1))
        .execute(conn)
        .map(|_| ())
        .map_err(|e| e.to_string())
}

//esto por el momento es con conexion a sqlite, luego se cambiara a postgres
pub fn check_login(input_username: String, input_password: String) -> Result<Option<UserInfo>, String> {
    let mut conn = db::get_conn();

    let db_user: Option<DbUser> = users
        .select((user_id, username, password_hash, rol))
        .filter(username.eq(&input_username))
        .first::<DbUser>(&mut conn)
        .optional()
        .map_err(|e| e.to_string())?;

    if db_user.is_none() {
        let _ = log_failed_attempt(&mut conn, &input_username);
        return Ok(None);
    }

    let db_user = db_user.unwrap();

    
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
            id: db_user.user_id,
        }))
    } else {
        let _ = log_failed_attempt(&mut conn, &input_username);
        Ok(None)
    }
}