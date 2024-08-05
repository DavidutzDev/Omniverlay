use log::info;
use omniverlay_core::{errors::OmniverlayError, extensions::ExtensionInfo, get_omniverlay};
use tauri::{AppHandle, Manager};

#[tauri::command]
pub fn list_extensions() -> Result<serde_json::Value, String> {
    let overlay = get_omniverlay();

    let extensions = overlay.extension_manager.list_extensions()?;

    let json = serde_json::to_value(extensions).map_err(OmniverlayError::SerdeJson)?;

    Ok(json)
}

#[tauri::command]
pub fn update_extensions(app: AppHandle, extensions: Vec<ExtensionInfo>) -> Result<(), String> {
    info!("Invoked update_extensions with extensions: {:?}", extensions);

    let mut omniverlay = get_omniverlay();

    //TODO: ASYNC
    for extension in &extensions {
        omniverlay.extension_manager.update_extension(extension.clone()).map_err(|_| OmniverlayError::ExtensionNotFound(extension.name.clone()))?;
    }

    app.emit_all("Omniverlay://refresh_extensions_data", true).unwrap();

    Ok(())
}