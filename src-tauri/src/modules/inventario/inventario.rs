use serde::{Serialize, Deserialize};
use rusqlite::{params, Result};
use tauri::{AppHandle, Manager};
use crate::config::db::{self, get_connection};
use std::fs::File;
use std::io::Write;

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
pub fn export_products_csv(app: AppHandle) -> Result<String, String> {
    let conn = get_connection();
    let mut stmt = conn.prepare(
        "SELECT id, nombre, sku, descripcion, price, quantity, category FROM products"
    ).map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([], |row| {
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

    // Serializar a CSV
    let mut wtr = csv::Writer::from_writer(vec![]);
    wtr.write_record(&["id", "nombre", "sku", "descripcion", "price", "quantity", "category"])
        .map_err(|e| e.to_string())?;
    for p in rows {
        wtr.write_record(&[
            p.id.to_string(),
            p.nombre,
            p.sku.clone().unwrap_or_default(),
            p.descripcion.clone().unwrap_or_default(),
            p.price.to_string(),
            p.quantity.to_string(),
            p.category.clone().unwrap_or_default(),
        ]).map_err(|e| e.to_string())?;
    }
    let data = wtr.into_inner().map_err(|e| e.to_string())?;

    // Obtener directorio AppData desde AppHandle
    let dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let path = dir.join("products_export.csv");

    let mut file = File::create(&path).map_err(|e| e.to_string())?;
    file.write_all(&data).map_err(|e| e.to_string())?;

    Ok(path.to_string_lossy().into_owned())
}