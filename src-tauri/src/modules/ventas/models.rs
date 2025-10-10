// src/modules/ventas/models.rs
use serde::{Deserialize, Serialize};
use diesel::{prelude::*};
use crate::schema::{sales, sale_items};

#[derive(Queryable, Identifiable, Serialize)]
#[diesel(table_name = sales)]
pub struct Sale {
    pub id: i32,
    pub user_id: i32,
    pub fecha: String,
    pub total: f32,
    pub forma_pago: String,
    pub estado: String,
    pub deleted_at: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = sales)]
pub struct NewSale<'a> {
    pub user_id: i32,
    pub fecha: &'a str,
    pub total: f32,
    pub forma_pago: &'a str,
    pub estado: &'a str,
}

#[derive(Queryable, Identifiable, Associations, Serialize)]
#[diesel(belongs_to(Sale, foreign_key = sale_id))]
#[diesel(table_name = sale_items)]
pub struct SaleItem {
    pub id: i32,
    pub sale_id: i32,
    pub product_id: Option<i32>,
    pub combo_id: Option<i32>,
    pub cantidad: i32,
    pub precio_unitario: f32,
    pub costo_unitario: f32,
}

#[derive(Insertable)]
#[diesel(table_name = sale_items)]
pub struct NewSaleItem {
    pub sale_id: i32,
    pub product_id: Option<i32>,
    pub combo_id: Option<i32>,
    pub cantidad: i32,
    pub precio_unitario: f32,
    pub costo_unitario: f32,
}

#[derive(Deserialize)]
pub struct NewSaleItemInput {
    pub product_id: Option<i32>,
    pub combo_id: Option<i32>,
    pub cantidad: i32,
    // Nota: el precio lo IGNORAMOS del front para que no lo modifique el vendedor.
    // Si querés, podés incluirlo y no usarlo.
}

#[derive(Deserialize)]
pub struct NewSaleRequest {
    pub user_id: i32,
    pub forma_pago: String,
    pub items: Vec<NewSaleItemInput>,
}

#[derive(Deserialize)]
pub struct UpdateSaleStatusRequest {
    pub id: i32,
    pub estado: String, // "completada" | "anulada" | ...
}

/// Obtener venta + items
#[derive(Serialize)]
pub struct SaleWithItems {
    pub sale: Sale,
    pub items: Vec<SaleItem>,
}

#[derive(QueryableByName)]
pub struct LastInsertId {
    #[sql_type = "diesel::sql_types::Integer"]
    pub id: i32,
}

// === DTOs que devolverás al front ===
#[derive(serde::Serialize)]
pub struct SaleWithItemsNamed {
    pub sale: Sale,
    pub items: Vec<SaleItemNamed>,
}

#[derive(serde::Serialize)]
pub struct SaleItemNamed {
    pub id: i32,
    pub sale_id: i32,
    pub product_id: Option<i32>,
    pub combo_id: Option<i32>,
    pub cantidad: i32,
    pub precio_unitario: f32,
    pub costo_unitario: f32,
    pub nombre: String,
    //pub kind: &'static str, // "product" o "combo"
}

#[derive(Serialize)]
pub struct VentaDetalle {
    pub id: i32,
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

#[derive(Serialize)]
pub struct TodaySummary {
    pub ventas_count: i64,
    pub total_dia: f32,
    pub por_forma_pago: Vec<PaymentTotal>,
}

#[derive(Queryable, Serialize)]
pub struct PaymentTotal {
    pub forma_pago: String,
    pub monto: f32,
}