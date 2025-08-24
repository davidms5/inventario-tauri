-- Your SQL goes here
CREATE TABLE  IF NOT EXISTS daily_closures (
  id            INTEGER PRIMARY KEY AUTOINCREMENT,
  fecha         TEXT NOT NULL,                         -- 'YYYY-MM-DD'
  total         REAL NOT NULL DEFAULT 0,
  ventas_count  INTEGER NOT NULL DEFAULT 0,
  created_by    INTEGER NOT NULL REFERENCES users(id),
  created_at    TEXT NOT NULL DEFAULT (datetime('now','localtime')),
  UNIQUE (fecha)                                       -- evita 2 cierres el mismo día
);

-- Totales por forma de pago del cierre
CREATE TABLE  IF NOT EXISTS daily_closure_totals (
  id          INTEGER PRIMARY KEY AUTOINCREMENT,
  closure_id  INTEGER NOT NULL REFERENCES daily_closures(id) ON DELETE CASCADE,
  forma_pago  TEXT NOT NULL,
  monto       REAL NOT NULL DEFAULT 0,
  UNIQUE (closure_id, forma_pago)
);

CREATE INDEX  IF NOT EXISTS idx_daily_closures_fecha ON daily_closures(fecha);
CREATE INDEX  IF NOT EXISTS idx_closure_totals_pago ON daily_closure_totals(forma_pago);
