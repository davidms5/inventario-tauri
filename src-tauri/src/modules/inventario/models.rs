use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use crate::schema::products;
use crate::schema::combos;
use crate::schema::combo_items;
//use diesel::sql_types::Integer;
#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = products)]
pub struct Product {
    pub id: i32,
    pub nombre: String,
    pub sku: Option<String>,
    pub descripcion: Option<String>,
    pub price: f32,       // según schema
    pub quantity: i32,
    pub category: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = products)]
pub struct NewProduct<'a> {
    pub nombre: &'a str,
    pub sku: Option<&'a str>,
    pub descripcion: Option<&'a str>,
    pub price: f32,
    pub quantity: i32,
    pub category: Option<&'a str>,
}

#[derive(AsChangeset)]
#[diesel(table_name = products)]
pub struct UpdateProduct<'a> {
    pub nombre: &'a str,
    pub sku: Option<&'a str>,
    pub descripcion: Option<&'a str>,
    pub price: f32,
    pub quantity: i32,
    pub category: Option<&'a str>,
}

#[derive(Queryable, Serialize)]
pub struct Combo { id: i32, nombre: String, descripcion: Option<String>, price: f32, enabled: bool }

#[derive(Insertable, Deserialize)]
#[diesel(table_name=combos)]
pub struct NewCombo { pub nombre: String, pub descripcion: Option<String>, pub price: f32, pub enabled: bool }

#[derive(Deserialize)]
pub struct UpdateCombo { pub id: i32, pub nombre: String, pub descripcion: Option<String>, pub price: f32, pub enabled: bool }

// --- nuevos ---
#[derive(Deserialize)]
pub struct ComboItemInput { pub product_id: i32, pub cantidad: i32 }

#[derive(Insertable)]
#[diesel(table_name = combo_items)]
pub struct NewComboItem { pub combo_id: i32, pub product_id: i32, pub cantidad: i32 }

#[derive(Serialize, Queryable)]
pub struct ComboItem { pub combo_id: i32, pub product_id: i32, pub cantidad: i32 }

#[derive(Deserialize)]
pub struct NewComboWithItems {
    pub combo: NewCombo,
    pub items: Vec<ComboItemInput>,
}

#[derive(Deserialize)]
pub struct UpdateComboWithItems {
    pub id: i32,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub price: f32,
    pub enabled: bool,
    pub items: Vec<ComboItemInput>,
}

// Para last_insert_rowid() en SQLite


#[derive(Serialize)]
pub struct ComboItemView { pub product_id: i32, pub cantidad: i32, pub product_name: String }

#[derive(Serialize)]
pub struct ComboWithItemsView {
    pub id: i32, pub nombre: String, pub descripcion: Option<String>,
    pub price: f32, pub enabled: bool, pub items: Vec<ComboItemView>,
}

#[derive(Serialize)]
pub struct Page<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub total_pages: i64,
    pub current_page: i64,
    pub per_page: i64,
}