# Data Model: Tauri App Shell with Python Sidecar

**Feature**: 001-tauri-sidecar-shell
**Date**: 2026-01-17

## Overview

This feature introduces minimal new data structures focused on sidecar lifecycle management. The existing Python data model (`Book`, `ChapterContent`, etc.) remains unchanged and is accessed through the sidecar.

---

## Rust Data Structures

### SidecarState

Represents the lifecycle state of the Python sidecar process.

```rust
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
```

**State Transitions**:
```
[App Launch] → Stopped → Starting → Healthy → [normal operation]
                            ↓
                         Failed → [show error dialog]

[Crash Detected] → Healthy → Restarting → Healthy (success)
                                        → Failed (show manual restart)

[App Shutdown] → * → Stopped
```

---

### StartupError

Categorizes startup failures for user-facing error messages.

```rust
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
                 instances or applications using this port.".to_string()
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
                 Check the logs for more information.".to_string()
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
```

---

### HealthCheckResult

Result of a single health check attempt.

```rust
#[derive(Debug, Clone)]
pub struct HealthCheckResult {
    /// HTTP status code (200 = success)
    pub status_code: Option<u16>,

    /// Response time in milliseconds
    pub latency_ms: u64,

    /// Timestamp of the check
    pub timestamp: std::time::Instant,

    /// Error message if check failed
    pub error: Option<String>,
}

impl HealthCheckResult {
    pub fn is_healthy(&self) -> bool {
        self.status_code == Some(200)
    }
}
```

---

### SidecarManager

Manages sidecar lifecycle and state. Stored in Tauri app state.

```rust
use std::sync::Arc;
use tokio::sync::Mutex;
use std::process::Child;

#[derive(Clone)]
pub struct SidecarManager {
    /// Sidecar child process handle
    pub process: Arc<Mutex<Option<Child>>>,

    /// Current sidecar state
    pub state: Arc<Mutex<SidecarState>>,

    /// Whether auto-restart has been attempted this session
    pub restart_attempted: Arc<Mutex<bool>>,

    /// Path to log directory
    pub log_dir: std::path::PathBuf,

    /// Path to books directory
    pub books_dir: std::path::PathBuf,

    /// Path to AI settings file
    pub ai_settings_path: std::path::PathBuf,
}
```

---

### AppConfig

Application-level configuration derived from Tauri app paths.

```rust
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// ~/Library/Application Support/Reader3/
    pub app_data_dir: std::path::PathBuf,

    /// ~/Library/Application Support/Reader3/books/
    pub books_dir: std::path::PathBuf,

    /// ~/Library/Application Support/Reader3/config/ai_settings.json
    pub ai_settings_path: std::path::PathBuf,

    /// ~/Library/Application Support/Reader3/logs/
    pub log_dir: std::path::PathBuf,

    /// Fixed port for sidecar HTTP server
    pub sidecar_port: u16,  // 8123

    /// Health check configuration
    pub health_check_interval_ms: u64,  // 500
    pub health_check_timeout_s: u64,    // 30
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            app_data_dir: std::path::PathBuf::new(),
            books_dir: std::path::PathBuf::new(),
            ai_settings_path: std::path::PathBuf::new(),
            log_dir: std::path::PathBuf::new(),
            sidecar_port: 8123,
            health_check_interval_ms: 500,
            health_check_timeout_s: 30,
        }
    }
}
```

---

## Environment Variables (Sidecar Contract)

The Tauri app passes these environment variables to the Python sidecar:

| Variable | Description | Example Value |
|----------|-------------|---------------|
| `READER3_BOOKS_DIR` | Path to books storage | `~/Library/Application Support/Reader3/books` |
| `READER3_AI_SETTINGS` | Path to AI config file | `~/Library/Application Support/Reader3/config/ai_settings.json` |
| `READER3_LIBRARY_UI` | Library UI mode | `modern` |

---

## Entity Relationships

```
┌─────────────────┐
│   Tauri App     │
│  (main.rs)      │
└────────┬────────┘
         │ manages
         ▼
┌─────────────────┐      ┌─────────────────┐
│ SidecarManager  │─────▶│  SidecarState   │
│                 │      │  (enum)         │
└────────┬────────┘      └─────────────────┘
         │ spawns/kills
         ▼
┌─────────────────┐      ┌─────────────────┐
│ Child Process   │─────▶│  FastAPI Server │
│ (Python)        │      │  (unchanged)    │
└─────────────────┘      └─────────────────┘
         │ health check
         ▼
┌─────────────────┐
│HealthCheckResult│
└─────────────────┘
```

---

## No Changes to Python Data Model

The existing Python data structures remain unchanged:

- `Book` - EPUB metadata, spine, TOC
- `ChapterContent` - Chapter content and text
- `TOCEntry` - Navigation hierarchy
- `Annotation` - User highlights and notes
- `SearchResult` - Search result with context

These are accessed via the sidecar's HTTP API and are not duplicated in Rust.
