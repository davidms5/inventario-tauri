use serde::{Serialize, Deserialize};
use rusqlite::{params, Result};
use crate::config::db::{self, get_connection};
use csv::Writer;

#[derive(Serialize, Deserialize)]
pub struct Product {
    pub id: i32,
    pub nombre: String,
    pub sku: Option<String>,
    pub descripcion: Option<String>,
    pub price: f64,
    pub quantity: i32,
    pub category: Option<String>,
}



#[tauri::command]
pub fn list_products() -> Result<Vec<Product>, String> {
    let conn = db::get_connection();
    let mut stmt = conn.prepare(
        "SELECT id, nombre, sku, descripcion, price, quantity, category FROM products"
    ).map_err(|e| e.to_string())?;

    let products = stmt.query_map([], |row| {
        Ok(Product {
            id: row.get(0)?,
            nombre: row.get(1)?,
            sku: row.get(2)?,
            descripcion: row.get(3)?,
            price: row.get(4)?,
            quantity: row.get(5)?,
            category: row.get(6)?,
        })
    }).map_err(|e| e.to_string())?
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| e.to_string())?;

    Ok(products)
}

#[tauri::command]
pub fn create_product(nombre: String, sku: Option<String>, descripcion: Option<String>, price: f64, quantity: i32, category: Option<String>) -> Result<(), String> {
    let conn = db::get_connection();
    conn.execute(
        "INSERT INTO products (nombre, sku, descripcion, price, quantity, category) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![nombre, sku, descripcion, price, quantity, category],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn update_product(id: i32, nombre: String, sku: Option<String>, descripcion: Option<String>, price: f64, quantity: i32, category: Option<String>) -> Result<(), String> {
    let conn = db::get_connection();
    conn.execute(
        "UPDATE products SET nombre = ?1, sku = ?2, descripcion = ?3, price = ?4, quantity = ?5, category = ?6 WHERE id = ?7",
        params![nombre, sku, descripcion, price, quantity, category, id],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn delete_product(id: i32) -> Result<(), String> {
    let conn = db::get_connection();
    conn.execute(
        "DELETE FROM products WHERE id = ?1",
        params![id],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn export_table_to_csv(path: String) -> Result<(), String> {
    let conn = get_connection();
    let mut stmt = conn.prepare(
        "SELECT id, nombre, sku, descripcion, price, quantity, category FROM products"
    ).map_err(|e| e.to_string())?;

    let rows = stmt.query_map([], |row| {
        Ok(Product {
            id: row.get(0)?,
            nombre: row.get(1)?,
            sku: row.get(2)?,
            descripcion: row.get(3)?,
            price: row.get(4)?,
            quantity: row.get(5)?,
            category: row.get(6)?,
        })
    }).map_err(|e| e.to_string())?;

    let mut wtr = Writer::from_path(path).map_err(|e| e.to_string())?;
    for prod in rows {
        let p = prod.map_err(|e| e.to_string())?;
        wtr.serialize(p).map_err(|e| e.to_string())?;
    }
    wtr.flush().map_err(|e| e.to_string())?;

    Ok(())
}