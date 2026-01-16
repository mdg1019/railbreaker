use tokio::fs;
use serde_json::Value;
use crate::json::to_camel_case_value;

#[tauri::command]
pub async fn load_racecard_file(path: String) -> Result<Value, String> {
    let contents = fs::read_to_string(&path)
        .await
        .map_err(|e| format!("Failed to read racecard file: {}", e))?;

    let racecard: Value = serde_json::from_str(&contents)
        .map_err(|e| format!("Failed to parse racecard JSON: {}", e))?;

    Ok(to_camel_case_value(racecard))
}
