-- Your SQL goes here
PRAGMA foreign_keys=off;

-- Crear nueva tabla temporal con las restricciones deseadas
CREATE TABLE sale_items_new (
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

INSERT INTO sale_items_new
SELECT id, sale_id, product_id, combo_id, cantidad, precio_unitario, costo_unitario
FROM sale_items;

DROP TABLE sale_items;
ALTER TABLE sale_items_new RENAME TO sale_items;


-- combo_items con ON DELETE CASCADE
CREATE TABLE combo_items_new (
    combo_id INTEGER NOT NULL,
    product_id INTEGER NOT NULL,
    cantidad INTEGER NOT NULL,
    PRIMARY KEY (combo_id, product_id),
    FOREIGN KEY (combo_id) REFERENCES combos(id) ON DELETE CASCADE,
    FOREIGN KEY (product_id) REFERENCES products(id)
);

INSERT INTO combo_items_new
SELECT combo_id, product_id, cantidad
FROM combo_items;

DROP TABLE combo_items;
ALTER TABLE combo_items_new RENAME TO combo_items;

PRAGMA foreign_keys=on;
