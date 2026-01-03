use tokio::fs;
use crate::models::racecard::Racecard;

#[tauri::command]
pub async fn load_racecard_file(path: String) -> Result<Racecard, String> {
    let contents = fs::read_to_string(&path)
        .await
        .map_err(|e| format!("Failed to read racecard file: {}", e))?;

    let racecard: Racecard = serde_json::from_str(&contents)
        .map_err(|e| format!("Failed to parse racecard JSON: {}", e))?;

    Ok(racecard)
}
