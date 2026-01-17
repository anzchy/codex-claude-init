// Sidecar process management (spawn, health check, shutdown)
// Feature: 001-tauri-sidecar-shell

use crate::config::AppConfig;
use crate::error::StartupError;
use crate::health::{is_port_available, poll_health_check, HealthCheckConfig};
use crate::state::SidecarState;
use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;
use std::process::Child;
use std::sync::Arc;
use std::time::Duration;
use tauri::{AppHandle, Manager};
use tokio::sync::Mutex;
use tracing::{error, info, warn};

/// Manages sidecar lifecycle and state
pub struct SidecarManager {
    /// Sidecar child process handle
    pub process: Arc<Mutex<Option<Child>>>,

    /// Current sidecar state
    pub state: Arc<Mutex<SidecarState>>,

    /// Whether auto-restart has been attempted this session
    pub restart_attempted: Arc<Mutex<bool>>,
}

impl Default for SidecarManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SidecarManager {
    pub fn new() -> Self {
        Self {
            process: Arc::new(Mutex::new(None)),
            state: Arc::new(Mutex::new(SidecarState::Stopped)),
            restart_attempted: Arc::new(Mutex::new(false)),
        }
    }
}

/// Start the sidecar and wait for it to become healthy
pub async fn start_sidecar_and_wait(app: &AppHandle) -> Result<(), StartupError> {
    let config = app
        .try_state::<AppConfig>()
        .ok_or_else(|| StartupError::SidecarSpawnFailed("AppConfig not found".to_string()))?;

    // Check if port is available
    if !is_port_available(config.sidecar_port) {
        error!("Port {} is already in use", config.sidecar_port);
        return Err(StartupError::PortInUse);
    }

    // Initialize sidecar manager if not exists
    if app.try_state::<SidecarManager>().is_none() {
        app.manage(SidecarManager::new());
    }

    let manager = app.state::<SidecarManager>();
    {
        let mut state = manager.state.lock().await;
        *state = SidecarState::Starting;
    }

    // Spawn the sidecar process
    info!("Spawning Python sidecar...");

    // Get the path to the Python project
    let current_dir = std::env::current_dir().map_err(|e| {
        StartupError::SidecarSpawnFailed(format!("Failed to get current directory: {}", e))
    })?;

    // Look for reader3-pro-python directory
    let python_dir = current_dir.join("reader3-pro-python");
    if !python_dir.exists() {
        return Err(StartupError::SidecarSpawnFailed(format!(
            "Python sidecar directory not found: {:?}",
            python_dir
        )));
    }

    // Spawn the process
    let child = std::process::Command::new("uv")
        .args(["run", "reader3-server"])
        .current_dir(&python_dir)
        .env("READER3_BOOKS_DIR", config.books_dir.to_string_lossy().to_string())
        .env("READER3_AI_SETTINGS", config.ai_settings_path.to_string_lossy().to_string())
        .env("READER3_LIBRARY_UI", "modern")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| StartupError::SidecarSpawnFailed(format!("Failed to spawn: {}", e)))?;

    info!("Sidecar spawned with PID: {}", child.id());

    // Store the child process
    {
        let mut process = manager.process.lock().await;
        *process = Some(child);
    }

    // Poll health check
    let health_config = HealthCheckConfig::default();
    match poll_health_check(&health_config).await {
        Ok(()) => {
            let mut state = manager.state.lock().await;
            *state = SidecarState::Healthy;
            info!("Sidecar is healthy");
            Ok(())
        }
        Err(e) => {
            let mut state = manager.state.lock().await;
            *state = SidecarState::Failed(e.clone());
            error!("Health check failed: {:?}", e);

            // Kill the sidecar if it's still running
            shutdown_sidecar(app).await;

            Err(e)
        }
    }
}

/// Shutdown the sidecar process gracefully
pub async fn shutdown_sidecar(app: &AppHandle) {
    let Some(manager) = app.try_state::<SidecarManager>() else {
        return;
    };

    let mut process_guard = manager.process.lock().await;
    let Some(ref mut child) = *process_guard else {
        info!("No sidecar process to shutdown");
        return;
    };

    let pid = child.id();
    info!("Shutting down sidecar (PID: {})", pid);

    // Step 1: Send SIGTERM (graceful)
    let nix_pid = Pid::from_raw(pid as i32);
    if let Err(e) = kill(nix_pid, Signal::SIGTERM) {
        warn!("Failed to send SIGTERM: {}", e);
    } else {
        info!("Sent SIGTERM to sidecar");
    }

    // Step 2: Wait up to 2 seconds for graceful shutdown
    for _ in 0..20 {
        tokio::time::sleep(Duration::from_millis(100)).await;
        if let Ok(Some(_)) = child.try_wait() {
            info!("Sidecar exited gracefully");
            *process_guard = None;
            return;
        }
    }

    // Step 3: Send SIGKILL if still running
    warn!("Sidecar did not exit gracefully, sending SIGKILL");
    if let Err(e) = kill(nix_pid, Signal::SIGKILL) {
        error!("Failed to send SIGKILL: {}", e);
    }

    // Wait a bit more for SIGKILL to take effect
    tokio::time::sleep(Duration::from_millis(500)).await;

    if let Ok(Some(status)) = child.try_wait() {
        info!("Sidecar killed with status: {:?}", status);
    }

    *process_guard = None;

    // Update state
    let mut state = manager.state.lock().await;
    *state = SidecarState::Stopped;
}

/// Restart the sidecar (manual restart after auto-restart failed)
pub async fn restart_sidecar(app: &AppHandle) -> Result<(), StartupError> {
    info!("Restarting sidecar...");

    // Shutdown existing sidecar
    shutdown_sidecar(app).await;

    // Small delay before restart
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Start again
    start_sidecar_and_wait(app).await
}
