mod models;
mod global_state;
mod menus;

use std::collections::HashMap;
use global_state::global_state;
use tauri::Emitter;

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
        .setup(|app| {
            menus::setup_menus(app)?;

            Ok(())
        })
                .on_menu_event(|app, event| {
            match event.id().as_ref() {
                "new" => {
                    let _ = app.emit("menu-new", ()).unwrap();
                }
                "open" => {
                    let _ = app.emit("menu-open", ()).unwrap();
                }
                "exit" => {
                    let _ = app.emit("menu-exit", ()).unwrap();
                }
                _ => {}
            }
        })

        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_tracks
        ])
        .run(context)
        .expect("error while running tauri application");
}
