use omniverlay_core::{
    extensions::data::{DataLoader, OmniverlayLayout, OmniverlayProfile},
    get_omniverlay,
};
use tauri::{
    AppHandle, CustomMenuItem, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
    SystemTraySubmenu,
};

pub async fn get_tray_menu() -> SystemTrayMenu {
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
            let mut item =
                CustomMenuItem::new(format!("profile:{}", profile.clone()), profile.clone());

            match profile_manager_guard.get_current().await {
                Ok(current_profile) => {
                    if current_profile.read().await.name == profile {
                        item = item.selected();
                    }
                }
                Err(e) => {
                    log::error!("Failed to get current profile: {}", e);
                }
            }

            profiles_menu = profiles_menu.add_item(item);
        }
    }

    let profiles_submenu = SystemTraySubmenu::new("Profiles", profiles_menu);

    let mut layouts_menu =
        SystemTrayMenu::new().add_item(CustomMenuItem::new("add_layout".to_string(), "Add Layout"));

    {
        let omniverlay = get_omniverlay();
        let omniverlay_guard = omniverlay.read().await;

        let layout_manager = omniverlay_guard.get_layout_manager().await;
        let layout_manager_guard = layout_manager.read().await;

        for layout in OmniverlayLayout::list_datas().unwrap_or_else(|_| vec![]) {
            let mut item =
                CustomMenuItem::new(format!("layout:{}", layout.clone()), layout.clone());

            match layout_manager_guard.get_current().await {
                Ok(current_layout) => {
                    if current_layout.read().await.name == layout {
                        item = item.selected();
                    }
                }
                Err(e) => {
                    log::error!("Failed to get current layout: {}", e);
                }
            }

            layouts_menu = layouts_menu.add_item(item);
        }
    }

    let layouts_submenu = SystemTraySubmenu::new("Layouts", layouts_menu);

    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("quit".to_string(), "Show Overlay"))
        .add_item(CustomMenuItem::new("quit".to_string(), "Open Studio"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_submenu(profiles_submenu)
        .add_submenu(layouts_submenu)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit"));

    tray_menu
}

pub async fn create_system_tray() -> tauri::SystemTray {
    tauri::SystemTray::new().with_menu(get_tray_menu().await)
}

pub async fn update_system_tray(app_handle: &AppHandle) {
    app_handle
        .tray_handle()
        .set_menu(get_tray_menu().await)
        .expect("Failed to update tray menu");
}

pub async fn on_system_tray_event(app_handle: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => {
            if id.starts_with("profile:") {
                let profile_name = id.strip_prefix("profile:").unwrap_or("");

                {
                    let omniverlay = get_omniverlay();
                    let omniverlay_guard = omniverlay.read().await;

                    let profile_manager = omniverlay_guard.get_profile_manager().await;
                    let profile_manager_guard = profile_manager.read().await;

                    if let Err(e) = profile_manager_guard
                        .switch_data(profile_name.to_string())
                        .await
                    {
                        log::error!("Failed to switch profile: {}", e);
                    }
                }
            } else if id.starts_with("layout:") {
                let layout_name = id.strip_prefix("layout:").unwrap_or("");

                {
                    let omniverlay = get_omniverlay();
                    let omniverlay_guard = omniverlay.read().await;

                    let layout_manager = omniverlay_guard.get_layout_manager().await;
                    let layout_manager_guard = layout_manager.read().await;

                    if let Err(e) = layout_manager_guard
                        .switch_data(layout_name.to_string())
                        .await
                    {
                        log::error!("Failed to switch layout: {}", e);
                    }
                }
            } else {
                match id.as_str() {
                    "quit" => {
                        app_handle.exit(0);
                    }
                    "add_profile" => {
                        let omnierlay = get_omniverlay();
                        let omniverlay_guard = omnierlay.read().await;

                        let profile_manager = omniverlay_guard.get_profile_manager().await;
                        let profile_manager_guard = profile_manager.read().await;

                        let used_names = OmniverlayProfile::list_datas().unwrap_or_else(|_| vec![]);

                        profile_manager_guard
                            .new_data(format!("Profile {}", used_names.len() + 1))
                            .await
                            .unwrap();
                    },
                    "add_layout" => {
                        let omniverlay = get_omniverlay();
                        let omniverlay_guard = omniverlay.read().await;

                        let layout_manager = omniverlay_guard.get_layout_manager().await;
                        let layout_manager_guard = layout_manager.read().await;

                        let used_names = OmniverlayLayout::list_datas().unwrap_or_else(|_| vec![]);

                        layout_manager_guard
                            .new_data(format!("Layout {}", used_names.len() + 1))
                            .await
                            .unwrap();
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
}
