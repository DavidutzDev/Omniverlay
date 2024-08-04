// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod utils;

use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use omniverlay_core::{get_omniverlay, Omniverlay, TAURI_APP_HANDLE};
use once_cell::sync::OnceCell;
use tauri::{AppHandle, WindowBuilder};
use utils::tray::create_system_tray;

const OVERLAY_SIZE: tauri::Size = tauri::Size::Physical(tauri::PhysicalSize {
    width: 256,
    height: 256,
});

const STUDIO_SIZE: tauri::Size = tauri::Size::Physical(tauri::PhysicalSize {
    width: 1280,
    height: 720,
});

fn main() {
    tauri::Builder::default()
        .setup(move |app| {
            use tauri::Manager;

            let studio_window = WindowBuilder::new(app, "studio", tauri::WindowUrl::App("/studio".into())).build().unwrap();

            studio_window.set_min_size(Some(STUDIO_SIZE)).unwrap();
            studio_window.set_size(STUDIO_SIZE).unwrap();

            let overlay_window = app.get_window("main").unwrap();

            overlay_window.set_min_size(Some(OVERLAY_SIZE)).unwrap();
            overlay_window.set_size(OVERLAY_SIZE).unwrap();
            overlay_window.center().unwrap();
            overlay_window.set_decorations(false).unwrap();

            overlay_window.open_devtools();

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
                info!("Not Windows, window transparency not implmented yet !")
            }

            Ok(())
        })
        .system_tray(create_system_tray())
        .invoke_handler(tauri::generate_handler![
            commands::bootstrap_backend,
            commands::extensions::list_extensions,
            commands::extensions::update_extension_geometry
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
