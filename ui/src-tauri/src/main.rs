// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod utils;

use std::sync::Arc;

use omniverlay_core::{errors::OmniverlayError, event::{EventHandler, OmniverlayEventType, EVENT_HANDLER}, get_omniverlay};
use performance::PerformanceExtension;
use tauri::{AppHandle, WindowBuilder, Manager};
use tokio::sync::RwLock;
use utils::tray::{create_system_tray, on_system_tray_event};

const OVERLAY_SIZE: tauri::Size = tauri::Size::Physical(tauri::PhysicalSize {
    width: 0,
    height: 0,
});

const STUDIO_SIZE: tauri::Size = tauri::Size::Physical(tauri::PhysicalSize {
    width: 1280,
    height: 720,
});

#[tauri::command]
async fn bootstrap_backend(app: AppHandle) -> Result<(), String> {
    let app_handle: Arc<AppHandle> = Arc::new(app);

    let event: EventHandler = Box::new(move |event| {
        match event.event_type {
            OmniverlayEventType::UpdateExtensions => {
                app_handle.clone().emit_all("Omniverlay://refresh_extensions_data", true).unwrap();
            },
        }
    });

    EVENT_HANDLER.set(RwLock::new(Some(event))).map_err(|_| OmniverlayError::BackendInitialization("Failed to get EVENT_HANDLER".to_string()))?; 

    let omniverlay = get_omniverlay();
    let guard = omniverlay.read().await;

    // Register extension (Write lock)
    {
        guard.get_extension_manager().await.write().await.register_extension(PerformanceExtension::new()).await?;
    }

    guard.startup().await.map_err(|_| OmniverlayError::BackendInitialization("Failed to start Omniverlay".to_string()))?;

    //invoke_event!(guard, EventType::UpdateExtensions("AAaaaaa".to_string()));

    //TAURI_APP_HANDLE.set(Arc::new(app)).map_err(|_| OmniverlayError::BackendInitialization("Failed to get TAURI_APP_HANDLE".to_string()))?;
    
    Ok(())
}


#[tokio::main]
async  fn main() {
    tauri::Builder::default()
        .setup(move |app| {
            use tauri::Manager;

            let studio_window = WindowBuilder::new(app, "studio", tauri::WindowUrl::App("/studio".into())).build().unwrap();

            studio_window.set_min_size(Some(STUDIO_SIZE)).unwrap();
            studio_window.set_size(STUDIO_SIZE).unwrap();
            studio_window.set_title("Omniverlay Studio").unwrap();
            studio_window.set_resizable(false).unwrap();

            let overlay_window = app.get_window("main").unwrap();

            overlay_window.set_min_size(Some(OVERLAY_SIZE)).unwrap();
            overlay_window.set_size(OVERLAY_SIZE).unwrap();
            overlay_window.center().unwrap();

            let _ = overlay_window.set_fullscreen(true);

            // Windows Implementation with native API to set Click through
            #[cfg(target_os = "windows")]
            {
                let hwnd = overlay_window.hwnd().unwrap().0;
                let _pre_val;
                let hwnd = windows::Win32::Foundation::HWND(hwnd as *mut _);

                unsafe {
                    use windows::Win32::UI::WindowsAndMessaging::*;
                    let nindex = GWL_EXSTYLE;
                    let style = WS_EX_APPWINDOW
                        | WS_EX_COMPOSITED
                        | WS_EX_LAYERED
                        | WS_EX_TRANSPARENT
                        | WS_EX_TOPMOST;
                    _pre_val = SetWindowLongA(hwnd, nindex, style.0 as i32);
                };
            }
            #[cfg(not(target_os = "windows"))]
            {
                panic!("Not Windows, window Click through not implmented yet !")
            }

            Ok(())
        })
        .system_tray(create_system_tray().await)
        .on_system_tray_event(|app_handle, event| on_system_tray_event(app_handle, event))
        .invoke_handler(tauri::generate_handler![
            bootstrap_backend,
            commands::native::open_url,
            commands::extensions::list_extensions,
            commands::extensions::update_extensions_state,
            commands::extensions::update_extensions_layout
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}