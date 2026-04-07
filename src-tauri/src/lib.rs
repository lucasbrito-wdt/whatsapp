pub mod commands;
pub mod setup;
pub mod tray;

use tauri::Emitter;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use tauri_plugin_notification::NotificationExt;

#[tauri::command]
async fn relay_notification(title: String, body: String, app: tauri::AppHandle) {
    let title = title.chars().take(128).collect::<String>();
    let body = body.chars().take(512).collect::<String>();
    let _ = app.notification().builder().title(&title).body(&body).show();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            setup::run(app)?;

            app.handle()
                .plugin(tauri_plugin_global_shortcut::Builder::new().build())?;

            app.global_shortcut().on_shortcuts(
                [
                    Shortcut::new(Some(Modifiers::CONTROL), Code::Comma),
                    Shortcut::new(Some(Modifiers::CONTROL), Code::KeyQ),
                ],
                move |app, shortcut, event| {
                    if event.state == ShortcutState::Pressed {
                        if shortcut == &Shortcut::new(Some(Modifiers::CONTROL), Code::Comma) {
                            let _ = app.emit("toggle-settings", ());
                        } else if shortcut
                            == &Shortcut::new(Some(Modifiers::CONTROL), Code::KeyQ)
                        {
                            app.exit(0);
                        }
                    }
                },
            )?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            relay_notification,
            commands::audio::list_audio_inputs,
            commands::audio::list_audio_outputs,
            commands::session::set_autostart,
            commands::session::check_for_updates,
            commands::window::save_window_state,
            commands::window::restore_window_state,
            commands::window::minimize_window,
            commands::window::toggle_maximize_window,
            commands::window::close_window,
            commands::window::start_dragging_window,
            commands::tray::set_badge_count,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
