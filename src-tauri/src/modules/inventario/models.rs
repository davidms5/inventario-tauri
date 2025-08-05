use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use crate::schema::products;

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = products)]
pub struct Product {
    pub id: Option<i32>,
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
