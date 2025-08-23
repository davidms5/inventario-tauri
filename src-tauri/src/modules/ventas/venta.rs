
// src/modules/ventas/sales.rs
use diesel::{prelude::*, RunQueryDsl, ExpressionMethods, QueryDsl};
use chrono::Utc;

use crate::config::db::get_conn;
use crate::schema::{sales, sale_items, products, combos, combo_items};
use super::models::*;

#[derive(Queryable)]
struct ProductRow { pub id: i32, pub price: f32, pub quantity: i32 }

#[derive(Queryable)]
struct ComboRow { pub id: i32, pub price: f32, pub enabled: bool }

#[derive(Queryable)]
struct ComboItemRow { pub combo_id: i32, pub product_id: i32, pub cantidad: i32 }

// Utilidad: fecha ISO corta
fn now_ymdhms() -> String {
    Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

/// CREA una venta con items (productos y/o combos)
/// - Descuenta stock (productos directos y los productos que componen combos)
#[tauri::command]
pub fn create_sale(payload: NewSaleRequest) -> Result<i32, String> {
    let mut conn = get_conn();
    conn.immediate_transaction::<_, diesel::result::Error, _>(|tx| {
        use products::dsl as P;
        use combos::dsl as C;
        use combo_items::dsl as CI;

        // 1) Precalcular total sumando lineas a partir de precios de DB
        let mut total: f32 = 0.0;

        // Para ir almacenando los NewSaleItem ya con valores correctos
        let mut prepared_items: Vec<NewSaleItem> = Vec::new();

        for it in &payload.items {
            if let Some(pid) = it.product_id {
                // Producto simple
                let pr: ProductRow = P::products
                    .filter(P::id.eq(pid))
                    .select((P::id, P::price, P::quantity))
                    .first(tx)?;

                if pr.quantity < it.cantidad {
                    return Err(diesel::result::Error::RollbackTransaction);
                }

                let precio_unitario = pr.price;
                // Si no manejas costo real, puede ser 0.0 o igual a price (ganancia 0)
                let costo_unitario = 0.0;

                total += precio_unitario * (it.cantidad as f32);

                prepared_items.push(NewSaleItem {
                    sale_id: 0, // lo llenamos después
                    product_id: Some(pid),
                    combo_id: None,
                    cantidad: it.cantidad,
                    precio_unitario,
                    costo_unitario,
                });

            } else if let Some(cid) = it.combo_id {
                // Combo
                let cr: ComboRow = C::combos
                    .filter(C::id.eq(cid))
                    .select((C::id, C::price, C::enabled))
                    .first(tx)?;

                if !cr.enabled {
                    return Err(diesel::result::Error::RollbackTransaction);
                }

                // Chequear stock de cada producto del combo
                let parts: Vec<ComboItemRow> = CI::combo_items
                    .filter(CI::combo_id.eq(cid))
                    .select((CI::combo_id, CI::product_id, CI::cantidad))
                    .load(tx)?;

                // verificar stock
                for part in &parts {
                    let pr: ProductRow = P::products
                        .filter(P::id.eq(part.product_id))
                        .select((P::id, P::price, P::quantity))
                        .first(tx)?;
                    let needed = part.cantidad * it.cantidad;
                    if pr.quantity < needed {
                        return Err(diesel::result::Error::RollbackTransaction);
                    }
                }

                // Agregar renglón de combo (una sola línea)
                let precio_unitario = cr.price;
                let costo_unitario = 0.0; // si no hay costo, 0

                total += precio_unitario * (it.cantidad as f32);

                prepared_items.push(NewSaleItem {
                    sale_id: 0,
                    product_id: None,
                    combo_id: Some(cid),
                    cantidad: it.cantidad,
                    precio_unitario,
                    costo_unitario,
                });

            } else {
                return Err(diesel::result::Error::RollbackTransaction);
            }
        }

        // 2) Insertar SALE
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

        // SQLite devuelve i64; casteamos a i32
        let inserted_id: i32 = diesel::sql_query("SELECT last_insert_rowid() AS id")
            .get_result::<LastInsertId>(tx)?
            .id;
        
        // 3) Insertar ITEMS y descontar stock
        for mut it in prepared_items {
            it.sale_id = inserted_id;

            diesel::insert_into(sale_items::table)
                .values(&it)
                .execute(tx)?;

            // Descontar stock
            if let Some(pid) = it.product_id {
                diesel::update(P::products.filter(P::id.eq(pid)))
                    .set(P::quantity.eq(P::quantity - it.cantidad))
                    .execute(tx)?;
            } else if let Some(cid) = it.combo_id {
                // restar por cada producto del combo
                let parts: Vec<ComboItemRow> = CI::combo_items
                    .filter(CI::combo_id.eq(cid))
                    .select((CI::combo_id, CI::product_id, CI::cantidad))
                    .load(tx)?;
                for part in parts {
                    let to_sub = part.cantidad * it.cantidad;
                    diesel::update(P::products.filter(P::id.eq(part.product_id)))
                        .set(P::quantity.eq(P::quantity - to_sub))
                        .execute(tx)?;
                }
            }
        }

        Ok(inserted_id)
    }).map_err(|e| e.to_string())
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
