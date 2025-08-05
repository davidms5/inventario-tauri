use rusqlite::{Result};
use crate::config::db::{get_conn};
use csv::Writer;
use crate::schema::products::dsl::*;
use super::models::{Product, NewProduct, UpdateProduct};
use diesel::prelude::*;
use std::fs::File;
use crate::schema::combo_items::dsl::{combo_items, product_id as combo_product_id};

#[tauri::command]
pub fn list_products() -> Result<Vec<Product>, String> {

    let mut conn = get_conn();

    products
        .filter(enabled.eq(true))
        .select((id, nombre, sku, descripcion, price, quantity, category))
        .load::<Product>(&mut conn)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_product(
    nombre_: String,
    sku_: Option<String>,
    descripcion_: Option<String>,
    price_: f32,
    quantity_: i32,
    category_: Option<String>,
) -> Result<usize, String> {
    use crate::schema::products::dsl::*;

    let mut conn = get_conn();

    let new_product = NewProduct {
        nombre: &nombre_,
        sku: sku_.as_deref(),
        descripcion: descripcion_.as_deref(),
        price: price_,
        quantity: quantity_,
        category: category_.as_deref(),
    };

    diesel::insert_into(products)
        .values(&new_product)
        .execute(&mut conn)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_product(
    id_: i32,
    nombre_: String,
    sku_: Option<String>,
    descripcion_: Option<String>,
    price_: f32,
    quantity_: i32,
    category_: Option<String>,
) -> Result<usize, String> {
    use crate::schema::products::dsl::*;

    let mut conn = get_conn();

    let changes = UpdateProduct {
        nombre: &nombre_,
        sku: sku_.as_deref(),
        descripcion: descripcion_.as_deref(),
        price: price_,
        quantity: quantity_,
        category: category_.as_deref(),
    };

    diesel::update(products.filter(id.eq(id_)))
        .set(&changes)
        .execute(&mut conn)
        .map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub fn delete_product(target_id: i32) -> Result<(), String> {

    let mut conn = get_conn();

    // Verificamos si el producto está en algún combo
    let en_combos = combo_items
        .filter(combo_product_id.eq(target_id))
        .count()
        .get_result::<i64>(&mut conn)
        .map_err(|e| e.to_string())?;

    if en_combos > 0 {
        return Err("El producto no puede eliminarse porque forma parte de uno o más combos.".into());
    }

    // Borrado físico del producto (esto solo es posible porque en sale_items el FK es ON DELETE SET NULL)
    diesel::delete(products.filter(id.eq(target_id)))
        .execute(&mut conn)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn export_table_to_csv(path: String) -> Result<(), String> {
    use crate::schema::products::dsl::*;

    let mut conn = get_conn();

    let all_products = products
        .select((id, nombre, sku, descripcion, price, quantity, category))
        .load::<Product>(&mut conn)
        .map_err(|e| e.to_string())?;

    let file = File::create(path).map_err(|e| e.to_string())?;
    let mut wtr = Writer::from_writer(file);

    for p in all_products {
        wtr.serialize(p).map_err(|e| e.to_string())?;
    }
    wtr.flush().map_err(|e| e.to_string())?;

    Ok(())
}