pub mod db;
pub mod racecards;
pub mod analysis;

use crate::global_state;

pub fn get_database_file_path() -> Result<String, String> {
    let global_state = global_state()
        .lock()
        .map_err(|e| format!("Failed to lock global state: {}", e))?;

    if global_state.current_directory.is_empty() {
        return Err("Current directory not initialized".to_string());
    }

    Ok(format!("{}/Racecards/railbreaker.db", global_state.current_directory))
}