PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username VARCHAR(50) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
, rol TEXT CHECK (rol IN ('admin', 'empleado')) DEFAULT 'empleado', enabled_add_products BOOLEAN NOT NULL DEFAULT 1);
INSERT INTO users VALUES(1,'admin','$2b$12$ky/hss7I5rmhh.UB21pD7.kXfqUApFSwIqaTZlzxNMpl5GDPVKMa.','2025-07-23 03:12:22','admin',1);
INSERT INTO users VALUES(5,'davidms5','$2b$12$wjce99k3UIGoF14jGsI/hOmEYluMLWbXTiv.l8PQCtXQToyiYGwC.','2025-08-05 04:35:32','admin',1);
CREATE TABLE products (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  nombre TEXT NOT NULL,
  sku TEXT UNIQUE,
  descripcion TEXT,
  price REAL NOT NULL CHECK(price >= 0),
  quantity INTEGER NOT NULL CHECK(quantity >= 0),
  category TEXT,
  created_at TEXT DEFAULT (datetime('now')),
  updated_at TEXT DEFAULT (datetime('now'))
, enabled BOOLEAN NOT NULL DEFAULT 1);
INSERT INTO products VALUES(3,'cafe','cafe34tyh','un buen cafe',45.0,46,'cafe','2025-08-05 05:22:00','2025-08-05 05:22:00',1);
CREATE TABLE __diesel_schema_migrations (
       version VARCHAR(50) PRIMARY KEY NOT NULL,
       run_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
INSERT INTO __diesel_schema_migrations VALUES('20250801051012','2025-08-01 05:10:58');
INSERT INTO __diesel_schema_migrations VALUES('20250804152158','2025-08-04 15:28:59');
INSERT INTO __diesel_schema_migrations VALUES('20250807014040','2025-08-07 01:42:59');
INSERT INTO __diesel_schema_migrations VALUES('20250807020948','2025-08-07 02:10:16');
CREATE TABLE sales (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  user_id INTEGER NOT NULL REFERENCES users(id),
  fecha TEXT NOT NULL,
  total REAL NOT NULL,
  forma_pago TEXT NOT NULL,
  estado TEXT NOT NULL DEFAULT 'completada',
  deleted_at TEXT NULL
);
CREATE TABLE payments (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  sale_id INTEGER NOT NULL REFERENCES sales(id),
  monto REAL NOT NULL,
  forma_pago TEXT NOT NULL,
  referencia TEXT
);
CREATE TABLE IF NOT EXISTS "combos" (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  nombre TEXT NOT NULL,
  descripcion TEXT,
  price REAL NOT NULL,
  enabled BOOLEAN NOT NULL DEFAULT 1
);
INSERT INTO combos VALUES(1,'super combo x2','combo de medialuna mas cafe',8500.0,1);
CREATE TABLE IF NOT EXISTS "sale_items" (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    sale_id INTEGER NOT NULL,
    product_id INTEGER,
    combo_id INTEGER,
    cantidad INTEGER NOT NULL,
    precio_unitario FLOAT NOT NULL,
    costo_unitario FLOAT NOT NULL,
    FOREIGN KEY (sale_id) REFERENCES sales(id),
    FOREIGN KEY (product_id) REFERENCES products(id),
    FOREIGN KEY (combo_id) REFERENCES combos(id) ON DELETE SET NULL
);
CREATE TABLE IF NOT EXISTS "combo_items" (
    combo_id INTEGER NOT NULL,
    product_id INTEGER NOT NULL,
    cantidad INTEGER NOT NULL,
    PRIMARY KEY (combo_id, product_id),
    FOREIGN KEY (combo_id) REFERENCES combos(id) ON DELETE CASCADE,
    FOREIGN KEY (product_id) REFERENCES products(id)
);
DELETE FROM sqlite_sequence;
INSERT INTO sqlite_sequence VALUES('users',5);
INSERT INTO sqlite_sequence VALUES('products',3);
INSERT INTO sqlite_sequence VALUES('combos',2);
INSERT INTO sqlite_sequence VALUES('sale_items',0);
COMMIT;
