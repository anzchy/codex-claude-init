# Research: Tauri App Shell with Python Sidecar

**Feature**: 001-tauri-sidecar-shell
**Date**: 2026-01-17

## 1. Tauri 2.x Sidecar Management

### Decision: Use `tauri::api::process::Command` for sidecar spawning

**Rationale**: Tauri's built-in process API provides automatic cleanup when the app drops, async output streaming, and cross-platform compatibility.

**Alternatives Considered**:
- `std::process::Command` - Lower level, no automatic cleanup on app exit
- External process manager crate - Unnecessary complexity

### Spawning Pattern

```rust
use tauri::api::process::Command;

let (mut rx, child) = Command::new("python3")
    .args(&["-m", "uvicorn", "reader3_app.web.server:app", "--port", "8123"])
    .env("READER3_BOOKS_DIR", books_path)
    .env("READER3_AI_SETTINGS", ai_settings_path)
    .spawn()
    .expect("Failed to spawn sidecar");
```

**Key Feature**: Tauri automatically kills child processes when `tauri::App` drops.

---

## 2. Process Cleanup on macOS

### Decision: SIGTERM → 2 seconds → SIGKILL escalation

**Rationale**: Industry standard pattern. Allows FastAPI to close connections gracefully while guaranteeing no orphan processes.

**Implementation**:

```rust
use nix::signal::{kill, Signal};
use nix::unistd::Pid;

pub async fn shutdown_sidecar(child: &mut Child) {
    let pid = Pid::from_raw(child.id() as i32);

    // Step 1: SIGTERM (graceful)
    let _ = kill(pid, Signal::SIGTERM);

    // Step 2: Wait 2 seconds
    sleep(Duration::from_secs(2)).await;

    // Step 3: SIGKILL if still running
    if child.try_wait().ok().flatten().is_none() {
        let _ = kill(pid, Signal::SIGKILL);
        sleep(Duration::from_millis(500)).await;
    }
}
```

**Dependencies**:
```toml
nix = { version = "0.27", features = ["signal"] }
```

---

## 3. Single Instance Enforcement

### Decision: Use `tauri-plugin-single-instance`

**Rationale**: Official Tauri plugin, handles all platforms uniformly, supports deep linking, and provides callback for focusing existing window.

**Alternatives Considered**:
- Manual port check (8123 in use = already running) - Less reliable, doesn't handle window focusing
- Lock file - macOS file locking semantics are complex

**Implementation**:

```rust
use tauri_plugin_single_instance::SingleInstance;

fn main() {
    tauri::Builder::default()
        .plugin(
            SingleInstance::new("com.reader3app", |app, _argv, _cwd| {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.set_focus();
                    let _ = window.unminimize();
                }
            })
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

**Dependencies**:
```toml
tauri-plugin-single-instance = "2.0"
```

---

## 4. macOS Unified Logging

### Decision: Use `tracing` crate with os_log backend + file output

**Rationale**:
- `tracing` is the Rust ecosystem standard for structured logging
- `tracing-oslog` provides macOS Console.app integration
- File logging provides persistent logs for user troubleshooting

**Implementation**:

```rust
use tracing_subscriber::prelude::*;

fn setup_logging(log_dir: &Path) {
    let file_appender = tracing_appender::rolling::daily(log_dir, "reader3.log");

    tracing_subscriber::registry()
        .with(tracing_oslog::OsLogger::new("com.reader3app", "default"))
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(file_appender)
                .with_ansi(false)
        )
        .init();
}
```

**Dependencies**:
```toml
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
tracing-appender = "0.2"
tracing-oslog = "0.1"
```

**Log Location**: `~/Library/Application Support/Reader3/logs/`

---

## 5. CSP Configuration

### Decision: Permissive but scoped CSP for desktop app context

**Rationale**:
- Desktop app with trusted backend = different threat model than web
- Must allow local HTTP (sidecar), blob: (epub.js), and external AI APIs
- Google Fonts required for typography

**Configuration** (`tauri.conf.json`):

```json
{
  "security": {
    "csp": {
      "default-src": ["'self'"],
      "connect-src": [
        "'self'",
        "http://127.0.0.1:8123",
        "https://api.anthropic.com",
        "https://api.openai.com",
        "https://generativelanguage.googleapis.com",
        "https://openrouter.ai",
        "http://localhost:11434"
      ],
      "img-src": ["'self'", "asset:", "http://127.0.0.1:8123", "blob:", "data:"],
      "style-src": ["'self'", "'unsafe-inline'", "https://fonts.googleapis.com"],
      "font-src": ["'self'", "https://fonts.gstatic.com"],
      "frame-src": ["'self'", "http://127.0.0.1:8123"]
    }
  }
}
```

**Why `unsafe-inline` is acceptable**:
- Single-user desktop app (no untrusted user input)
- Content from trusted Python backend with sanitization
- No third-party scripts

---

## 6. Health Check Polling

### Decision: 500ms interval, 30s timeout, 3s per-request timeout

**Rationale**: Matches spec requirements. Responsive enough for good UX, long enough for slow Python startup.

**Implementation**:

```rust
pub async fn poll_health_check() -> Result<(), StartupError> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(3))
        .build()?;

    let max_attempts = 60; // 30s / 500ms

    for attempt in 1..=max_attempts {
        match client.get("http://127.0.0.1:8123/").send().await {
            Ok(resp) if resp.status() == 200 => return Ok(()),
            _ => {
                if attempt < max_attempts {
                    sleep(Duration::from_millis(500)).await;
                }
            }
        }
    }

    Err(StartupError::HealthCheckTimeout)
}
```

---

## 7. Auto-Restart on Crash

### Decision: One automatic restart, then manual

**Rationale**: Per spec clarification. Recovers from transient failures without masking persistent issues.

**State Machine**:
```
Healthy → (crash detected) → Restarting → (restart succeeds) → Healthy
                                        → (restart fails) → Failed (show manual button)
```

---

## Dependencies Summary

```toml
[dependencies]
tauri = { version = "2", features = ["macos-private-api"] }
tauri-plugin-single-instance = "2.0"
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.12", features = ["json"] }
nix = { version = "0.27", features = ["signal"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
tracing-appender = "0.2"
tracing-oslog = "0.1"
```

---

## All Research Tasks Resolved

| Task | Status | Decision |
|------|--------|----------|
| Sidecar management | ✅ Resolved | `tauri::api::process::Command` with auto-cleanup |
| Process cleanup | ✅ Resolved | SIGTERM → 2s → SIGKILL via `nix` crate |
| Single instance | ✅ Resolved | `tauri-plugin-single-instance` |
| macOS logging | ✅ Resolved | `tracing` + `tracing-oslog` + file appender |
| CSP configuration | ✅ Resolved | Scoped CSP allowing local HTTP + AI providers |
