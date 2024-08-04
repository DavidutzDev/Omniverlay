use tauri::{CustomMenuItem, SystemTrayMenu};

pub fn create_system_tray() -> tauri::SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new().add_item(quit);

    tauri::SystemTray::new().with_menu(tray_menu)
}