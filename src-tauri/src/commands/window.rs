use tauri::{AppHandle, Manager, WebviewWindow};
use tauri_plugin_store::StoreExt;

#[tauri::command]
pub fn minimize_window(app: AppHandle) -> Result<(), String> {
    let win = app.get_webview_window("main").ok_or("window not found")?;
    win.minimize().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn toggle_maximize_window(app: AppHandle) -> Result<(), String> {
    let win = app.get_webview_window("main").ok_or("window not found")?;
    if win.is_maximized().map_err(|e| e.to_string())? {
        win.unmaximize().map_err(|e| e.to_string())
    } else {
        win.maximize().map_err(|e| e.to_string())
    }
}

#[tauri::command]
pub fn close_window(app: AppHandle) -> Result<(), String> {
    let win = app.get_webview_window("main").ok_or("window not found")?;
    win.close().map_err(|e| e.to_string())
}

// Non-async so the OS drag starts while the mouse button is still held
#[tauri::command]
pub fn start_dragging_window(window: WebviewWindow) -> Result<(), String> {
    window.start_dragging().map_err(|e| e.to_string())
}

const STORE_PATH: &str = "window-state.json";
const KEY_X: &str = "x";
const KEY_Y: &str = "y";
const KEY_WIDTH: &str = "width";
const KEY_HEIGHT: &str = "height";

#[tauri::command]
pub async fn save_window_state(app: AppHandle) -> Result<(), String> {
    let window = app
        .get_webview_window("main")
        .ok_or_else(|| "Main window not found".to_string())?;

    let position = window
        .outer_position()
        .map_err(|e| format!("Failed to get position: {}", e))?;
    let size = window
        .outer_size()
        .map_err(|e| format!("Failed to get size: {}", e))?;

    let store = app
        .store(STORE_PATH)
        .map_err(|e| format!("Failed to open store: {}", e))?;

    store.set(KEY_X, position.x);
    store.set(KEY_Y, position.y);
    store.set(KEY_WIDTH, size.width);
    store.set(KEY_HEIGHT, size.height);
    store.save().map_err(|e| format!("Failed to save store: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn restore_window_state(app: AppHandle) -> Result<(), String> {
    let store = app
        .store(STORE_PATH)
        .map_err(|e| format!("Failed to open store: {}", e))?;

    let x = store.get(KEY_X).and_then(|v| v.as_f64());
    let y = store.get(KEY_Y).and_then(|v| v.as_f64());
    let width = store.get(KEY_WIDTH).and_then(|v| v.as_f64());
    let height = store.get(KEY_HEIGHT).and_then(|v| v.as_f64());

    let window = app
        .get_webview_window("main")
        .ok_or_else(|| "Main window not found".to_string())?;

    if let (Some(w), Some(h)) = (width, height) {
        window
            .set_size(tauri::LogicalSize::new(w, h))
            .map_err(|e| format!("Failed to set size: {}", e))?;
    }

    if let (Some(x), Some(y)) = (x, y) {
        window
            .set_position(tauri::LogicalPosition::new(x, y))
            .map_err(|e| format!("Failed to set position: {}", e))?;
    }

    Ok(())
}
