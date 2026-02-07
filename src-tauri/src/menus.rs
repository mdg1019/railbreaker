use tauri::{menu::{Menu, MenuItem, Submenu, PredefinedMenuItem}, App, Manager};

pub fn setup_menus(app: &App) -> tauri::Result<()> {
    let open = MenuItem::with_id(app, "open", "Open Racecard…", true, Some("CmdOrCtrl+O"))?;
    let open_zip = MenuItem::with_id(app, "open-zip", "Open Zip…", true, Some("CmdOrCtrl+Shift+O"))?;
    let separator0 = PredefinedMenuItem::separator(app)?;
    let print_racecard = MenuItem::with_id(app, "print-racecard", "Print Racecard…", true, Some("CmdOrCtrl+Shift+P"))?;
    let _ = print_racecard.set_enabled(false);
    let separator1 = PredefinedMenuItem::separator(app)?;
    let quit = MenuItem::with_id(app, "exit", "Exit", true, Some("CmdOrCtrl+Q"))?;
    let about = MenuItem::with_id(app, "about", "About RailBreaker", true, None::<&str>)?;
    let help = MenuItem::with_id(app, "help-item", "Help", true, Some("CmdOrCtrl+H"))?;

    let prev_page = MenuItem::with_id(app, "prev-page", "Previous Page", true, Some("CmdOrCtrl+P"))?;
    let separator_view = PredefinedMenuItem::separator(app)?;
    let sort_horses = MenuItem::with_id(app, "sort-horses", "Sort Horses", true, Some("CmdOrCtrl+S"))?;
    let next_page = MenuItem::with_id(app, "next-page", "Next Page", true, Some("CmdOrCtrl+N"))?;
    let _ = next_page.set_enabled(false);
    let _ = prev_page.set_enabled(false);
    let _ = sort_horses.set_enabled(false);

    let file_menu = Submenu::with_id_and_items(
        app,
        "file",
        "File",
        true,
        &[&open, &open_zip, &separator0, &print_racecard, &separator1, &quit],
    )?;
    let view_menu = Submenu::with_id_and_items(
        app,
        "view",
        "View",
        true,
        &[&next_page, &prev_page, &separator_view, &sort_horses],
    )?;
    let help_menu = Submenu::with_id_and_items(app, "help", "Help", true, &[&help, &about])?;

    let menu = Menu::with_items(app, &[&file_menu, &view_menu, &help_menu])?;

    if let Some(window) = app.get_webview_window("main") {
        window.set_menu(menu)?;
    }

    Ok(())
}
