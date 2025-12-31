mod models;
mod states;
mod menus;
mod files;

use std::collections::HashMap;
use tokio::fs;
use states::global_state::global_state;
use states::config_state::ConfigState;
use tauri::{Emitter, Manager};

fn get_config_file_path() -> Result<String, String> {
    let global_state = global_state()
        .lock()
        .map_err(|e| format!("Failed to lock global state: {}", e))?;
    
    if global_state.current_directory.is_empty() {
        return Err("Current directory not initialized".to_string());
    }
    
    Ok(format!("{}/config.json", global_state.current_directory))
}

#[tauri::command]
async fn load_config_file() -> Result<ConfigState, String> {
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
        };

        files::write_json_file(&path, &cs).await?;
        return Ok(cs);
    }

    files::read_json_file::<ConfigState>(path).await
}

#[tauri::command]
async fn save_config_file(config_state: ConfigState) -> Result<(), String> {
    let path = get_config_file_path()?;
    files::write_json_file(path, &config_state).await
}

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
    let resource_path = exe_dir.join("resources/tracks.csv");
    let tracks = models::track::load_tracks(&resource_path).unwrap_or_default();
 
    tauri::Builder::default()
        .setup(move |app| {
            let global_state = global_state();
            {
                let mut state = global_state.lock().unwrap();
                state.tracks = tracks.clone();
            }

            {
                let current_dir = std::env::current_dir()
                    .map_err(|e| format!("Failed to get current directory: {}", e)).unwrap();

                let downloads_dir = dirs::download_dir()
                    .ok_or_else(|| "Could not find downloads directory".to_string()).unwrap();

                let mut gs = global_state.lock().unwrap();
                gs.current_directory = current_dir.to_string_lossy().to_string();
                gs.downloads_directory = downloads_dir.to_string_lossy().to_string();
            }

            menus::setup_menus(app)?;

            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let path = get_config_file_path().unwrap();

                if let Ok(cfg) = files::read_json_file::<ConfigState>(path.clone()).await {
                    if let Some(window) = app_handle.get_webview_window("main") {
                        if let (Some(w), Some(h)) = (cfg.window_width, cfg.window_height) {
                            let _ = window.set_size(tauri::Size::Logical(tauri::LogicalSize { width: w, height: h }));
                        }
                        if let (Some(x), Some(y)) = (cfg.window_x, cfg.window_y) {
                            let _ = window.set_position(tauri::Position::Logical(tauri::LogicalPosition { x, y }));
                        }
                    }
                } else {
                    let downloads_directory = {
                        let gs = global_state.lock().unwrap();
                        gs.downloads_directory.clone()
                    };
                    let mut cfg = ConfigState::default();
                    cfg.last_directory = downloads_directory;

                    let _ = files::write_json_file(path, &cfg).await;
                }
            });

            Ok(())
        })        
        .on_window_event(|_window, event| {
            use tauri::WindowEvent;
            match event {
                WindowEvent::Moved(position) => {
                    let pos_x = position.x as f64;
                    let pos_y = position.y as f64;

                    tauri::async_runtime::spawn(async move {
                        tokio::time::sleep(std::time::Duration::from_millis(250)).await;
                        let path = match get_config_file_path() { Ok(p) => p, Err(_) => return };
                        if let Ok(mut cfg) = files::read_json_file::<ConfigState>(path.clone()).await {
                           cfg.window_x = Some(pos_x);
                            cfg.window_y = Some(pos_y);
                            let _ = files::write_json_file(path, &cfg).await;
                        }
                    });
                }
                WindowEvent::Resized(size) => {
                    let w = size.width as f64;
                    let h = size.height as f64;

                    tauri::async_runtime::spawn(async move {
                        tokio::time::sleep(std::time::Duration::from_millis(250)).await;
                        let path = match get_config_file_path() { Ok(p) => p, Err(_) => return };
                        if let Ok(mut cfg) = files::read_json_file::<ConfigState>(path.clone()).await {
                            cfg.window_width = Some(w);
                            cfg.window_height = Some(h);
                            let r = files::write_json_file(path, &cfg).await;
                        }
                    });
                }
                _ => {}
            }
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
