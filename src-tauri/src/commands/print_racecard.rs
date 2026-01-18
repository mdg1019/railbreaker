use tauri::{menu::MenuItemKind, Manager};

#[tauri::command]
pub fn print_racecard(_: tauri::AppHandle) {}

#[tauri::command]
pub fn set_print_racecard_enabled(app: tauri::AppHandle, enabled: bool) {
    if let Some(window) = app.get_webview_window("main") {
        if let Some(menu) = window.menu() {
            if let Some(MenuItemKind::Submenu(file_menu)) = menu.get("file") {
                if let Some(item) = file_menu.get("print-racecard") {
                    if let MenuItemKind::MenuItem(item) = item {
                        let _ = item.set_enabled(enabled);
                    }
                }
            }
        }
    }
}

#[tauri::command]
pub fn close_print_window(app: tauri::AppHandle) -> bool {
    if let Some(window) = app.get_webview_window("print") {
        let _ = window.hide();
        let _ = window.close();
        let _ = window.destroy();
        return true;
    }

    false
}

#[tauri::command]
pub fn hide_print_window_menu(app: tauri::AppHandle) -> bool {
    if cfg!(target_os = "macos") {
        return false;
    }

    if let Some(window) = app.get_webview_window("print") {
        let _ = window.remove_menu();
        let _ = window.hide_menu();
        return true;
    }

    false
}
