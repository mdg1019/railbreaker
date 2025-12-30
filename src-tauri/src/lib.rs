mod models;
mod global_state;

use std::collections::HashMap;
use global_state::global_state;

#[tauri::command]
fn get_tracks() -> HashMap<String, String> {
    global_state().lock().unwrap().tracks.clone()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let context = tauri::generate_context!();
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_default();
    let resource_path = exe_dir.join("resources/tracks.txt");
    let tracks = models::track::load_tracks(&resource_path).unwrap_or_default();
    let global_state = global_state();
    {
        let mut state = global_state.lock().unwrap();
        state.tracks = tracks;
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_tracks])
        .run(context)
        .expect("error while running tauri application");
}
