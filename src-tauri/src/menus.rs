use tauri::{menu::{Menu, MenuItem, Submenu, PredefinedMenuItem}, App, Manager};

pub fn setup_menus(app: &App) -> tauri::Result<()> {
    let open = MenuItem::with_id(app, "open", "Open Racecard…", true, Some("CmdOrCtrl+O"))?;
    let open_zip = MenuItem::with_id(app, "open-zip", "Open Zip…", true, Some("CmdOrCtrl+Shift+O"))?;
    let separator0 = PredefinedMenuItem::separator(app)?;
    let print_racecard = MenuItem::with_id(app, "print-racecard", "Print Racecard…", true, Some("CmdOrCtrl+P"))?;
    let _ = print_racecard.set_enabled(false);
    let separator1 = PredefinedMenuItem::separator(app)?;
    let quit = MenuItem::with_id(app, "exit", "Exit", true, Some("CmdOrCtrl+Q"))?;

    let file_menu = Submenu::with_id_and_items(
        app,
        "file",
        "File",
        true,
        &[&open, &open_zip, &separator0, &print_racecard, &separator1, &quit],
    )?;

    let menu = Menu::with_items(app, &[&file_menu])?;

    if let Some(window) = app.get_webview_window("main") {
        window.set_menu(menu)?;
    }

    Ok(())
}
