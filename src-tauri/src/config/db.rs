use once_cell::sync::{OnceCell};
use ::r2d2::PooledConnection;
//use rusqlite::Connection;
use std::{path::Path};
//use std::env;
//use dotenvy::dotenv;
use diesel::{r2d2::{ConnectionManager, Pool}, SqliteConnection};

//static DB_CONN: Lazy<Mutex<Connection>> = Lazy::new(|| {
//    let conn = Connection::open("inventory.db").expect("No se pudo abrir la base de datos");
//    Mutex::new(conn)
//});
//
//pub fn get_connection() -> std::sync::MutexGuard<'static, Connection> {
//    DB_CONN.lock().expect("No se pudo bloquear la conexión")
//}



type Manager = ConnectionManager<SqliteConnection>;
type DbPool = Pool<Manager>;

static POOL: OnceCell<DbPool> = OnceCell::new();

pub fn init_pool(db_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Para Diesel + SQLite, la "database_url" puede ser simplemente la ruta del archivo
    // (p.ej. C:\Users\...\AppData\Local\com.inventario.app\inventory.db)
    let url = db_path.to_string_lossy().into_owned();
    let manager = ConnectionManager::<SqliteConnection>::new(url);
    let pool = Pool::builder().max_size(8).build(manager)?;
    POOL.set(pool).map_err(|_| "DB pool ya inicializado".into())
}

pub fn get_conn() -> PooledConnection<Manager> {
    POOL.get().expect("DB pool no inicializado").get().expect("Sin conexión")
}