use sqlx::{Row, SqlitePool};
use tauri::State;
use railbreaker_lib::models::racecard::Racecard;
use railbreaker_lib::sqlite::racecards::read_racecard;
use serde_json::Value;

#[tauri::command]
pub async fn add_racecard(
    pool: State<'_, SqlitePool>,
    racecard: Racecard,
) -> Result<Racecard, String> {
    railbreaker_lib::sqlite::racecards::add_racecard(&pool, racecard)
        .await
        .map_err(|e| format!("Failed to add racecard: {}", e))
}

#[tauri::command]
pub async fn racecard_exists_by_zip_name(
    pool: State<'_, SqlitePool>,
    zip_file_name: String,
) -> Result<bool, String> {
    let exists = sqlx::query_scalar::<_, i64>(
        "SELECT 1 FROM racecards WHERE zip_file_name = ? LIMIT 1;",
    )
    .bind(zip_file_name)
    .fetch_optional(&*pool)
    .await
    .map_err(|e| format!("Failed to check racecard: {}", e))?
    .is_some();

    Ok(exists)
}

#[tauri::command]
pub async fn get_racecard_by_id(
    pool: State<'_, SqlitePool>,
    racecard_id: i64,
) -> Result<Value, String> {
    let racecard = read_racecard_by_id(&pool, racecard_id)
        .await
        .map_err(|e| format!("Failed to load racecard: {}", e))?;
    let value = serde_json::to_value(&racecard)
        .map_err(|e| format!("Failed to serialize racecard: {}", e))?;
    Ok(value)
}

#[tauri::command]
pub async fn get_all_racecards(
    pool: State<'_, SqlitePool>,
) -> Result<Value, String> {
    let rows = sqlx::query("SELECT * FROM racecards ORDER BY track ASC, date DESC;")
        .fetch_all(&*pool)
        .await
        .map_err(|e| format!("Failed to load racecards: {}", e))?;

    let racecards: Vec<Racecard> = rows
        .into_iter()
        .map(|row| Racecard {
            id: row.get("id"),
            zip_file_name: row.get("zip_file_name"),
            track: row.get("track"),
            date: row.get("date"),
            long_date: row.get("long_date"),
            races: Vec::new(),
        })
        .collect();

    let value = serde_json::to_value(&racecards)
        .map_err(|e| format!("Failed to serialize racecards: {}", e))?;
    Ok(value)
}

#[tauri::command]
pub async fn update_note(
    pool: State<'_, SqlitePool>,
    horse_id: i64,
    note: String,
) -> Result<(), String> {
    sqlx::query("UPDATE horses SET note = ? WHERE id = ?;")
        .bind(note)
        .bind(horse_id)
        .execute(&*pool)
        .await
        .map_err(|e| format!("Failed to update note: {}", e))?;

    Ok(())
}

pub async fn read_racecard_by_id(pool: &SqlitePool, racecard_id: i64) -> Result<Racecard, sqlx::Error> {
    let racecard_row = sqlx::query("SELECT * FROM racecards WHERE id = ?;")
        .bind(racecard_id)
        .fetch_one(pool)
        .await?;

    read_racecard(pool, racecard_row).await
}
