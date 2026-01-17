# Feature Specification: Tauri App Shell with Python Sidecar

**Feature Branch**: `001-tauri-sidecar-shell`
**Created**: 2026-01-17
**Status**: Draft
**Input**: User description: "根据migration_plan.md 在specs/ folder创建第一个feature，并创建详细的spec.md。"

## Clarifications

### Session 2026-01-17

- Q: Where should sidecar startup logs be written for troubleshooting? → A: Both log file in app data directory (`~/Library/Application Support/Reader3/logs/`) and macOS unified logging (Console.app)
- Q: Should the app auto-restart the sidecar on mid-session crash or require user action? → A: Auto-restart once, then manual - attempt one automatic restart; if it fails again, show manual restart option

## Overview

This feature establishes the foundational Tauri desktop application shell that launches and manages a Python sidecar process. The Python sidecar runs the existing FastAPI server (Reader3 Pro), and the Tauri WebView loads the server's pages. This is the critical first step in migrating Reader3 Pro from a web application to a native macOS desktop app.

The feature covers:
- Tauri project scaffolding for macOS 13+ ARM
- Rust-side sidecar process management (spawn, health check, shutdown)
- WebView configuration to load the Python server
- CSP and security configuration for local HTTP + external network access

## Visual Design

- **Site Diagram**: [site-diagram.html](./diagrams/site-diagram.html) - 应用架构和数据流图

*Note: This feature is infrastructure-focused with no new UI pages. The existing Reader3 Pro UI is loaded unchanged in the WebView. Wireframe and user-flow diagrams are not applicable.*

## User Scenarios & Testing *(mandatory)*

### User Story 1 - App Launch with Automatic Backend Start (Priority: P1)

As a user, I want to double-click the Reader3 app icon and have the application fully ready to use, without manually starting any servers or running terminal commands.

**Why this priority**: This is the core value proposition - turning a command-line web app into a native desktop app experience. Without this, the app is unusable.

**Independent Test**: Can be fully tested by launching the app from Finder and verifying the library page loads. Delivers the fundamental "it just works" experience.

**Acceptance Scenarios**:

1. **Given** the app is installed on macOS 13+ ARM, **When** user double-clicks the app icon, **Then** the app window appears within 5 seconds showing a loading indicator
2. **Given** the app window is displayed with loading indicator, **When** the Python server becomes ready (health check passes), **Then** the library page loads automatically in the WebView
3. **Given** the user launches the app, **When** all startup completes, **Then** total time from click to usable library page is under 15 seconds

---

### User Story 2 - Graceful Shutdown (Priority: P1)

As a user, I want to close the app and have all background processes terminate cleanly, so my system resources are freed and no orphan processes remain.

**Why this priority**: Equal to P1 because orphan processes would degrade user trust and system performance. This is table-stakes for a desktop app.

**Independent Test**: Can be tested by launching the app, closing it, and verifying no python/uvicorn processes remain running via Activity Monitor.

**Acceptance Scenarios**:

1. **Given** the app is running with the Python sidecar active, **When** user closes the app window (Cmd+Q or red close button), **Then** the Python sidecar process terminates within 3 seconds
2. **Given** the app is running, **When** user force-quits via Activity Monitor, **Then** the sidecar process is also terminated (not orphaned)
3. **Given** the app is running, **When** user closes the app, **Then** no python, uvicorn, or reader3-server processes remain in Activity Monitor

---

### User Story 3 - Startup Failure Recovery (Priority: P2)

As a user, if something goes wrong during app startup (port conflict, missing dependencies), I want to see a clear error message explaining the problem and suggesting next steps.

**Why this priority**: Graceful error handling improves user experience and reduces support burden, but is secondary to the happy path working.

**Independent Test**: Can be tested by blocking port 8123 before launch and verifying the error dialog appears with helpful guidance.

**Acceptance Scenarios**:

1. **Given** port 8123 is already in use by another application, **When** user launches the app, **Then** an error dialog appears stating the port is busy and suggesting the user close the conflicting application
2. **Given** the Python sidecar fails to start (missing python, corrupted installation), **When** user launches the app, **Then** an error dialog appears with the specific error and a suggestion to reinstall or contact support
3. **Given** the sidecar starts but health check fails for 30 seconds, **When** timeout is reached, **Then** an error dialog appears stating the server failed to become ready and offering to view logs

---

### User Story 4 - Network Access for AI Features (Priority: P3)

As a user, I want my AI assistant features to continue working in the desktop app, which requires outbound network access to AI provider APIs.

**Why this priority**: AI features are valuable but not blocking for basic EPUB reading functionality. Can be validated after core launch/shutdown works.

**Independent Test**: Can be tested by opening a book, clicking AI panel, and sending a question to an external AI provider (requires API key configured).

**Acceptance Scenarios**:

1. **Given** the app is running and user has configured an AI provider API key, **When** user asks a question in the AI panel, **Then** the AI response streams back successfully
2. **Given** the app is running, **When** the AI panel makes requests to external APIs (Anthropic, OpenAI, Gemini, etc.), **Then** network requests are not blocked by CSP or sandbox restrictions

---

### Edge Cases

- What happens when the user launches a second instance of the app while one is already running?
  - Display a dialog: "Reader3 is already running" and focus the existing window
- How does the system handle the sidecar crashing mid-session?
  - Attempt one automatic restart silently (with brief "Reconnecting..." indicator)
  - If auto-restart succeeds, resume normal operation
  - If auto-restart fails, display error overlay: "Backend connection lost. Click to restart." with manual restart button
- What if the default port 8123 is configured differently in user settings?
  - For Phase 1, use fixed port 8123. Dynamic port support deferred to a future enhancement.
- How does the app behave on macOS versions below 13?
  - App installation is blocked with a clear minimum OS version error from the installer

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST create a valid Tauri project structure targeting macOS 13+ ARM64
- **FR-002**: System MUST spawn the Python sidecar (FastAPI server) on app launch as a child process
- **FR-003**: System MUST perform health checks by polling `GET http://127.0.0.1:8123/` until 200 response
- **FR-004**: System MUST display a loading indicator while waiting for the sidecar to become ready
- **FR-005**: System MUST load the library page (`http://127.0.0.1:8123/`) in the WebView after health check passes
- **FR-006**: System MUST terminate the sidecar process when the app is closed (SIGTERM, then SIGKILL after timeout)
- **FR-007**: System MUST configure CSP to allow `http://127.0.0.1:8123`, `blob:`, `tauri://`, and Google Fonts
- **FR-008**: System MUST allow outbound network access for AI provider APIs (no network sandbox)
- **FR-009**: System MUST display user-friendly error dialogs when startup fails (port conflict, health check timeout, sidecar crash)
- **FR-010**: System MUST prevent multiple app instances from running simultaneously
- **FR-011**: System MUST pass environment variables to sidecar for data paths (READER3_BOOKS_DIR, READER3_AI_SETTINGS)
- **FR-012**: System MUST write sidecar logs to both `~/Library/Application Support/Reader3/logs/` and macOS unified logging (accessible via Console.app)
- **FR-013**: System MUST attempt one automatic sidecar restart on mid-session crash before requiring user intervention

### Key Entities

- **Sidecar Process**: The Python FastAPI server running as a child process of the Tauri app. Attributes: process ID, port (8123), health status, startup time.
- **Health Check**: Polling mechanism to verify sidecar readiness. Attributes: endpoint URL, poll interval (500ms), timeout (30s), status (pending/healthy/failed).
- **App Window**: The Tauri WebView container. Attributes: URL (loaded after health check), CSP configuration, size, position.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: App launches and displays the library page within 15 seconds on a MacBook Air M1 with a fresh start
- **SC-002**: App shutdown completes (window closed, sidecar terminated) within 5 seconds
- **SC-003**: No orphan python/uvicorn processes remain after app closure in 100% of test cases
- **SC-004**: Port conflict errors are detected and displayed to user within 3 seconds of launch attempt
- **SC-005**: Health check failure (server not responding) is detected and error displayed within 35 seconds
- **SC-006**: AI features (sending questions to external providers) work identically to the web version
- **SC-007**: App successfully blocks second instance launch in 100% of test cases

## Assumptions

- Python 3.10+ is bundled with the sidecar or available on the user's system (sidecar packaging is Phase 7, out of scope)
- For Phase 1, the sidecar runs from the development Python environment (`uv run reader3-server`)
- Fixed port 8123 is used; dynamic port assignment is a future enhancement
- The existing `reader3-pro-python` codebase requires no modifications to work as a sidecar
- macOS 13+ ARM is the only target platform for this phase

## Out of Scope

- Sidecar binary packaging (PyInstaller/embedded Python) - deferred to Phase 7
- Dynamic port assignment - deferred to future enhancement
- Windows/Linux support - macOS only for initial release
- Auto-update mechanism
- Code signing and notarization (handled separately)
- App Store distribution
