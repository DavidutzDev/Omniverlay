
#[tauri::command]
pub fn open_url(url: String) {
    webbrowser::open(&url).unwrap();
}