-- users
CREATE TABLE IF NOT EXISTS users (
  id                   INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  username             TEXT    NOT NULL UNIQUE,
  password_hash        TEXT    NOT NULL,
  created_at           TIMESTAMP NULL,
  rol                  TEXT    NULL,
  enabled_add_products BOOLEAN NOT NULL DEFAULT 1
);

-- products
CREATE TABLE IF NOT EXISTS products (
  id          INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  nombre      TEXT    NOT NULL,
  sku         TEXT    NULL UNIQUE,
  descripcion TEXT    NULL,
  price       REAL    NOT NULL CHECK (price >= 0),
  quantity    INTEGER NOT NULL CHECK (quantity >= 0),
  category    TEXT    NULL,
  created_at  TEXT    NULL,
  updated_at  TEXT    NULL,
  enabled     BOOLEAN NOT NULL DEFAULT 1
);

INSERT INTO users (username, password_hash, rol, enabled_add_products)
SELECT 'admin', '12345', 'admin', 1
WHERE NOT EXISTS (SELECT 1 FROM users WHERE username = 'admin');