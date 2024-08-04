use std::sync::Mutex;

use log::info;
use omniverlay_core::{errors::{OmniverlayError, OmniverlayResult}, extensions::ExtensionGeometry, get_omniverlay, Omniverlay};
use serde::Serialize;
use tauri::{AppHandle, Manager, State};

#[tauri::command]
pub fn list_extensions(app: AppHandle) -> Result<serde_json::Value, String> {
    let overlay = get_omniverlay();

    let extensions = overlay.extension_manager.list_extensions()?;

    let json = serde_json::to_value(extensions).map_err(OmniverlayError::SerdeJson)?;

    Ok(json)
}

#[tauri::command]
pub fn update_extension_geometry(app: AppHandle, name: String, geometry: ExtensionGeometry) -> Result<(), String> {
    info!("Invoked update_extension_geometry with name: {} and geometry: {:?}", name, geometry);

    let mut overlay = get_omniverlay();

    overlay
        .extension_manager
        .update_extension_geometry(&name, geometry)
        .map_err(|_| OmniverlayError::ExtensionNotFound(name))?;

    app.emit_all("Omniverlay://refresh_extensions_data", true).unwrap();

    Ok(())
}