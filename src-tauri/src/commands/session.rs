use std::fs;

const KEYRING_SERVICE: &str = "com.whatsapp.unofficial";
const KEYRING_USER: &str = "session";
const AUTOSTART_DESKTOP: &str = "[Desktop Entry]
Type=Application
Name=WhatsApp
Exec=whatsapp
Hidden=false
NoDisplay=false
X-GNOME-Autostart-enabled=true
";

#[tauri::command]
pub async fn save_session_key(key: String) -> Result<(), String> {
    let entry = keyring_core::Entry::new(KEYRING_SERVICE, KEYRING_USER)
        .map_err(|e| format!("Keyring entry error: {}", e))?;
    entry
        .set_password(&key)
        .map_err(|e| format!("Failed to save session key: {}", e))
}

#[tauri::command]
pub async fn get_session_key() -> Result<Option<String>, String> {
    let entry = keyring_core::Entry::new(KEYRING_SERVICE, KEYRING_USER)
        .map_err(|e| format!("Keyring entry error: {}", e))?;
    match entry.get_password() {
        Ok(password) => Ok(Some(password)),
        Err(keyring_core::Error::NoEntry) => Ok(None),
        Err(e) => Err(format!("Failed to get session key: {}", e)),
    }
}

#[tauri::command]
pub async fn set_autostart(enabled: bool, app: tauri::AppHandle) -> Result<(), String> {
    use tauri::Manager;
    let config_dir = app.path().config_dir()
        .map_err(|e| e.to_string())?;
    let autostart_dir = config_dir.join("autostart");
    let path = autostart_dir.join("whatsapp-unofficial.desktop");

    if enabled {
        fs::create_dir_all(&autostart_dir)
            .map_err(|e| format!("Failed to create autostart dir: {e}"))?;
        fs::write(&path, AUTOSTART_DESKTOP)
            .map_err(|e| format!("Failed to write autostart file: {}", e))?;
    } else if path.exists() {
        fs::remove_file(&path)
            .map_err(|e| format!("Failed to remove autostart file: {}", e))?;
    }

    Ok(())
}

#[tauri::command]
pub async fn check_for_updates() -> Result<Option<String>, String> {
    Ok(None)
}
