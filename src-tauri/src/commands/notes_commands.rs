use tokio::fs;
use serde_json::Value;
use std::io::ErrorKind;
use crate::json::to_camel_case_value;

#[tauri::command]
pub async fn load_notes_file(path: String) -> Result<Value, String> {
    match fs::metadata(&path).await {
        Ok(_) => {}
        Err(e) if e.kind() == ErrorKind::NotFound => {
            let empty_notes = Value::Array(Vec::new());
            save_notes_file(path, empty_notes.clone()).await?;
            return Ok(empty_notes);
        }
        Err(e) => return Err(format!("Failed to access notes file: {}", e)),
    }

    let contents = fs::read_to_string(&path)
        .await
        .map_err(|e| format!("Failed to read notes file: {}", e))?; 

    let notes: Value = serde_json::from_str(&contents)
        .map_err(|e| format!("Failed to parse notes JSON: {}", e))?;

    Ok(to_camel_case_value(notes))
}

#[tauri::command]
pub async fn save_notes_file(path: String, notes: Value) -> Result<(), String> {
    let notes_snake_case = serde_json::to_string_pretty(&notes)
        .map_err(|e| format!("Failed to serialize notes to JSON: {}", e))?;

    fs::write(&path, notes_snake_case)
        .await
        .map_err(|e| format!("Failed to write notes file: {}", e))?;

    Ok(())
}
