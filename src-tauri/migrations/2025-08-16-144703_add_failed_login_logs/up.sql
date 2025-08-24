-- Your SQL goes here
-- Tabla de logs de intentos fallidos (agregado diario por usuario)
CREATE TABLE  IF NOT EXISTS intentos_fallidos (
  username TEXT NOT NULL,
  intentos INTEGER NOT NULL DEFAULT 0,
  -- guardamos sólo la fecha local del intento
  fecha TEXT NOT NULL DEFAULT (date('now','localtime')),
  PRIMARY KEY (username, fecha)
);

CREATE INDEX IF NOT EXISTS idx_products_nombre ON products(nombre);
CREATE INDEX IF NOT EXISTS idx_products_sku ON products(sku);
