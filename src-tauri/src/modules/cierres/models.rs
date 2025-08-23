// src/modules/cierres/models.rs
use diesel::{prelude::*, sql_types::{BigInt, Double, Integer, Text}};
use serde::{Serialize};

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::daily_closures)]
pub struct DailyClosure {
    pub id: i32,
    pub fecha: String,        // 'YYYY-MM-DD'
    pub total: f32,
    pub ventas_count: i32,
    pub created_by: i32,
    pub created_at: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::daily_closures)]
pub struct NewDailyClosure<'a> {
    pub fecha: &'a str,
    pub total: f32,
    pub ventas_count: i32,
    pub created_by: i32,
}

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::daily_closure_totals)]
pub struct ClosurePaymentTotal {
    pub id: i32,
    pub closure_id: i32,
    pub forma_pago: String,
    pub monto: f32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::daily_closure_totals)]
pub struct NewClosurePaymentTotal<'a> {
    pub closure_id: i32,
    pub forma_pago: &'a str,
    pub monto: f32,
}

#[derive(Serialize)]
pub struct DailyClosureFull {
    pub cierre: DailyClosure,
    pub pagos: Vec<ClosurePaymentTotal>,
}

#[derive(QueryableByName)]
pub struct AggRow {
    #[diesel(sql_type = Text)]
    pub forma_pago: String,          // en tu schema es NOT NULL
    #[diesel(sql_type = Double)]
    pub total: f64,                  // usamos COALESCE en SQL, así que no es NULL
}

#[derive(serde::Serialize)]
pub struct ClosureListRow {
    pub fecha: String,
    pub total: f32,
    pub ventas_count: i32,
    pub creado_por: i32,
    pub efectivo: f32,
    pub tarjeta: f32,
    pub transferencia: f32,
    pub mercado_pago: f32,
    pub otros: f32,
}

#[derive(QueryableByName)]
pub struct AggCountSum {
    #[diesel(sql_type = BigInt)]
    pub cnt: i64,
    #[diesel(sql_type = Double)]
    pub sum_total: f64,
}

#[derive(diesel::QueryableByName)]
pub struct CountTotalRow {
    #[diesel(sql_type = BigInt)]
    pub cnt: i64,
    #[diesel(sql_type = Double)]
    pub sum_total: f64,
}

#[derive(diesel::QueryableByName)]
pub struct PagoAggRow {
    #[diesel(sql_type = Text)]
    pub forma_pago: String,
    #[diesel(sql_type = Double)]
    pub total: f64,
}

#[derive(diesel::QueryableByName)]
pub struct RowId {
    #[diesel(sql_type = Integer)]
    pub id: i32,
}