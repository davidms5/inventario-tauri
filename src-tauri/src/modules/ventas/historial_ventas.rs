use diesel::{dsl::count_star, prelude::*, dsl::sum};
use crate::schema::{sales, sale_items, products, combos, payments}; // payments
use tauri::command;
use super::models::*;
//use diesel::dsl::count_star;

fn today_ymd() -> String {
    use chrono::Local;
    Local::now().format("%Y-%m-%d").to_string()
}

#[command]
pub fn list_sales_paginated(
    fecha: Option<String>,
    estado: Option<String>,
    forma_pago: Option<String>,
    page: i64,
) -> Result<PaginatedVentas, String> {
    
    let mut conn = crate::config::db::get_conn();
    const PAGE_SIZE: i64 = 10;

    // ---------- COUNT ----------
    let mut count_query = sales::table
        .inner_join(
            sale_items::table.on(sale_items::sale_id.eq(sales::id))
        )
        .into_boxed();

    if let Some(f) = &fecha {
        count_query = count_query.filter(sales::fecha.like(format!("%{}%", f)));
    }
    if let Some(e) = &estado {
        count_query = count_query.filter(sales::estado.eq(e));
    }
    if let Some(fp) = &forma_pago {
        // usa la columna de SALES, no PAYMENTS
        count_query = count_query.filter(sales::forma_pago.eq(fp));
    }

    let total_count: i64 = count_query
        .count()
        .get_result(&mut conn)
        .map_err(|e| e.to_string())?;

    // ---------- DATA ----------
    let mut data_query = sales::table
        .inner_join(
            sale_items::table.on(sale_items::sale_id.eq(sales::id))
        )
        .left_outer_join(
            products::table.on(sale_items::product_id.eq(products::id.nullable()))
        )
        .left_outer_join(
            combos::table.on(sale_items::combo_id.eq(combos::id.nullable()))
        )
        .select((
            sales::id,
            sales::fecha,
            sale_items::cantidad,
            sale_items::precio_unitario,
            sale_items::costo_unitario,
            sales::estado,
            sales::forma_pago,                  // <- de sales
            products::nombre.nullable(),        // puede ser NULL si es combo
            combos::nombre.nullable(),          // puede ser NULL si es producto
        ))
        .into_boxed();

    if let Some(f) = &fecha {
        data_query = data_query.filter(sales::fecha.like(format!("%{}%", f)));
    }
    if let Some(e) = &estado {
        data_query = data_query.filter(sales::estado.eq(e));
    }
    if let Some(fp) = &forma_pago {
        data_query = data_query.filter(sales::forma_pago.eq(fp)); // idem
    }

    let rows = data_query
        .limit(PAGE_SIZE)
        .offset((page - 1) * PAGE_SIZE)
        .load::<(i32, String, i32, f32, f32, String, String, Option<String>, Option<String>)>(&mut conn)
        .map_err(|e| e.to_string())?;

    let data = rows
        .into_iter()
        .map(|(id, fecha, cantidad, pu, cu, estado_s, fp, nombre_prod, nombre_combo)| {
            let nombre = nombre_prod.or(nombre_combo).unwrap_or_else(|| "-".to_string());
            let ingresos = pu * cantidad as f32;
            let costo_total = cu * cantidad as f32;
            let ganancia = ingresos - costo_total;
            VentaDetalle {
                id: id,
                fecha,
                producto: Some(nombre),
                cantidad,
                precio_unitario: pu,
                ingresos,
                costo_unitario: cu,
                costo_total,
                ganancia,
                estado: estado_s,
                forma_pago: Some(fp),
            }
        })
        .collect();

    Ok(PaginatedVentas {
        data,
        total_pages: ((total_count + PAGE_SIZE - 1) / PAGE_SIZE).max(1),
        current_page: page,
    })
}

#[command]
pub fn get_today_sales_summary() -> Result<TodaySummary, String> {
    let mut conn = crate::config::db::get_conn();
    let today = today_ymd();
    let prefix = format!("{}%", today);

    let (count_res, sum_res): (i64, Option<f32>) = sales::table
    .filter(sales::fecha.like(prefix.clone()))
    .filter(sales::deleted_at.is_null())
    .filter(sales::estado.ne("cancelada")) //TODO: fijarse si es correcto asi o mejor 'anulada'
    .select((count_star(), sum(sales::total)))
    .first(&mut conn)
    .map_err(|e| e.to_string())?;


    let por_fp = payments::table
    .inner_join(sales::table.on(payments::sale_id.eq(sales::id)))
    .filter(sales::fecha.like(prefix))
    .filter(sales::deleted_at.is_null())
    .filter(sales::estado.ne("anulada"))
    .group_by(payments::forma_pago)
    .select((payments::forma_pago, sum(payments::monto)))
    .load::<(String, Option<f32>)>(&mut conn)
    .map_err(|e| e.to_string())?
    .into_iter()
    .map(|(forma_pago, monto)| PaymentTotal { forma_pago, monto: monto.unwrap_or(0.0) })
        .collect::<Vec<_>>();

        
    Ok(TodaySummary { ventas_count: count_res, total_dia: sum_res.unwrap_or(0.0), por_forma_pago: por_fp })
}