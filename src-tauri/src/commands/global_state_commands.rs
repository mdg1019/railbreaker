use crate::json::to_camel_case_value;
use crate::states::global_state::global_state;
use serde_json::Value;

#[tauri::command]
pub fn load_global_state() -> Result<Value, String> {
    let gs = global_state().lock().unwrap().clone();
    let value = serde_json::to_value(&gs)
        .map_err(|e| format!("Failed to serialize global state: {}", e))?;
    Ok(to_camel_case_value(value))
}
