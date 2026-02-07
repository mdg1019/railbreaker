use crate::ConfigState;
use crate::global_state;
use crate::files;
use crate::constants::HORSE_SORTING_METHOD_DEFAULT;
use tokio::fs;

pub fn get_config_file_path() -> Result<String, String> {
    let global_state = global_state()
        .lock()
        .map_err(|e| format!("Failed to lock global state: {}", e))?;

    if global_state.current_directory.is_empty() {
        return Err("Current directory not initialized".to_string());
    }

    Ok(format!("{}/config.json", global_state.current_directory))
}

#[tauri::command]
pub async fn load_config_file() -> Result<ConfigState, String> {
    let path = get_config_file_path()?;

    if !fs::try_exists(&path).await.unwrap_or(false) {
        let downloads_directory = {
            let global_state = global_state()
                .lock()
                .map_err(|e| format!("Failed to lock global state: {}", e))?;
            global_state.downloads_directory.clone()
        };
        let cs = ConfigState {
            last_directory: downloads_directory,
            window_x: None,
            window_y: None,
            window_width: None,
            window_height: None,
            horse_sorting_method: HORSE_SORTING_METHOD_DEFAULT.to_string(),
        };

        files::write_json_file(&path, &cs).await?;
        return Ok(cs);
    }

    let config_state = files::read_json_file::<ConfigState>(path).await.unwrap();

    Ok(config_state)
}

#[tauri::command]
pub async fn save_config_file(config_state: ConfigState) -> Result<(), String> {
    let path = get_config_file_path()?;
    files::write_json_file(path, &config_state).await
}