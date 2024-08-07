use omniverlay_core::{extensions::data::{DataLoader, OmniverlayProfile}, get_omniverlay};
use tauri::{
    AppHandle, CustomMenuItem, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
    SystemTraySubmenu,
};

pub async fn create_system_tray() -> tauri::SystemTray {
    let mut profiles_menu = SystemTrayMenu::new()
    .add_item(CustomMenuItem::new(
        "add_profile".to_string(),
        "Add Profile",
    ))
    .add_native_item(SystemTrayMenuItem::Separator);

    {
        let omniverlay = get_omniverlay();
        let omniverlay_guard = omniverlay.read().await;

        let profile_manager = omniverlay_guard.get_profile_manager().await;
        let profile_manager_guard = profile_manager.read().await;

        for profile in OmniverlayProfile::list_datas().unwrap_or_else(|_| vec![]) {
            let mut item = CustomMenuItem::new(profile.clone(), profile.clone());

            match profile_manager_guard.get_current().await {
                Ok(current_profile) => {
                    if current_profile.read().await.name == profile {
                        item = item.selected();
                    }
                },
                Err(e) => {
                    println!("Failed to get current profile: {}", e);
                }
            }

            profiles_menu = profiles_menu.add_item(item);
        }
    }

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
