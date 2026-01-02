mod files;
mod menus;
mod models;
mod single_file_indexes;
mod states;

use models::racecard::{Horse, Race, Racecard};
use single_file_indexes::*;
use states::config_state::ConfigState;
use states::global_state::{global_state, GlobalState};
use std::collections::HashMap;
use tauri::{Emitter, Manager};
use tokio::fs;

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
    let file = std::fs::File::open(&path).map_err(|e| format!("Failed to open zip file: {}", e))?;

    let mut archive =
        zip::ZipArchive::new(file).map_err(|e| format!("Failed to read zip archive: {}", e))?;

    if archive.len() == 0 || archive.len() > 1 {
        return Err("Zip archive is not a valid single file archive".to_string());
    }

    let mut file = archive
        .by_index(0)
        .map_err(|e| format!("Failed to access file in zip: {}", e))?;

    let enclosed_name = file.enclosed_name();
    let filename = match enclosed_name.as_ref() {
        Some(path) => path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or("Invalid filename")?,
        None => return Err("File has no valid enclosed name".to_string()),
    };

    if !filename.to_lowercase().ends_with(".drf") {
        return Err(format!(
            "File must have a .DRF extension, got: {}",
            filename
        ));
    }

    let racecards_dir = {
        let global_state = global_state()
            .lock()
            .map_err(|e| format!("Failed to lock global state: {}", e))?;
        format!("{}/Racecards", global_state.current_directory)
    };

    let outpath = std::path::PathBuf::from(&racecards_dir).join(filename);

    if (*file.name()).ends_with('/') {
        return Err(format!("Zip contains directories, which is not supported"));
    } else {
        use std::io::Read;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .map_err(|e| format!("Failed to read file from zip: {}", e))?;

        fs::write(&outpath, buffer)
            .await
            .map_err(|e| format!("Failed to write file: {}", e))?;
    }

    Ok(outpath.to_string_lossy().to_string())
}

#[tauri::command]
async fn process_racecard_file<'a>(path: String) -> Result<Racecard, String> {
    let contents = fs::read_to_string(&path)
        .await
        .map_err(|e| format!("Failed to racecard file: {}", e))?;

    let lines: Vec<Vec<String>> = contents
        .lines()
        .map(|line| {
            line.split(',')
                .map(|field| field.trim().trim_matches('"').to_string())
                .collect()
        })
        .collect();

    let number_of_columns = lines.first().ok_or("Racecard file is empty")?.len();

    for (i, line) in lines.iter().enumerate() {
        if line.len() != number_of_columns {
            return Err(format!("Inconsistent number of columns at line {}", i + 1));
        }
    }

    let track_code = &lines[0][SF_TRACK];
    let track_name = {
        let gs = global_state().lock().unwrap();
        gs.tracks
            .get(track_code)
            .cloned()
            .unwrap_or_else(|| track_code.clone())
    };

    let mut races = Vec::<Race>::new();

    for line in &lines {
        let race_number = line[SF_RACE_NUMBER].parse::<u32>().ok();

        let race_index = races.iter().position(|r| r.race_number == race_number);

        let race_idx = if let Some(idx) = race_index {
            idx
        } else {
            let race = Race {
                race_number: race_number,
                distance: line[SF_DISTANCE].parse::<i32>().ok(),
                surface: line[SF_SURFACE].clone(),
                race_type: line[SF_RACE_TYPE].clone(),
                age_sex_restrictions: line[SF_AGE_SEX_RESTRICTIONS].clone(),
                todays_race_classification: line[SF_TODAYS_RACE_CLASSIFICATION].clone(),
                purse: line[SF_PURSE].parse::<u32>().ok(),
                claiming_price: line[SF_CLAIMING_PRICE].parse::<u32>().ok(),
                track_record: line[SF_TRACK_RECORD].parse::<f64>().ok(),
                race_conditions: line[SF_RACE_CONDITIONS].clone(),
                todays_lasix_list: line[SF_TODAYS_LASIX_LIST].clone(),
                todays_bute_list: line[SF_TODAYS_BUTE_LIST].clone(),
                todays_coupled_list: line[SF_TODAYS_COUPLED_LIST].clone(),
                todays_mutuel_list: line[SF_TODAYS_MUTUEL_LIST].clone(),
                horses: Vec::new(),
            };
            races.push(race);
            races.len() - 1
        };

        let horse = models::racecard::Horse {
            post_position: line[SF_POST_POSITION].parse::<u32>().ok(),
            entry: line[SF_ENTRY].clone(),
            claiming_price_of_horse: line[SF_CLAIMING_PRICE_OF_HORSE].parse::<u32>().ok(),
        };

        races[race_idx].horses.push(horse);
    }

    let racecard = Racecard {
        track: track_name,
        date: lines[0][SF_RACE_DATE].clone(),
        races: races,
    };

    // println!("{:?}", racecard);
    Ok(racecard)
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
            process_racecard_file,
        ])
        .run(context)
        .expect("error while running tauri application");
}

fn init_setup() -> Result<(), String> {
    let current_directory =
        std::env::current_dir().map_err(|e| format!("Failed to get current directory: {}", e))?;

    let downloads_directory =
        dirs::download_dir().ok_or_else(|| "Could not find downloads directory".to_string())?;

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
        _ => {}
    }
}
