
// src/modules/ventas/sales.rs
use diesel::{prelude::*, RunQueryDsl, ExpressionMethods, QueryDsl};
use chrono::Utc;

use crate::config::db::get_conn;
use crate::schema::{sales, sale_items}; //, products, combos, combo_items
use super::models::*;
//use std::collections::HashMap;

// Utilidad: fecha ISO corta
fn now_ymdhms() -> String {
    Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

/// CREA una venta con items (productos y/o combos)
/// - Descuenta stock (productos directos y los productos que componen combos)
#[tauri::command]
pub fn create_sale(payload: NewSaleRequest) -> Result<i32, String> {
    let mut conn = get_conn();
    use crate::schema::{products::dsl as P, combos::dsl as C, combo_items::dsl as CI};

    // 1) PREVALIDACIÓN (sin tx): calcular requerimientos y detectar faltantes
    let mut errors = Vec::<String>::new();
    let mut total: f32 = 0.0;

    // Para insertar luego
    let mut prepared: Vec<NewSaleItem> = Vec::new();

        for it in &payload.items {
        if let Some(pid) = it.product_id {
            let (_id, nombre, price, qty): (i32, String, f32, i32) = P::products
                .filter(P::id.eq(pid))
                .select((P::id, P::nombre, P::price, P::quantity))
                .first(&mut conn)
                .map_err(|e| e.to_string())?;

            if qty < it.cantidad {
                errors.push(format!(
                    "Stock insuficiente para '{}' (id {}): requerido {}, disponible {}",
                    nombre, _id, it.cantidad, qty
                ));
            }

            total += price * it.cantidad as f32;
            prepared.push(NewSaleItem {
                sale_id: 0, product_id: Some(pid), combo_id: None,
                cantidad: it.cantidad, precio_unitario: price, costo_unitario: 0.0,
            });

        } else if let Some(cid) = it.combo_id {
            let (_id, combo_nombre, price, enabled): (i32, String, f32, bool) = C::combos
                .filter(C::id.eq(cid))
                .select((C::id, C::nombre, C::price, C::enabled))
                .first(&mut conn)
                .map_err(|e| e.to_string())?;
            if !enabled {
                errors.push(format!("El combo '{}' (id {}) está inactivo", combo_nombre, _id));
            }

            let parts: Vec<(i32, i32, String, i32)> = CI::combo_items
                .inner_join(P::products)
                .filter(CI::combo_id.eq(cid))
                .select((CI::product_id, CI::cantidad, P::nombre, P::quantity))
                .load(&mut conn)
                .map_err(|e| e.to_string())?;

            for (_prod_id, cant_en_combo, prod_nombre, stock_disp) in parts {
                let needed = cant_en_combo * it.cantidad;
                if stock_disp < needed {
                    errors.push(format!(
                        "'{}' del combo '{}' sin stock suficiente: requerido {}, disponible {}",
                        prod_nombre, combo_nombre, needed, stock_disp
                    ));
                }
            }

            total += price * it.cantidad as f32;
            prepared.push(NewSaleItem {
                sale_id: 0, product_id: None, combo_id: Some(cid),
                cantidad: it.cantidad, precio_unitario: price, costo_unitario: 0.0,
            });

        } else {
            errors.push("Ítem inválido (sin product_id ni combo_id)".into());
        }
    }

    
    if !errors.is_empty() {
        // devolvés *textual* todo lo que falta
        return Err(errors.join("\n"));
    }

    // 2) TRANSACCIÓN: insertar venta e ítems y actualizar stock
    let inserted_id = conn.immediate_transaction::<_, diesel::result::Error, _>(|tx| {
        let fecha = now_ymdhms();
        let new_sale = NewSale {
            user_id: payload.user_id,
            fecha: &fecha,
            total,
            forma_pago: &payload.forma_pago,
            estado: "completada",
        };

        diesel::insert_into(sales::table)
            .values(&new_sale)
            .execute(tx)?;

        let inserted_id: i32 = diesel::sql_query("SELECT last_insert_rowid() AS id")
            .get_result::<LastInsertId>(tx)?
            .id;

        for mut it in prepared {
            it.sale_id = inserted_id;

            diesel::insert_into(sale_items::table).values(&it).execute(tx)?;

            if let Some(pid) = it.product_id {
                diesel::update(P::products.filter(P::id.eq(pid)))
                    .set(P::quantity.eq(P::quantity - it.cantidad))
                    .execute(tx)?;
            } else if let Some(cid) = it.combo_id {
                let parts: Vec<(i32, i32)> = CI::combo_items
                    .select((CI::product_id, CI::cantidad))
                    .filter(CI::combo_id.eq(cid))
                    .load(tx)?;
                for (prod_id, cant_en_combo) in parts {
                    let to_sub = cant_en_combo * it.cantidad;
                    diesel::update(P::products.filter(P::id.eq(prod_id)))
                        .set(P::quantity.eq(P::quantity - to_sub))
                        .execute(tx)?;
                }
            }
        }

        Ok(inserted_id)
    }).map_err(|e| e.to_string())?;

    Ok(inserted_id)
}



#[tauri::command]
pub fn get_sale(id: i32) -> Result<SaleWithItemsNamed, String> {
    let mut conn = get_conn();

    let sale: Sale = sales::table.find(id).first(&mut conn).map_err(|e| e.to_string())?;

       use crate::schema::sale_items as SI;
    use crate::schema::products as P;
    use crate::schema::combos as C;

    // LEFT JOIN + seleccionar nombres como columnas nullable
    let rows = SI::table
        .left_join(P::table.on(SI::product_id.eq(P::id.nullable())))
        .left_join(C::table.on(SI::combo_id.eq(C::id.nullable())))
        .filter(SI::sale_id.eq(id))
        .select((
            SI::id,
            SI::sale_id,
            SI::product_id,
            SI::combo_id,
            SI::cantidad,
            SI::precio_unitario,
            SI::costo_unitario,
            P::nombre.nullable(), // <- importante por LEFT JOIN
            C::nombre.nullable(), // <- importante por LEFT JOIN
        ))
        .load::<(
            i32,                // id
            i32,                // sale_id
            Option<i32>,        // product_id
            Option<i32>,        // combo_id
            i32,                // cantidad
            f32,                // precio_unitario
            f32,                // costo_unitario
            Option<String>,     // nombre producto
            Option<String>,     // nombre combo
        )>(&mut conn)
        .map_err(|e| e.to_string())?;

    let items = rows
        .into_iter()
        .map(|(id, sale_id, product_id, combo_id, cantidad, precio_unitario, costo_unitario, p_nombre, c_nombre)| {
            let nombre = p_nombre.or(c_nombre).unwrap_or_else(|| "Ítem".to_string());
            SaleItemNamed {
                id, sale_id, product_id, combo_id, cantidad, precio_unitario, costo_unitario, nombre
            }
        })
        .collect::<Vec<_>>();

    Ok(SaleWithItemsNamed { sale, items })
}

/// Anular / cambiar estado de venta
#[tauri::command]
pub fn update_sale_status(req: UpdateSaleStatusRequest) -> Result<(), String> {
    let mut conn = get_conn();
    diesel::update(sales::table.find(req.id))
        .set(sales::estado.eq(req.estado))
        .execute(&mut conn)
        .map_err(|e| e.to_string())
        .map(|_| ())
}

/// Soft delete (marca deleted_at)
#[tauri::command]
pub fn delete_sale_soft(id: i32) -> Result<(), String> {
    let mut conn = get_conn();
    let ts = now_ymdhms();
    diesel::update(sales::table.find(id))
        .set(sales::deleted_at.eq(Some(ts)))
        .execute(&mut conn)
        .map_err(|e| e.to_string())
        .map(|_| ())
}
