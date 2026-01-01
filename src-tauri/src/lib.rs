mod models;
mod states;
mod menus;
mod files;

use std::collections::HashMap;
use tokio::fs;
use states::global_state::{global_state, GlobalState};
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
fn load_global_state() -> GlobalState {
    global_state().lock().unwrap().clone()
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

#[tauri::command]
async fn process_zip_file(path: String) -> Result<String, String> {
    Ok(format!("Processed zip file at path: {}", path))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let context = tauri::generate_context!();
 
    tauri::Builder::default()
        .setup(move |app| {
            init_setup()?;

            menus::setup_menus(app)?;

            if let Some(window) = app.get_webview_window("main") {
                let window_clone = window.clone();
                tauri::async_runtime::spawn(async move {
                    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                    let _ = window_clone.show();
                });
            }

            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                load_or_init_config(app_handle).await;
            });

            Ok(())
        })        
        .on_window_event(|_window, event| {
            on_window_event(event);
        })
        .on_menu_event(|app, event| {
            on_menu_event(app, event);
        })

        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            get_tracks,
            load_config_file,
            save_config_file,
            load_global_state,
            process_zip_file,
        ])
        .run(context)
        .expect("error while running tauri application");
}

fn init_setup() -> Result<(), String> {
    let current_directory = std::env::current_dir()
        .map_err(|e| format!("Failed to get current directory: {}", e))?;
                        
    let downloads_directory = dirs::download_dir()
        .ok_or_else(|| "Could not find downloads directory".to_string())?;

   
    let resource_path = current_directory.join("resources/tracks.csv");
    let tracks = models::track::load_tracks(&resource_path).unwrap_or_default();
 

    let global_state = global_state();

    {
        let mut gs = global_state.lock().unwrap();
        gs.tracks = tracks;
        gs.current_directory = current_directory.to_string_lossy().to_string();
        gs.downloads_directory = downloads_directory.to_string_lossy().to_string();

        let racecard_path = current_directory.join("Racecards");
        if !racecard_path.exists() {
            if let Err(e) = std::fs::create_dir_all(&racecard_path) {
                eprintln!("Failed to create Racecards directory: {}", e);
            }
        }
        
        gs.racecards_directory = racecard_path.to_string_lossy().to_string();
    }
    
    Ok(())
}

async fn load_or_init_config(app_handle: tauri::AppHandle) {
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
            let gs = global_state().lock().unwrap();
            gs.downloads_directory.clone()
        };

        // Wait a little for the window to be ready
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        let window = app_handle.get_webview_window("main").unwrap();
            let outer_size = window.outer_size().unwrap();
            let width = outer_size.width as f64;
            let height = outer_size.height as f64;
            let x = window.outer_position().unwrap().x as f64;
            let y = window.outer_position().unwrap().y as f64;

            let cfg = ConfigState {
                last_directory: downloads_directory,
                window_x: Some(x),
                window_y: Some(y),
                window_width: Some(width),
                window_height: Some(height),
            };  

        let _ = files::write_json_file(path, &cfg).await;
    }
}

fn on_window_event(event: &tauri::WindowEvent) {
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
                    let _ = files::write_json_file(path, &cfg).await;
                }
            });
        }
        _ => {}
    }
}

fn on_menu_event(app: &tauri::AppHandle, event: tauri::menu::MenuEvent) {
    match event.id().as_ref() {
        "open" => {
            let _ = app.emit("menu-open", ()).unwrap();
        }
        "open-zip" => {
            let _ = app.emit("menu-open-zip", ()).unwrap();
        }
        "exit" => {
            let _ = app.emit("menu-exit", ()).unwrap();
        }
        _ => {}
    }
}
