use tauri::AppHandle;

#[tauri::command]
pub async fn set_badge_count(count: u32, app: AppHandle) -> Result<(), String> {
    let tray = app
        .tray_by_id("main")
        .ok_or_else(|| "Tray icon not found".to_string())?;

    let tooltip = if count > 0 {
        format!("WhatsApp ({} unread)", count)
    } else {
        "WhatsApp".to_string()
    };

    tray.set_tooltip(Some(&tooltip))
        .map_err(|e| format!("Failed to update tray tooltip: {}", e))?;

    Ok(())
}
