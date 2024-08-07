use std::collections::HashMap;

use log::info;
use omniverlay_core::{
    errors::OmniverlayError, event::OmniverlayEventType, extensions::{ExtensionLayout, ExtensionState}, get_omniverlay, invoke_event
};

#[tauri::command]
pub async fn list_extensions() -> Result<serde_json::Value, String> {
    let omniverlay = get_omniverlay();
    let guard = omniverlay.read().await;

    let extensions = guard
        .get_extension_manager()
        .await
        .read()
        .await
        .list_extensions()
        .await?;

    let json = serde_json::to_value(extensions).map_err(OmniverlayError::SerdeJson)?;

    Ok(json)
}

#[tauri::command]
pub async fn update_extensions_state(
    states: HashMap<String, ExtensionState>,
) -> Result<(), String> {
    info!("Invoked update_extensions_state with states: {:?}", states);

    let omniverlay = get_omniverlay();
    let omniverlay_guard = omniverlay.read().await;


    let profile_manager = omniverlay_guard.get_profile_manager().await;
    let profile_manager_guard = profile_manager.read().await;

    {
        let profile = profile_manager_guard.get_current().await?;
        let mut profile_guard = profile.write().await;

        for (name, state) in &states {
            info!("Updating extension state for {} to {:?}", name, state);
            profile_guard.extensions.insert(name.clone(), state.clone());
        }
    }

    profile_manager_guard.save_data().await?;

    invoke_event!(OmniverlayEventType::UpdateExtensions);

    Ok(())
}

#[tauri::command]
pub async fn update_extensions_layout(layouts: HashMap<String, ExtensionLayout>) -> Result<(), String> {
    info!("Invoked update_extension_layout with layouts: {:?}", layouts);

    let omniverlay = get_omniverlay();
    let omniverlay_guard = omniverlay.read().await;

    info!("Get omniverlay_guard: ");

    let layout_manager = omniverlay_guard.get_layout_manager().await;
    let layout_manager_guard = layout_manager.read().await;

    {
        let layout = layout_manager_guard.get_current().await?;
        let mut layout_guard = layout.write().await;

        for (name, layout) in &layouts {
            info!("Updating extension layout for {} to {:?}", name, layout);
            layout_guard.extensions.insert(name.clone(), layout.clone());
        }
    }

    layout_manager_guard.save_data().await?;

    invoke_event!(OmniverlayEventType::UpdateExtensions);

    Ok(())
}