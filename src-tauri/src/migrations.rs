use diesel::sqlite::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub fn run(conn: &mut SqliteConnection) -> Result<(), Box<dyn std::error::Error>> {

    let applied = conn.run_pending_migrations(MIGRATIONS).map_err(|e| format!("Error running migrations: {}", e))?;
      if !applied.is_empty() {
        // `MigrationVersion` implementa Debug; podés registrar cantidad o nombres
        eprintln!("Migraciones aplicadas: {:?}", applied);
    }
    Ok(())
}
