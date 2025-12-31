use tauri::{menu::{Menu, MenuItem, Submenu, PredefinedMenuItem}, App};

pub fn setup_menus(app: &App) -> tauri::Result<()> {
    let open = MenuItem::with_id(app, "open", "Open Racecard…", true, Some("CmdOrCtrl+O"))?;
    let open_zip = MenuItem::with_id(app, "open_zip", "Open Zip…", true, Some("CmdOrCtrl+Shift+O"))?;
    let separator0 = PredefinedMenuItem::separator(app)?;
    let quit = MenuItem::with_id(app, "exit", "Exit", true, Some("CmdOrCtrl+Q"))?;

    let file_menu = Submenu::with_items(
        app,
        "File",
        true,
        &[&open, &open_zip, &separator0, &quit],
    )?;

    let menu = Menu::with_items(app, &[&file_menu])?;

    app.set_menu(menu)?;

    Ok(())
}
