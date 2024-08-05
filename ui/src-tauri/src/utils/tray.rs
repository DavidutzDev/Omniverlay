use tauri::{
    AppHandle, CustomMenuItem, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
    SystemTraySubmenu,
};

pub fn create_system_tray() -> tauri::SystemTray {
    let profiles_menu = SystemTrayMenu::new().add_item(CustomMenuItem::new(
        "add_profile".to_string(),
        "Add Profile",
    ));
    let profiles_submenu = SystemTraySubmenu::new("Profiles", profiles_menu);

    let layouts_menu =
        SystemTrayMenu::new().add_item(CustomMenuItem::new("add_layout".to_string(), "Add Layout"));
    let layouts_submenu = SystemTraySubmenu::new("Layouts", layouts_menu);

    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("quit".to_string(), "Show Overlay"))
        .add_item(CustomMenuItem::new("quit".to_string(), "Open Studio"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_submenu(profiles_submenu)
        .add_submenu(layouts_submenu)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit"));

    tauri::SystemTray::new().with_menu(tray_menu)
}

pub fn on_system_tray_event(app_handle: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => {
                app_handle.exit(0);
            }
            _ => {}
        },
        _ => {}
    }
}
