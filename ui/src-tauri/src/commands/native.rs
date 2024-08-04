use log::info;
use serde::{Deserialize, Serialize};
use tauri::window::Monitor;
use tauri::{AppHandle, Manager, Size};

#[derive(Debug, Serialize, Deserialize)]
pub struct WindowInfos {
    pub width: u32,
    pub height: u32,
    pub dpi: f64,
}

#[tauri::command]
pub fn open_url(app_handle: AppHandle, url: String) {
    webbrowser::open(&url).unwrap();
}

#[tauri::command]
pub fn get_window_infos(app_handle: AppHandle) -> Result<WindowInfos, String> {
    // Retrieve the focused window
    if let Some(window) = app_handle.get_focused_window() {
        // Get the monitor for the focused window
        if let Some(monitor) = window.current_monitor().unwrap() {
            // Retrieve the size and scale factor
            let size = monitor.size();
            let dpi = monitor.scale_factor();

            // Log window information
            info!("Window size: {}x{}", size.width, size.height);
            info!("DPI: {}", dpi);

            // Return window information in the Result
            return Ok(WindowInfos {
                width: size.width,
                height: size.height,
                dpi,
            });
        } else {
            return Err("Could not get the current monitor.".to_string());
        }
    } else {
        return Err("Could not get the focused window.".to_string());
    }
}
