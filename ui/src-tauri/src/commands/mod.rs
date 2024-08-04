use std::{sync::Arc, thread, time::Duration};

use log::info;
use omniverlay_core::{errors::OmniverlayError, get_omniverlay, Omniverlay, TAURI_APP_HANDLE};
use tauri::{AppHandle, Manager};
use winapi::um::winuser::{GetDpiForWindow, GetForegroundWindow};

pub mod extensions;
pub mod native;

#[tauri::command]
pub async fn bootstrap_backend(app: AppHandle) -> Result<(), String> {
    let mut omniverlay = get_omniverlay();

    TAURI_APP_HANDLE.set(Arc::new(app));

    omniverlay
        .extension_manager
        .register_extension(Box::new(performance::PerformanceExtension::new()));

    omniverlay
        .extension_manager
        .enable_extension("Performance")
        .map_err(|_| OmniverlayError::ExtensionLoadFailed("Performance".to_string()))?;

    unsafe {
        let hwnd = GetForegroundWindow(); // Obtenez le handle de la fenêtre en plein écran
        let dpi = GetDpiForWindow(hwnd);

        info!("DPI: {}", dpi);
    }

    //omniverlay.set_app_handle(Arc::new(app));

    Ok(())
}
