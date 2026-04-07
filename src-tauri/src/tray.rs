use tauri::{
    image::Image,
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, Runtime,
};

const TRAY_ID: &str = "main";

pub fn setup_tray<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<TrayIcon<R>> {
    let show = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
    let hide = MenuItem::with_id(app, "hide", "Hide", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&show, &hide, &quit])?;

    let icon = load_tray_icon(app);

    let mut builder = TrayIconBuilder::with_id(TRAY_ID)
        .tooltip("WhatsApp")
        .menu(&menu)
        .show_menu_on_left_click(false);

    if let Some(img) = icon {
        builder = builder.icon(img);
    }

    let tray = builder
        .on_menu_event(|app, event| handle_menu_event(app, event.id.as_ref()))
        .on_tray_icon_event(|tray: &TrayIcon<R>, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app)?;

    Ok(tray)
}

fn load_tray_icon<R: Runtime>(app: &AppHandle<R>) -> Option<Image<'static>> {
    let resource_path = app
        .path()
        .resource_dir()
        .ok()?
        .join("icons")
        .join("32x32.png");

    Image::from_path(resource_path).ok()
}

fn handle_menu_event<R: Runtime>(app: &AppHandle<R>, event_id: &str) {
    match event_id {
        "show" => {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }
        "hide" => {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.hide();
            }
        }
        "quit" => {
            app.exit(0);
        }
        _ => {}
    }
}
