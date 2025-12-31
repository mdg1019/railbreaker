use tauri::{menu::{Menu, MenuItem, Submenu, PredefinedMenuItem}, App};

pub fn setup_menus(app: &App) -> tauri::Result<()> {
    let new_file = MenuItem::with_id(app, "new", "New", true, Some("CmdOrCtrl+N"))?;
    let open = MenuItem::with_id(app, "open", "Open…", true, Some("CmdOrCtrl+O"))?;
    let separator0 = PredefinedMenuItem::separator(app)?;
    let quit = MenuItem::with_id(app, "exit", "Exit", true, Some("CmdOrCtrl+Q"))?;

    let file_menu = Submenu::with_items(
        app,
        "File",
        true,
        &[&new_file, &open, &separator0, &quit],
    )?;

    let menu = Menu::with_items(app, &[&file_menu])?;

    app.set_menu(menu)?;

    Ok(())
}
