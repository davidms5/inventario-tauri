use crate::modules::inventario::models::ComboItemView;
use crate::modules::inventario::models::ComboWithItemsView;
use crate::modules::ventas::models::LastInsertId;
use crate::schema::combo_items;
use crate::schema::combos;
use crate::schema::products;
use crate::config::db::get_conn;
use diesel::{RunQueryDsl, prelude::*};
use super::models::{Combo, NewCombo, UpdateCombo, NewComboWithItems, UpdateComboWithItems, NewComboItem};

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

#[tauri::command]
pub fn create_combo_with_items(payload: NewComboWithItems) -> Result<i32, String> {
    let mut conn = get_conn();
    conn.immediate_transaction::<_, diesel::result::Error, _>(|tx| {
        // Validaciones básicas
        if payload.combo.nombre.trim().is_empty() { return Err(diesel::result::Error::RollbackTransaction); }
        if payload.items.is_empty() { return Err(diesel::result::Error::RollbackTransaction); }
        if payload.items.iter().any(|i| i.cantidad <= 0) { return Err(diesel::result::Error::RollbackTransaction); }

        // Verificar que los productos existen y (opcional) están enabled
        let ids: Vec<i32> = payload.items.iter().map(|i| i.product_id).collect();
        let count_ok: i64 = products::table
            .filter(products::id.eq_any(&ids))
            .count()
            .get_result(tx)?;
        if count_ok != ids.len() as i64 {
            return Err(diesel::result::Error::RollbackTransaction);
        }

        // Insert combo
        diesel::insert_into(combos::table)
            .values(&payload.combo)
            .execute(tx)?;

        // Obtener id insertado (SQLite)
        let combo_id = diesel::sql_query("SELECT last_insert_rowid() AS id")
            .get_result::<LastInsertId>(tx)?.id;

        // Insert items
        let to_insert: Vec<NewComboItem> = payload.items.iter().map(|i| NewComboItem {
            combo_id,
            product_id: i.product_id,
            cantidad: i.cantidad,
        }).collect();

        diesel::insert_into(combo_items::table)
            .values(&to_insert)
            .execute(tx)?;

        Ok(combo_id)
    }).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_combo_with_items(payload: UpdateComboWithItems) -> Result<(), String> {
    let mut conn = get_conn();
    conn.immediate_transaction::<_, diesel::result::Error, _>(|tx| {
        if payload.items.iter().any(|i| i.cantidad <= 0) {
            return Err(diesel::result::Error::RollbackTransaction);
        }

        // Actualizar cabecera
        diesel::update(combos::table.find(payload.id))
            .set((
                combos::nombre.eq(&payload.nombre),
                combos::descripcion.eq(&payload.descripcion),
                combos::price.eq(payload.price),
                combos::enabled.eq(payload.enabled),
            ))
            .execute(tx)?;

        // Reemplazar items: delete + insert
        diesel::delete(combo_items::table.filter(combo_items::combo_id.eq(payload.id)))
            .execute(tx)?;

        let to_insert: Vec<NewComboItem> = payload.items.iter().map(|i| NewComboItem {
            combo_id: payload.id,
            product_id: i.product_id,
            cantidad: i.cantidad,
        }).collect();

        diesel::insert_into(combo_items::table)
            .values(&to_insert)
            .execute(tx)?;

        Ok(())
    }).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_combo_with_items(id_query: i32) -> Result<ComboWithItemsView, String> {
    use crate::schema::combos::dsl as C;
    use crate::schema::combo_items::dsl as CI;
    use crate::schema::products::dsl as P;

    let mut conn = get_conn();

    let (id, nombre, descripcion, price, enabled): (i32, String, Option<String>, f32, bool) =
        C::combos
            .filter(C::id.eq(id_query))
            .select((C::id, C::nombre, C::descripcion, C::price, C::enabled))
            .first(&mut conn)
            .map_err(|e| e.to_string())?;

    let rows: Vec<(i32, i32, String)> = CI::combo_items
        .inner_join(P::products.on(P::id.eq(CI::product_id)))
        .filter(CI::combo_id.eq(id))
        .select((CI::product_id, CI::cantidad, P::nombre))
        .load(&mut conn)
        .map_err(|e| e.to_string())?;

    let items = rows.into_iter().map(|(pid, cant, pname)| ComboItemView {
        product_id: pid, cantidad: cant, product_name: pname
    }).collect();

    Ok(ComboWithItemsView { id, nombre, descripcion, price, enabled, items })
}
