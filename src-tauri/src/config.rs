// Application configuration and path resolution
// Feature: 001-tauri-sidecar-shell

use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use tracing::info;

/// Application-level configuration derived from Tauri app paths
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// ~/Library/Application Support/Reader3/
    pub app_data_dir: PathBuf,

    /// ~/Library/Application Support/Reader3/books/
    pub books_dir: PathBuf,

    /// ~/Library/Application Support/Reader3/config/ai_settings.json
    pub ai_settings_path: PathBuf,

    /// ~/Library/Application Support/Reader3/logs/
    pub log_dir: PathBuf,

    /// Fixed port for sidecar HTTP server
    pub sidecar_port: u16,

    /// Health check configuration
    pub health_check_interval_ms: u64,
    pub health_check_timeout_s: u64,
}

impl AppConfig {
    /// Create a new AppConfig from Tauri app paths
    pub fn new(app: &AppHandle) -> Result<Self, Box<dyn std::error::Error>> {
        let app_data_dir = app
            .path()
            .app_data_dir()
            .map_err(|e| format!("Failed to get app data dir: {}", e))?;

        let books_dir = app_data_dir.join("books");
        let config_dir = app_data_dir.join("config");
        let log_dir = app_data_dir.join("logs");
        let ai_settings_path = config_dir.join("ai_settings.json");

        // Create directories if they don't exist
        std::fs::create_dir_all(&books_dir)?;
        std::fs::create_dir_all(&config_dir)?;
        std::fs::create_dir_all(&log_dir)?;

        info!("App data directory: {:?}", app_data_dir);
        info!("Books directory: {:?}", books_dir);
        info!("Logs directory: {:?}", log_dir);

        Ok(Self {
            app_data_dir,
            books_dir,
            ai_settings_path,
            log_dir,
            sidecar_port: 8123,
            health_check_interval_ms: 500,
            health_check_timeout_s: 30,
        })
    }

    /// Get the sidecar URL
    pub fn sidecar_url(&self) -> String {
        format!("http://127.0.0.1:{}/", self.sidecar_port)
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            app_data_dir: PathBuf::new(),
            books_dir: PathBuf::new(),
            ai_settings_path: PathBuf::new(),
            log_dir: PathBuf::new(),
            sidecar_port: 8123,
            health_check_interval_ms: 500,
            health_check_timeout_s: 30,
        }
    }
}
