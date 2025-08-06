use diesel::prelude::*;
use crate::schema::{sales, sale_items, products, payments};
use tauri::command;
use serde::Serialize;



#[derive(Serialize)]
pub struct VentaDetalle {
    pub id: Option<i32>,
    pub fecha: String,
    pub producto: Option<String>, // puede venir de sale_items
    pub cantidad: i32,
    pub precio_unitario: f32,
    pub ingresos: f32,
    pub costo_unitario: f32,
    pub costo_total: f32,
    pub ganancia: f32,
    pub estado: String,
    pub forma_pago: Option<String>,
}

#[derive(Serialize)]
pub struct PaginatedVentas {
    pub data: Vec<VentaDetalle>,
    pub total_pages: i64,
    pub current_page: i64,
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
    // 1. Construir consulta base para conteo
    let mut count_query = sales::table
        .inner_join(sale_items::table.on(sale_items::sale_id.eq(sales::id.assume_not_null())))
        .left_join(payments::table.on(payments::sale_id.eq(sales::id.assume_not_null())))
        .inner_join(products::table.on(products::id.eq(sale_items::product_id.nullable())))
        .into_boxed();

    // aplicar filtros igual que abajo...
    if let Some(f) = &fecha {
    count_query = count_query.filter(sales::fecha.like(format!("%{}%", f)));
    }
    if let Some(e) = &estado {
    count_query = count_query.filter(sales::estado.eq(e));
    }
    if let Some(fp) = &forma_pago {
    count_query = count_query.filter(payments::forma_pago.eq(fp));
    }

    // obtener conteo
    let total_count: i64 = count_query.count().get_result(&mut conn).map_err(|e| e.to_string())?;

    // 2. Construir otra consulta para obtener datos paginados
    let mut data_query = sales::table
        .inner_join(sale_items::table.on(sale_items::sale_id.eq(sales::id.assume_not_null())))
        .left_join(payments::table.on(payments::sale_id.eq(sales::id.assume_not_null())))
        .inner_join(products::table.on(products::id.eq(sale_items::product_id.nullable())))
        .select((
        sales::id,
        sales::fecha,
        sale_items::cantidad,
        sale_items::precio_unitario,
        sale_items::costo_unitario,
        sales::estado,
        payments::forma_pago.nullable(),
        products::nombre.nullable(),
        ))
        .into_boxed();

    // aplicar mismo filtrado
    if let Some(f) = &fecha {
    data_query = data_query.filter(sales::fecha.like(format!("%{}%", f)));
    }
    if let Some(e) = &estado {
    data_query = data_query.filter(sales::estado.eq(e));
    }
    if let Some(fp) = &forma_pago {
    data_query = data_query.filter(payments::forma_pago.eq(fp));
    }

    // paginar y cargar
    let results = data_query
        .limit(PAGE_SIZE)
        .offset((page - 1) * PAGE_SIZE)
        .load::<(Option<i32>, String, i32, f32, f32, String, Option<String>, Option<String>)>(&mut conn)
        .map_err(|e| e.to_string())?;


    let data = results.into_iter().map(|(id, fecha, cantidad, pu, cu, estado_s, fp, producto)| {
        let ingresos = pu * cantidad as f32;
        let costo_total = cu * cantidad as f32;
        let ganancia = ingresos - costo_total;
        VentaDetalle {
            id,
            fecha,
            producto,
            cantidad,
            precio_unitario: pu,
            ingresos,
            costo_unitario: cu,
            costo_total,
            ganancia,
            estado: estado_s,
            forma_pago: fp,
        }
    }).collect();

    Ok(PaginatedVentas {
        data,
        total_pages: (total_count + PAGE_SIZE - 1) / PAGE_SIZE,
        current_page: page,
    })
}
