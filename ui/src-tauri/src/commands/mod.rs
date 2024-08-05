use std::sync::Arc;

use omniverlay_core::{errors::OmniverlayError, get_omniverlay, TAURI_APP_HANDLE};
use tauri::AppHandle;

pub mod extensions;
pub mod native;

#[tauri::command]
pub async fn bootstrap_backend(app: AppHandle) -> Result<(), String> {
    let mut omniverlay = get_omniverlay();

    TAURI_APP_HANDLE.set(Arc::new(app)).map_err(|_| OmniverlayError::BackendInitialization("Failed to get TAURI_APP_HANDLE".to_string()))?;

    omniverlay
        .extension_manager
        .register_extension(Box::new(performance::PerformanceExtension::new()));

    omniverlay
        .extension_manager
        .enable_extension("Performance")
        .map_err(|_| OmniverlayError::ExtensionLoadFailed("Performance".to_string()))?;

    Ok(())
}
