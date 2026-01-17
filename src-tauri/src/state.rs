// Sidecar lifecycle state management
// Feature: 001-tauri-sidecar-shell

use crate::error::StartupError;

/// Represents the lifecycle state of the Python sidecar process
#[derive(Debug, Clone, PartialEq)]
pub enum SidecarState {
    /// Sidecar process not running
    Stopped,

    /// Sidecar spawned, waiting for health check
    Starting,

    /// Health check passed, WebView can load
    Healthy,

    /// Startup or runtime failure
    Failed(StartupError),

    /// Automatic restart in progress (after crash)
    Restarting,
}

impl Default for SidecarState {
    fn default() -> Self {
        Self::Stopped
    }
}

impl std::fmt::Display for SidecarState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Stopped => write!(f, "Stopped"),
            Self::Starting => write!(f, "Starting"),
            Self::Healthy => write!(f, "Healthy"),
            Self::Failed(e) => write!(f, "Failed: {}", e.title()),
            Self::Restarting => write!(f, "Restarting"),
        }
    }
}
