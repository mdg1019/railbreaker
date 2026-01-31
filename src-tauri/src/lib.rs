mod commands;
mod files;
mod json;
mod menus;
mod models;
mod constants;
mod states;
mod sqlite;
mod analysis;

use tauri::{Emitter, Manager};
use commands::global_state_commands::load_global_state;
use commands::config_file_commands::{load_config_file, save_config_file, get_config_file_path};
use commands::process_zip_file_commands::process_zip_file;
use commands::print_racecard::{close_print_window, hide_print_window_menu, print_racecard, set_print_racecard_enabled};
use commands::process_racecard_file_commands::process_racecard_file;
use commands::analysis_commands::rank_race;
use commands::exit_app_command::exit_app;
use sqlite::racecards::{
    add_racecard, get_all_racecards, get_racecard_by_id, racecard_exists_by_zip_name, update_note,
};
use states::config_state::ConfigState;
use states::global_state::global_state;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let context = tauri::generate_context!();

    tauri::Builder::default()
        .setup(move |app| {
            init_setup(app)?;

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
            load_config_file,
            save_config_file,
            print_racecard,
            load_global_state,
            process_zip_file,
            process_racecard_file,
            rank_race,
            set_print_racecard_enabled,
            close_print_window,
            hide_print_window_menu,
            exit_app,
            add_racecard,
            get_all_racecards,
            get_racecard_by_id,
            racecard_exists_by_zip_name,
            update_note,
        ])
        .run(context)
        .expect("error while running tauri application");
}

fn init_setup(app: &tauri::App) -> Result<(), String> {
    let current_directory =
        std::env::current_dir().map_err(|e| format!("Failed to get current directory: {}", e))?;

    let downloads_directory =
        dirs::download_dir().ok_or_else(|| "Could not find downloads directory".to_string())?;

    let global_state = global_state();

    {
        let mut gs = global_state.lock().unwrap();
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

    let database_url = sqlite::get_database_file_path()?;
    
    let pool = tauri::async_runtime::block_on(async {
        let pool = sqlite::db::make_pool(&database_url).await?;
        sqlite::db::create_tables(&pool).await?;
        Ok::<_, anyhow::Error>(pool)
    })
    .map_err(|e| format!("Failed to initialize database: {}", e))?;
    app.manage(pool);

    Ok(())
}

async fn load_or_init_config(app_handle: tauri::AppHandle) {
    let path = get_config_file_path().unwrap();

    if let Ok(cfg) = files::read_json_file::<ConfigState>(path.clone()).await {
        if let Some(window) = app_handle.get_webview_window("main") {
            if let (Some(w), Some(h)) = (cfg.window_width, cfg.window_height) {
                let _ = window.set_size(tauri::Size::Logical(tauri::LogicalSize {
                    width: w,
                    height: h,
                }));
            }
            if let (Some(x), Some(y)) = (cfg.window_x, cfg.window_y) {
                let _ =
                    window.set_position(tauri::Position::Logical(tauri::LogicalPosition { x, y }));
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
                let path = match get_config_file_path() {
                    Ok(p) => p,
                    Err(_) => return,
                };
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
                let path = match get_config_file_path() {
                    Ok(p) => p,
                    Err(_) => return,
                };
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
        "print-racecard" => {
            print_racecard(app.clone());
            let _ = app.emit("menu-print", ()).unwrap();
        }
        "about" => {
            let _ = app.emit("menu-about", ()).unwrap();
        }
        _ => {}
    }
}
