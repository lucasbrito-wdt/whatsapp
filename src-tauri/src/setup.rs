use std::os::unix::fs::PermissionsExt;
use std::process::Command;
use tauri::{App, Manager, WebviewUrl, WebviewWindowBuilder};
use tauri::webview::NewWindowResponse;

const TITLEBAR_SCRIPT: &str = r##"
(function() {
  if (!window.__TAURI_INTERNALS__) return;
  var inv = function(cmd) {
    try { window.__TAURI_INTERNALS__.invoke(cmd); } catch(e) {}
  };
  var css = [
    '#__watb{position:fixed;top:0;left:0;right:0;height:32px;background:#1f2c34;',
    'display:flex;align-items:center;z-index:2147483647;',
    '-webkit-user-select:none;user-select:none;box-sizing:border-box}',
    '#__watb_drag{flex:1;height:100%;cursor:default}',
    '#__watb_info{display:flex;align-items:center;gap:6px;padding-left:12px;pointer-events:none}',
    '#__watb_btns{display:flex}',
    '#__watb_btns button{width:44px;height:32px;border:0;background:transparent;',
    'color:#aebac1;cursor:pointer;font-size:16px;line-height:1;',
    'display:flex;align-items:center;justify-content:center}',
    '#__watb_btns button:hover{background:rgba(255,255,255,0.1)}',
    '#__watb_close:hover{background:#c42b1c!important;color:#fff!important}',
    '#app{margin-top:32px!important;height:calc(100% - 32px)!important}',
    '#__watb{transition:top .15s ease}'
  ].join('');
  var waIcon = '<svg width="18" height="18" viewBox="0 0 24 24" fill="#25D366"><path d="M12.04 2C6.58 2 2.13 6.45 2.13 11.91 2.13 13.66 2.59 15.36 3.45 16.86L2.05 22l5.25-1.38c1.45.79 3.08 1.21 4.74 1.21 5.46 0 9.91-4.45 9.91-9.91C21.95 9.27 20.92 6.78 19.05 4.91 17.18 3.03 14.69 2 12.04 2zm.01 1.67c2.2 0 4.26.86 5.82 2.42 1.55 1.56 2.41 3.63 2.41 5.83 0 4.54-3.7 8.23-8.24 8.23-1.48 0-2.93-.39-4.19-1.13l-.3-.17-3.12.8.82-3.04-.19-.32C4.24 14.98 3.8 13.47 3.8 11.91c.01-4.54 3.71-8.24 8.25-8.24z"/></svg>';
  function init() {
    if (document.getElementById('__watb')) return;
    var s = document.createElement('style'); s.textContent = css;
    var b = document.createElement('div'); b.id = '__watb';
    b.innerHTML = '<div id="__watb_info">' + waIcon + '<span style="color:#e9edef;font-size:13px;font-family:system-ui,sans-serif;font-weight:500">WhatsApp</span></div>'
      + '<div id="__watb_drag"></div>'
      + '<div id="__watb_btns"><button id="__watb_min" title="Minimizar">&#x2212;</button><button id="__watb_max" title="Maximizar">&#x25A1;</button><button id="__watb_close" title="Fechar">&#x2715;</button></div>';
    document.head.appendChild(s);
    document.body.insertBefore(b, document.body.firstChild);
    document.getElementById('__watb_drag').onmousedown = function(e) { if (e.button === 0) inv('start_dragging_window'); };
    document.getElementById('__watb_info').onmousedown = function(e) { if (e.button === 0) inv('start_dragging_window'); };
    document.getElementById('__watb_min').onclick = function() { inv('minimize_window'); };
    document.getElementById('__watb_max').onclick = function() { inv('toggle_maximize_window'); };
    document.getElementById('__watb_close').onclick = function() { inv('close_window'); };
    // Push WhatsApp app container below titlebar (retry until #app is mounted by React)
    function pushApp() {
      var app = document.getElementById('app');
      if (app) {
        app.style.setProperty('margin-top', '32px', 'important');
        app.style.setProperty('height', 'calc(100% - 32px)', 'important');
        return true;
      }
      return false;
    }
    if (!pushApp()) {
      var tries = 0;
      var t = setInterval(function() { if (pushApp() || ++tries > 30) clearInterval(t); }, 100);
    }
    // Auto-hide titlebar when WhatsApp opens a full-viewport overlay (photo viewer, calls, etc.)
    // WhatsApp renders these as fixed-position children of #app (not always direct body children)
    function checkCover() {
      var bar = document.getElementById('__watb');
      if (!bar) return;
      var vw = window.innerWidth - 4, vh = window.innerHeight - 4;
      // Check direct children of both body and #app
      var candidates = document.querySelectorAll('body > *:not(#__watb), #app > *');
      var covered = false;
      for (var i = 0; i < candidates.length; i++) {
        var r = candidates[i].getBoundingClientRect();
        // An element starting at the top and covering almost the full viewport = overlay
        if (r.width >= vw && r.height >= vh && r.top <= 4) { covered = true; break; }
      }
      bar.style.top = covered ? '-32px' : '0';
    }
    var _checkT = null;
    function schedCheck() { clearTimeout(_checkT); _checkT = setTimeout(checkCover, 80); }
    var _obs = new MutationObserver(schedCheck);
    _obs.observe(document.body, { childList: true });
    // Start watching #app children once React mounts it
    var _appObs = setInterval(function() {
      var app = document.getElementById('app');
      if (app) { _obs.observe(app, { childList: true }); clearInterval(_appObs); }
    }, 300);
    window.addEventListener('resize', checkCover);
  }
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', init);
  } else {
    init();
  }
  setTimeout(init, 500);
})();
"##;

const WHATSAPP_URL: &str = "https://web.whatsapp.com";
const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36";

const NOTIFICATION_BRIDGE: &str = r#"
(function() {
    if (!window.__TAURI_INTERNALS__) return;
    const _OrigNotif = window.Notification;
    window.Notification = function(title, options) {
        window.__TAURI_INTERNALS__.invoke('relay_notification', {
            title: title,
            body: (options && options.body) ? options.body : ''
        });
        return new _OrigNotif(title, options);
    };
    window.Notification.prototype = _OrigNotif.prototype;
    window.Notification.requestPermission = _OrigNotif.requestPermission.bind(_OrigNotif);
})();
"#;

const BACKGROUND_SYNC_POLYFILL: &str = r#"
(function() {
    // WebKitGTK does not implement the Background Sync API (SyncManager / PeriodicSyncManager).
    // This polyfill mocks both APIs so WhatsApp Web believes sync is registered
    // and hides the "Ative a sincronização em segundo plano" banner.
    if (typeof SyncManager !== 'undefined') return;

    var _tags = [];

    function SyncManagerMock() {}
    SyncManagerMock.prototype.register = function(tag) {
        if (_tags.indexOf(tag) === -1) _tags.push(tag);
        return Promise.resolve();
    };
    SyncManagerMock.prototype.getTags = function() {
        return Promise.resolve(_tags.slice());
    };

    function PeriodicSyncManagerMock() {}
    PeriodicSyncManagerMock.prototype.register = function(tag) {
        if (_tags.indexOf(tag) === -1) _tags.push(tag);
        return Promise.resolve();
    };
    PeriodicSyncManagerMock.prototype.getTags = function() {
        return Promise.resolve(_tags.slice());
    };
    PeriodicSyncManagerMock.prototype.unregister = function(tag) {
        _tags = _tags.filter(function(t) { return t !== tag; });
        return Promise.resolve();
    };

    window.SyncManager = SyncManagerMock;
    window.PeriodicSyncManager = PeriodicSyncManagerMock;

    if ('serviceWorker' in navigator && window.ServiceWorkerRegistration) {
        var syncInst = new SyncManagerMock();
        var periodicInst = new PeriodicSyncManagerMock();
        Object.defineProperty(window.ServiceWorkerRegistration.prototype, 'sync', {
            get: function() { return syncInst; },
            configurable: true
        });
        Object.defineProperty(window.ServiceWorkerRegistration.prototype, 'periodicSync', {
            get: function() { return periodicInst; },
            configurable: true
        });
    }
})();
"#;

pub fn run(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    check_gstreamer();

    let webview_data_dir = setup_data_dir(app)?;

    let url: url::Url = WHATSAPP_URL.parse().expect("valid URL");

    // Combine all injection scripts
    let init_script = format!("{}\n{}\n{}", TITLEBAR_SCRIPT, NOTIFICATION_BRIDGE, BACKGROUND_SYNC_POLYFILL);

    let _window = WebviewWindowBuilder::new(app, "main", WebviewUrl::External(url))
        .title("WhatsApp")
        .inner_size(1200.0, 800.0)
        .min_inner_size(800.0, 600.0)
        .decorations(false)
        .center()
        .user_agent(USER_AGENT)
        .data_directory(webview_data_dir)
        .initialization_script(&init_script)
        .on_navigation(|nav_url| {
            let host = nav_url.host_str().unwrap_or("");
            is_whatsapp_host(host)
        })
        .on_new_window(|nav_url, _features| {
            let host = nav_url.host_str().unwrap_or("");
            if is_whatsapp_host(host) {
                NewWindowResponse::Allow
            } else {
                let scheme = nav_url.scheme();
                if scheme != "http" && scheme != "https" {
                    return NewWindowResponse::Deny;
                }
                let url_str = nav_url.to_string();
                std::thread::spawn(move || {
                    let _ = Command::new("xdg-open").arg(&url_str).spawn();
                });
                NewWindowResponse::Deny
            }
        })
        .build()?;

    // Auto-grant microphone/camera permissions (needed for voice messages and calls)
    #[cfg(target_os = "linux")]
    {
        _window.with_webview(|webview| {
            use webkit2gtk::WebViewExt;
            webview.inner().connect_permission_request(|_, request| {
                use webkit2gtk::PermissionRequestExt;
                request.allow();
                true
            });
        })?;
    }

    crate::tray::setup_tray(app.handle())?;

    let app_handle = app.handle().clone();
    tauri::async_runtime::spawn(async move {
        if let Err(e) = crate::commands::window::restore_window_state(app_handle).await {
            log::warn!("Could not restore window state: {}", e);
        }
    });

    Ok(())
}

fn is_whatsapp_host(host: &str) -> bool {
    host == "web.whatsapp.com"
        || host.ends_with(".whatsapp.net")
        || host.ends_with(".whatsapp.com")
}

fn setup_data_dir(app: &App) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let app_local_data = app.path().app_local_data_dir()?;
    let webview_data_dir = app_local_data.join("webview-data");

    std::fs::create_dir_all(&webview_data_dir)?;

    let permissions = std::fs::Permissions::from_mode(0o700);
    std::fs::set_permissions(&webview_data_dir, permissions)?;

    Ok(webview_data_dir)
}

fn check_gstreamer() {
    let result = Command::new("gst-inspect-1.0")
        .arg("pipewiresrc")
        .output();

    match result {
        Ok(output) if output.status.success() => {
            log::info!("GStreamer PipeWire support detected");
        }
        Ok(_) => {
            log::warn!(
                "gstreamer1.0-pipewire not available — audio/video calls may use ALSA fallback"
            );
        }
        Err(e) => {
            log::warn!(
                "Could not run gst-inspect-1.0: {} — GStreamer may not be installed",
                e
            );
        }
    }
}
