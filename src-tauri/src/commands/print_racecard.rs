use tauri::menu::MenuItemKind;

#[tauri::command]
pub fn print_racecard(_: tauri::AppHandle) {}

#[tauri::command]
pub fn set_print_racecard_enabled(app: tauri::AppHandle, enabled: bool) {
    if let Some(menu) = app.menu() {
        if let Some(MenuItemKind::Submenu(file_menu)) = menu.get("file") {
            if let Some(item) = file_menu.get("print-racecard") {
                match item {
                    MenuItemKind::MenuItem(item) => {
                        let _ = item.set_enabled(enabled);
                    }
                    _ => {}
                }
            }
        }
    }
}
