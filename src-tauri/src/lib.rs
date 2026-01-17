// Reader3 - Tauri App Shell with Python Sidecar
// Feature: 001-tauri-sidecar-shell

mod config;
mod error;
mod health;
mod logging;
mod sidecar;
mod state;

use tauri::Manager;
use tracing::info;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize logging first
    logging::init_logging();

    info!("Starting Reader3 application");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            // Focus existing window when second instance is launched
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_focus();
                let _ = window.unminimize();
            }
            info!("Second instance blocked - focusing existing window");
        }))
        .setup(|app| {
            let app_handle = app.handle().clone();

            // Initialize app configuration
            let config = config::AppConfig::new(&app_handle)?;
            info!("App data directory: {:?}", config.app_data_dir);

            // Store config in app state
            app.manage(config);

            // Spawn sidecar and start health check polling
            let window = app.get_webview_window("main").unwrap();

            tauri::async_runtime::spawn(async move {
                match sidecar::start_sidecar_and_wait(&app_handle).await {
                    Ok(()) => {
                        info!("Sidecar healthy, navigating to library");
                        let _ = window.eval("window.location.href = 'http://127.0.0.1:8123/'");
                    }
                    Err(e) => {
                        tracing::error!("Sidecar startup failed: {:?}", e);
                        error::show_error_dialog(&app_handle, e);
                    }
                }
            });

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                info!("Window close requested, shutting down sidecar");
                let app_handle = window.app_handle().clone();
                tauri::async_runtime::block_on(async {
                    sidecar::shutdown_sidecar(&app_handle).await;
                });
            }
        })
        .invoke_handler(tauri::generate_handler![restart_sidecar])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Tauri command to manually restart the sidecar (used after auto-restart fails)
#[tauri::command]
async fn restart_sidecar(app: tauri::AppHandle) -> Result<(), String> {
    info!("Manual sidecar restart requested");
    sidecar::restart_sidecar(&app).await.map_err(|e| e.to_string())
}
