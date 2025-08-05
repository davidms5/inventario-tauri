use once_cell::sync::Lazy;
use rusqlite::Connection;
use std::sync::Mutex;
use std::env;
use dotenvy::dotenv;
use diesel::{r2d2::{self, ConnectionManager}, SqliteConnection};

static DB_CONN: Lazy<Mutex<Connection>> = Lazy::new(|| {
    let conn = Connection::open("inventory.db").expect("No se pudo abrir la base de datos");
    Mutex::new(conn)
});

pub fn get_connection() -> std::sync::MutexGuard<'static, Connection> {
    DB_CONN.lock().expect("No se pudo bloquear la conexión")
}

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
static POOL: Lazy<DbPool> = Lazy::new(|| {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL missing");
    let mgr = ConnectionManager::<SqliteConnection>::new(url);
    r2d2::Pool::builder().build(mgr).expect("Pool creation failed")
});

pub fn get_conn() -> r2d2::PooledConnection<ConnectionManager<SqliteConnection>> {
    POOL.get().expect("Failed to get a connection from the pool")
}