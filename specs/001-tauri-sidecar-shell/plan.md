# Implementation Plan: Tauri App Shell with Python Sidecar

**Branch**: `001-tauri-sidecar-shell` | **Date**: 2026-01-17 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-tauri-sidecar-shell/spec.md`

## Summary

Create a Tauri desktop application shell for macOS 13+ ARM that manages a Python sidecar (FastAPI server). The Tauri app spawns the sidecar on launch, performs health checks, and loads the existing Reader3 Pro web UI in a WebView. This implements Migration Plan Phases 0-1, preserving all Python-based functionality (EPUB processing, AI, semantic search) while wrapping it in a native desktop experience.

## Technical Context

**Language/Version**: Rust (Tauri 2.x) + Python 3.10+ (existing FastAPI sidecar)
**Primary Dependencies**:
- Tauri 2.x (desktop framework)
- tokio (async runtime for Rust)
- reqwest (HTTP client for health checks)
- Existing Python stack: FastAPI, uvicorn, beautifulsoup4, ebooklib, chromadb, sentence-transformers
**Storage**: File-based (existing `books/*_data/` + `config/` under Tauri app data)
**Testing**: Manual testing (no automated test suite per constitution); cargo test for Rust unit tests
**Target Platform**: macOS 13+ ARM64
**Project Type**: Desktop application (Tauri shell + Python sidecar)
**Performance Goals**:
- App window visible in <5 seconds
- Full UI ready in <15 seconds
- Shutdown complete in <5 seconds
**Constraints**:
- Fixed port 8123 for Phase 1
- Development environment (not packaged sidecar)
- Single-user, local-only deployment
**Scale/Scope**: Single user, ~10-100 books, local machine only

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

The Reader3 Pro constitution (v1.0.0) defines 7 core principles. This feature's compliance:

| Principle | Compliance | Notes |
|-----------|------------|-------|
| I. Data Structures First | ✅ PASS | No new data structures; existing `Book`, `ChapterContent` preserved in Python sidecar |
| II. Simplicity Over Engineering | ✅ PASS | Sidecar approach is simplest path; no abstraction layers; direct process management |
| III. File-Based State Management | ✅ PASS | Existing `*_data/` directories preserved; Tauri passes paths via env vars |
| IV. HTML Content Sanitization | ✅ PASS | Handled by Python sidecar; no change |
| V. Path Security | ✅ PASS | Handled by Python sidecar; Tauri provides safe app data paths |
| VI. Explicit Over Implicit | ✅ PASS | Clear state machine for sidecar lifecycle; explicit health check protocol |
| VII. Performance Through Caching | ✅ PASS | No additional caching needed; existing `lru_cache` preserved |

**Architecture Standards Compliance**:
- Two-Step Processing Model: ✅ Preserved (sidecar runs existing reader3.py and server.py)
- Dependency Management: ✅ Rust dependencies minimal (Tauri, tokio, reqwest)

**No violations requiring justification.**

## Project Structure

### Documentation (this feature)

```text
specs/001-tauri-sidecar-shell/
├── spec.md              # Feature specification
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output (lightweight - sidecar state)
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output (health check, error dialogs)
├── diagrams/
│   └── site-diagram.html
├── checklists/
│   └── requirements.md
└── tasks.md             # Phase 2 output (created by /speckit.tasks)
```

### Source Code (repository root)

```text
src-tauri/
├── Cargo.toml           # Rust dependencies
├── tauri.conf.json      # Tauri configuration (CSP, window, sidecar)
├── src/
│   ├── main.rs          # Entry point, app setup, module wiring
│   ├── state.rs         # SidecarState enum (lifecycle states)
│   ├── error.rs         # StartupError enum, user-facing dialogs
│   ├── health.rs        # HealthCheckResult, polling logic
│   ├── config.rs        # AppConfig, path resolution
│   ├── sidecar.rs       # SidecarManager, spawn/shutdown
│   └── logging.rs       # tracing + tracing-oslog + file appender
├── icons/               # App icons
└── capabilities/        # Tauri 2.x capability declarations

src/                     # Frontend loading page (minimal HTML for loading indicator)
├── index.html           # Loading indicator shown during sidecar startup
└── styles.css           # Basic loading UI styles

reader3-pro-python/      # Existing Python codebase (unchanged)
└── ...
```

**Structure Decision**: Tauri 2.x standard layout with `src-tauri/` for Rust code. Minimal frontend in `src/` for loading indicator only. Python sidecar remains in `reader3-pro-python/` unchanged.

## Complexity Tracking

> No constitution violations requiring justification.

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| (none)    | -          | -                                   |

---

## Phase 0: Research

### Research Tasks

1. **Tauri 2.x sidecar management** - Best practices for spawning and managing child processes
2. **Process cleanup on macOS** - Ensuring no orphan processes (SIGTERM → SIGKILL escalation)
3. **Single instance enforcement** - Tauri plugin or Rust implementation
4. **macOS unified logging** - How to integrate with `os_log` from Rust
5. **CSP configuration** - Allowing local HTTP + blob: + external network

### Research Output

See [research.md](./research.md) for detailed findings.

---

## Phase 1: Design & Contracts

### Data Model

See [data-model.md](./data-model.md) for:
- `SidecarState` enum (Stopped, Starting, Healthy, Failed, Restarting)
- `HealthCheckResult` struct
- `StartupError` enum (PortInUse, SidecarSpawnFailed, HealthCheckTimeout)

### Contracts

See [contracts/](./contracts/) for:
- Health check endpoint contract (`GET /` → 200 OK)
- Error dialog specifications (port conflict, health timeout, crash recovery)
- Environment variable contract (READER3_BOOKS_DIR, READER3_AI_SETTINGS)

### Quickstart

See [quickstart.md](./quickstart.md) for:
- Development setup (Rust toolchain, Tauri CLI, Python venv)
- Build and run commands
- Testing procedures
