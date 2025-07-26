use once_cell::sync::Lazy;
use rusqlite::Connection;
use std::sync::Mutex;

static DB_CONN: Lazy<Mutex<Connection>> = Lazy::new(|| {
    let conn = Connection::open("inventory.db").expect("No se pudo abrir la base de datos");
    Mutex::new(conn)
});

pub fn get_connection() -> std::sync::MutexGuard<'static, Connection> {
    DB_CONN.lock().expect("No se pudo bloquear la conexión")
}
