use crate::schema::combos;
use crate::config::db::get_conn;
use diesel::{RunQueryDsl, prelude::*};
use super::models::{Combo, NewCombo, UpdateCombo};

#[tauri::command]
pub fn list_combos() -> Result<Vec<Combo>, String> {
    use crate::schema::combos::dsl::*;
    let mut conn = get_conn();
    
    let result = combos.load::<Combo>(&mut conn).map_err(|e| e.to_string())?;
    Ok(result)
}

#[tauri::command]
pub fn list_active_combos() -> Result<Vec<Combo>, String> {
    use crate::schema::combos::dsl::*;
    let mut conn = get_conn();
    let result = combos
        .filter(enabled.eq(true))
        .load::<Combo>(&mut conn)
        .map_err(|e| e.to_string())?;
    Ok(result)
}

#[tauri::command]
pub fn create_combo(new: NewCombo) -> Result<(), String> {

    let mut conn = get_conn();
    
    diesel::insert_into(combos::table)
        .values(&new)
        .execute(&mut conn)
        .map_err(|e| e.to_string())
        .map(|_| ())
}

#[tauri::command]
pub fn update_combo(form: UpdateCombo) -> Result<(), String> {

    let mut conn = get_conn();

    diesel::update(combos::table.find(form.id))
        .set((
          combos::nombre.eq(form.nombre),
          combos::descripcion.eq(form.descripcion),
          combos::price.eq(form.price),
          combos::enabled.eq(form.enabled),
        ))
        .execute(&mut conn)
        .map_err(|e| e.to_string())
        .map(|_| ())
}

#[tauri::command]
pub fn delete_combo(id_to_delete: i32) -> Result<(), String> {
    let mut conn = get_conn();

    diesel::delete(combos::table.find(id_to_delete))
        .execute(&mut conn)
        .map_err(|e| e.to_string())
        .map(|_| ())
}
