-- Your SQL goes here
-- Crear tabla de ventas
CREATE TABLE sales (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  user_id INTEGER NOT NULL REFERENCES users(id),
  fecha TEXT NOT NULL,
  total REAL NOT NULL,
  forma_pago TEXT NOT NULL,
  estado TEXT NOT NULL DEFAULT 'completada',
  deleted_at TEXT NULL
);

-- Detalle de ventas (productos o combos) tabla intermedia
CREATE TABLE sale_items (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  sale_id INTEGER NOT NULL REFERENCES sales(id),
  product_id INTEGER REFERENCES products(id),
  combo_id INTEGER REFERENCES combos(id),
  cantidad INTEGER NOT NULL,
  precio_unitario REAL NOT NULL,
  costo_unitario REAL NOT NULL
);

-- Tabla de combos
CREATE TABLE combos (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  nombre TEXT NOT NULL,
  descripcion TEXT,
  price REAL NOT NULL,
  enabled BOOLEAN NOT NULL DEFAULT 1
);

-- Items dentro de cada combo; tabla intermedia
CREATE TABLE combo_items (
  combo_id INTEGER NOT NULL REFERENCES combos(id),
  product_id INTEGER NOT NULL REFERENCES products(id),
  cantidad INTEGER NOT NULL,
  PRIMARY KEY (combo_id, product_id)
);

-- Opcional: historial de pagos
CREATE TABLE payments (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  sale_id INTEGER NOT NULL REFERENCES sales(id),
  monto REAL NOT NULL,
  forma_pago TEXT NOT NULL,
  referencia TEXT
);

-- Modificar productos para administrar activo
ALTER TABLE products ADD COLUMN enabled BOOLEAN NOT NULL DEFAULT 1;

-- Modificar usuarios para activar/desactivar
ALTER TABLE users ADD COLUMN enabled_add_products BOOLEAN NOT NULL DEFAULT 1;
