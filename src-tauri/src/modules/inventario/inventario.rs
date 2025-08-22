use rusqlite::{Result};
use crate::{config::db::get_conn, modules::inventario::models::Page};
use csv::Writer;
//use crate::schema::products::dsl::*;
use super::models::{Product, NewProduct, UpdateProduct};
use diesel::{prelude::*, sqlite::Sqlite, QueryDsl, RunQueryDsl, ExpressionMethods};
use std::fs::File;
use crate::schema::combo_items::dsl::{combo_items, product_id as combo_product_id};
use crate::schema::products as products_schema;
use crate::schema::products::dsl as P;

fn filtered_query<'a>(
    term: Option<&str>,
) -> products_schema::BoxedQuery<'a, Sqlite> {
    // base
    let mut q = P::products
        .filter(P::enabled.eq(true))
        .into_boxed::<Sqlite>();

    if let Some(t) = term {
        // patrón OWNED (String) + clones para no prestar &str efímeros
        let pat = format!("%{}%", t);
        q = q.filter(
            P::nombre.like(pat.clone())
                .or(P::sku.like(pat.clone()))
                .or(P::category.like(pat))
        );
    }
    q
}

#[tauri::command]
pub fn list_products_paginated(
    page: i64,
    per_page: i64,
    q: Option<String>,
) -> Result<Page<Product>, String> {
    let mut conn = get_conn();

    let p = page.max(1);
    let size = per_page.clamp(1, 100);
    let off = (p - 1) * size;

    // 1) total: usa SU PROPIA query (count consume la query)
    let total: i64 = filtered_query(q.as_deref())
        .count()
        .get_result(&mut conn)
        .map_err(|e| e.to_string())?;

    // 2) página: reconstruí la query
    let rows = filtered_query(q.as_deref())
        .order(P::id.asc())
        .limit(size)
        .offset(off)
        .select((P::id, P::nombre, P::sku, P::descripcion, P::price, P::quantity, P::category))
        .load::<Product>(&mut conn)
        .map_err(|e| e.to_string())?;

    let total_pages = ((total + size - 1) / size).max(1);

    Ok(Page {
        data: rows,
        total,
        total_pages,
        current_page: p,
        per_page: size,
    })
}
#[tauri::command]
pub fn list_products_in_stock() -> Result<Vec<Product>, String> {
    let mut conn = get_conn();
    P::products
        .filter(P::enabled.eq(true))
        .filter(P::quantity.gt(0))
        .select((P::id, P::nombre, P::sku, P::descripcion, P::price, P::quantity, P::category))
        .load::<Product>(&mut conn)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn search_products_in_stock(query: Option<String>) -> Result<Vec<Product>, String> {
    let mut conn = get_conn();

    // Si no hay término, devolvemos vacío (select vacío por defecto en el front)
    let term = match query {
        Some(q) if !q.trim().is_empty() => q.trim().to_string(),
        _ => return Ok(vec![]),
    };
    let pat = format!("%{}%", term);

    // Por NOMBRE
    let mut by_name: Vec<Product> = P::products
        .filter(P::enabled.eq(true))
        .filter(P::quantity.gt(0))
        .filter(P::nombre.like(&pat))
        .select((
            P::id, P::nombre, P::sku, P::descripcion, P::price, P::quantity, P::category,
        ))
        .load::<Product>(&mut conn)
        .map_err(|e| e.to_string())?;

    // Por SKU (SKU es Nullable en tu schema; esta consulta funciona igual)
    let mut by_sku: Vec<Product> = P::products
        .filter(P::enabled.eq(true))
        .filter(P::quantity.gt(0))
        .filter(P::sku.like(&pat))
        .select((
            P::id, P::nombre, P::sku, P::descripcion, P::price, P::quantity, P::category,
        ))
        .load::<Product>(&mut conn)
        .map_err(|e| e.to_string())?;

    // Merge + dedup por id
    by_name.append(&mut by_sku);
    by_name.sort_by_key(|p| p.id);
    by_name.dedup_by_key(|p| p.id);

    Ok(by_name)
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
    diesel::delete(P::products.filter(P::id.eq(target_id)))
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