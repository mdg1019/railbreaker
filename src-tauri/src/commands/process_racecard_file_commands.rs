// use crate::files::write_json_file;
use railbreaker_lib::build_racecard::build_racecard;
use crate::sqlite::racecards::add_racecard;
use sqlx::SqlitePool;
use serde_json::Value;
use tauri::AppHandle;
use tauri::Manager;


#[tauri::command]
pub async fn process_racecard_file(app: AppHandle, path: String, zip_file_name: String) -> Result<Value, String> {
    let racecard = build_racecard(path, zip_file_name)
        .await
        .map_err(|e| e.to_string())?; 

    let racecard = add_racecard(app.state::<SqlitePool>(), racecard).await
        .map_err(|e| e.to_string())?;
    //
    // Keep this for debugging purposes
    //
    // let json_path = PathBuf::from(&path).with_extension("json");
    // let json_racecard_value = serde_json::to_value(&racecard)
    //     .map_err(|e| format!("Failed to serialize racecard: {}", e))?;
    // let json_racecard_value = to_camel_case_value(json_racecard_value);
    // write_json_file(json_path, &json_racecard_value).await?;

    let racecard_value = serde_json::to_value(&racecard)
        .map_err(|e| format!("Failed to serialize racecard: {}", e))?;
    let racecard_value = racecard_value;

    Ok(racecard_value)
}