// Error types and user-facing dialogs
// Feature: 001-tauri-sidecar-shell

use tauri::AppHandle;
use tracing::error;

/// Categorizes startup failures for user-facing error messages
#[derive(Debug, Clone, PartialEq)]
pub enum StartupError {
    /// Port 8123 is already in use
    PortInUse,

    /// Failed to spawn Python process (missing python, path issue)
    SidecarSpawnFailed(String),

    /// Sidecar started but health check timed out (30s)
    HealthCheckTimeout,

    /// Sidecar crashed during startup
    SidecarCrashed(String),
}

impl StartupError {
    /// User-facing error title
    pub fn title(&self) -> &str {
        match self {
            Self::PortInUse => "Port Already in Use",
            Self::SidecarSpawnFailed(_) => "Failed to Start Backend",
            Self::HealthCheckTimeout => "Backend Not Responding",
            Self::SidecarCrashed(_) => "Backend Crashed",
        }
    }

    /// User-facing error message with recovery suggestion
    pub fn message(&self) -> String {
        match self {
            Self::PortInUse => {
                "Port 8123 is already in use. Please close any other Reader3 \
                 instances or applications using this port."
                    .to_string()
            }
            Self::SidecarSpawnFailed(detail) => {
                format!(
                    "Could not start the backend server: {}. \
                     Try reinstalling the application or contact support.",
                    detail
                )
            }
            Self::HealthCheckTimeout => {
                "The backend server started but did not become ready within 30 seconds. \
                 Check the logs for more information."
                    .to_string()
            }
            Self::SidecarCrashed(detail) => {
                format!(
                    "The backend server crashed unexpectedly: {}. \
                     Try restarting the application.",
                    detail
                )
            }
        }
    }
}

impl std::fmt::Display for StartupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.title(), self.message())
    }
}

impl std::error::Error for StartupError {}

/// Show a native error dialog to the user
pub fn show_error_dialog(app: &AppHandle, error: StartupError) {
    error!("Showing error dialog: {:?}", error);

    let title = error.title().to_string();
    let message = error.message();

    // Use tauri's dialog API
    tauri::async_runtime::spawn(async move {
        // For now, we'll use a simple approach - the WebView will show an error page
        // In a full implementation, we'd use tauri-plugin-dialog
        eprintln!("ERROR: {} - {}", title, message);
    });
}

/// Open the logs directory in Finder
pub fn open_logs_directory(app: &AppHandle) {
    if let Ok(config) = app.try_state::<crate::config::AppConfig>() {
        let log_dir = config.log_dir.clone();
        std::thread::spawn(move || {
            let _ = std::process::Command::new("open").arg(&log_dir).spawn();
        });
    }
}
