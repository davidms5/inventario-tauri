use diesel::sql_types::Text;
// src/modules/cierres/cierres.rs
use diesel::{prelude::*, RunQueryDsl, QueryDsl, ExpressionMethods};
use chrono::{NaiveDate};
use crate::config::db::get_conn;
use crate::schema::{daily_closures, daily_closure_totals};
use super::models::*;
use chrono::Local;

fn today_ymd() -> String {
    use chrono::Local;
    Local::now().format("%Y-%m-%d").to_string()
}

#[tauri::command]
pub fn preview_daily_closure(date_ymd: Option<String>)
    -> Result<(i64, f64, Vec<(String, f64)>), String>
{
    let mut conn = get_conn();
    let fecha = date_ymd.unwrap_or_else(today_ymd);

    // 1) Conteo y total del día
    let agg: AggCountSum = diesel::sql_query(r#"
        SELECT
          COUNT(*)                      AS cnt,
          COALESCE(SUM(total), 0)       AS sum_total
        FROM sales
        WHERE estado = 'completada'
          AND deleted_at IS NULL
          AND strftime('%Y-%m-%d', fecha) = ?1
    "#)
    .bind::<Text, _>(fecha.clone())
    .get_result(&mut conn)
    .map_err(|e| e.to_string())?;

    // 2) Totales por forma de pago
    let rows: Vec<AggRow> = diesel::sql_query(r#"
        SELECT
          forma_pago,
          COALESCE(SUM(total), 0) AS total
        FROM sales
        WHERE estado = 'completada'
          AND deleted_at IS NULL
          AND strftime('%Y-%m-%d', fecha) = ?1
        GROUP BY forma_pago
    "#)
    .bind::<Text, _>(fecha)
    .load(&mut conn)
    .map_err(|e| e.to_string())?;

    let pagos = rows.into_iter().map(|r| (r.forma_pago, r.total)).collect();
    Ok((agg.cnt, agg.sum_total, pagos))
}

#[tauri::command]
pub fn create_daily_closure(date_ymd: Option<String>, user_id: i32) -> Result<DailyClosureFull, String> {
    let mut conn = get_conn();
    let fecha = date_ymd.unwrap_or_else(today_ymd);

    // 1) Validar formato (YYYY-MM-DD)
    if NaiveDate::parse_from_str(&fecha, "%Y-%m-%d").is_err() {
        return Err("Fecha inválida (use YYYY-MM-DD).".into());
    }

    // 2) Chequear existencia FUERA de la tx (para poder devolver String legible)
    use daily_closures::dsl as DC;
    let exists: Option<i32> = DC::daily_closures
        .filter(DC::fecha.eq(&fecha))
        .select(DC::id)
        .first::<i32>(&mut conn)
        .optional()
        .map_err(|e| e.to_string())?;
    if exists.is_some() {
        return Err("Ya existe un cierre para esa fecha".into());
    }

    // 3) Hacer todo lo demás dentro de una transacción
    conn.immediate_transaction::<_, diesel::result::Error, _>(|tx| {
        // Agregados del día
        let CountTotalRow { cnt, sum_total } = diesel::sql_query(r#"
            SELECT COUNT(*) AS cnt, COALESCE(SUM(total), 0) AS sum_total
            FROM sales
            WHERE estado = 'completada'
              AND deleted_at IS NULL
              AND strftime('%Y-%m-%d', fecha) = ?1
        "#)
        .bind::<Text, _>(fecha.clone())
        .get_result::<CountTotalRow>(tx)?;

        let pagos_rows: Vec<PagoAggRow> = diesel::sql_query(r#"
            SELECT forma_pago, COALESCE(SUM(total), 0) AS total
            FROM sales
            WHERE estado = 'completada'
              AND deleted_at IS NULL
              AND strftime('%Y-%m-%d', fecha) = ?1
            GROUP BY forma_pago
        "#)
        .bind::<Text, _>(fecha.clone())
        .load::<PagoAggRow>(tx)?;

        // Insertar cierre
        diesel::insert_into(daily_closures::table)
            .values(&NewDailyClosure {
                fecha: &fecha,
                total: sum_total as f32,
                ventas_count: cnt as i32,
                created_by: user_id,
            })
            .execute(tx)?;

        // Recuperar id (SQLite)
        let closure_id: i32 = diesel::sql_query("SELECT last_insert_rowid() AS id")
            .get_result::<RowId>(tx)?
            .id;

        // Insertar totales por forma de pago
        for r in pagos_rows {
            diesel::insert_into(daily_closure_totals::table)
                .values(&NewClosurePaymentTotal {
                    closure_id,
                    forma_pago: &r.forma_pago,
                    monto: r.total as f32,
                })
                .execute(tx)?;
        }

        // Devolver cierre completo
        let cierre: DailyClosure = daily_closures::table.find(closure_id).first(tx)?;
        let pagos: Vec<ClosurePaymentTotal> = daily_closure_totals::table
            .filter(daily_closure_totals::closure_id.eq(closure_id))
            .load(tx)?;
        Ok(DailyClosureFull { cierre, pagos })
    })
    .map_err(|e| e.to_string())
}


/// Listar cierres del mes `YYYY-MM` con filtro opcional por forma de pago
#[tauri::command]
pub fn list_daily_closures(month_ym: String, forma_pago: Option<String>) -> Result<Vec<ClosureListRow>, String> {
    let mut conn = get_conn();

    // Trae cierres del mes
    let cierres: Vec<DailyClosure> = daily_closures::table
        .filter(diesel::dsl::sql::<diesel::sql_types::Bool>("substr(fecha,1,7) = ").bind::<diesel::sql_types::Text,_>(month_ym.clone()))
        .order(daily_closures::fecha.asc())
        .load(&mut conn)
        .map_err(|e| e.to_string())?;

    // Para cada cierre, sumar por forma de pago (o traer solo la forma filtrada)
    let mut out = Vec::with_capacity(cierres.len());
    for c in cierres {
        let mut efectivo=0.0; let mut tarjeta=0.0; let mut transferencia=0.0; let mut mp=0.0; let mut otros=0.0;

        let mut q = daily_closure_totals::table
            .filter(daily_closure_totals::closure_id.eq(c.id))
            .into_boxed();

        if let Some(ref fp) = forma_pago {
            q = q.filter(daily_closure_totals::forma_pago.eq(fp));
        }

        let rows: Vec<ClosurePaymentTotal> = q.load(&mut conn).map_err(|e| e.to_string())?;
        for r in rows {
            let v = r.monto as f32;
            match r.forma_pago.as_str() {
                "efectivo" => efectivo = v,
                "tarjeta" => tarjeta = v,
                "transferencia" => transferencia = v,
                "mercado_pago" => mp = v,
                _ => otros += v,
            }
        }

        out.push(ClosureListRow {
            fecha: c.fecha,
            total: c.total,
            ventas_count: c.ventas_count,
            creado_por: c.created_by,
            efectivo, tarjeta, transferencia, mercado_pago: mp, otros
        });
    }

    Ok(out)
}

#[tauri::command]
pub fn is_date_closed(date_ymd: Option<String>) -> Result<bool, String> {
    let mut conn = get_conn();
    let fecha = date_ymd.unwrap_or_else(|| Local::now().format("%Y-%m-%d").to_string());

    use daily_closures::dsl as DC;
    let exists: Option<i32> = DC::daily_closures
        .filter(DC::fecha.eq(&fecha))
        .select(DC::id)
        .first::<i32>(&mut conn)
        .optional()
        .map_err(|e| e.to_string())?;

    Ok(exists.is_some())
}
