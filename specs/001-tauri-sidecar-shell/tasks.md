# Tasks: Tauri App Shell with Python Sidecar

**Input**: Design documents from `/specs/001-tauri-sidecar-shell/`
**Prerequisites**: plan.md ✅, spec.md ✅, research.md ✅, data-model.md ✅, contracts/ ✅

**Tests**: Not explicitly requested in spec. Manual testing procedures defined in quickstart.md.

**Organization**: Tasks grouped by user story for independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (US1, US2, US3, US4)
- Exact file paths included in descriptions

## Path Conventions

- **Tauri Rust code**: `src-tauri/src/`
- **Tauri config**: `src-tauri/`
- **Frontend loading page**: `src/`
- **Python sidecar**: `reader3-pro-python/` (unchanged)

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Tauri project initialization and basic structure

- [ ] T001 Initialize Tauri 2.x project with `cargo tauri init` in src-tauri/
- [ ] T002 Configure Cargo.toml with dependencies from research.md in src-tauri/Cargo.toml
- [ ] T003 [P] Create Tauri capabilities configuration in src-tauri/capabilities/default.json
- [ ] T004 [P] Configure tauri.conf.json with window settings (1200x800, centered) in src-tauri/tauri.conf.json
- [ ] T005 [P] Add app icons to src-tauri/icons/ (placeholder or actual icons)

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T006 Define SidecarState enum per data-model.md in src-tauri/src/state.rs
- [ ] T007 [P] Define StartupError enum with user-facing messages in src-tauri/src/error.rs
- [ ] T008 [P] Define HealthCheckResult struct in src-tauri/src/health.rs
- [ ] T009 Define AppConfig struct with path resolution in src-tauri/src/config.rs
- [ ] T010 Create SidecarManager struct skeleton in src-tauri/src/sidecar.rs
- [ ] T011 Setup tracing + tracing-oslog + tracing-appender logging in src-tauri/src/logging.rs
  - Configure tracing-oslog for macOS Console.app integration
  - Configure tracing-appender with daily rolling to ~/Library/Application Support/Reader3/logs/
  - Both outputs per FR-012 (dual logging requirement)
- [ ] T012 Configure CSP in tauri.conf.json per research.md (allow 127.0.0.1:8123, blob:, AI APIs)
- [ ] T013 Wire up module declarations in src-tauri/src/main.rs (mod state, error, health, config, sidecar, logging)

**Checkpoint**: Foundation ready - user story implementation can now begin

---

## Phase 3: User Story 1 - App Launch with Automatic Backend Start (Priority: P1) 🎯 MVP

**Goal**: Double-click app icon → app window appears → loading indicator → library page loads automatically

**Independent Test**: Launch app from Finder, verify library page loads within 15 seconds

### Implementation for User Story 1

- [ ] T014 [US1] Create loading indicator HTML page in src/index.html
- [ ] T015 [P] [US1] Create loading indicator styles in src/styles.css
- [ ] T016 [US1] Implement spawn_sidecar() function in src-tauri/src/sidecar.rs
  - Spawn `uv run reader3-server` with environment variables per contracts/environment-variables.yaml
  - Set READER3_BOOKS_DIR, READER3_AI_SETTINGS, READER3_LIBRARY_UI
- [ ] T017 [US1] Implement poll_health_check() in src-tauri/src/health.rs
  - 500ms interval, 30s timeout, 3s per-request timeout
  - Poll GET http://127.0.0.1:8123/ until 200 response
- [ ] T018 [US1] Implement startup orchestration in src-tauri/src/main.rs
  - Show loading page first
  - Spawn sidecar, poll health check
  - Navigate WebView to http://127.0.0.1:8123/ on success
- [ ] T019 [US1] Add startup logging (sidecar spawned, health check attempts, ready state)
- [ ] T020 [US1] Configure window to load src/index.html initially in tauri.conf.json

**Checkpoint**: App launches, shows loading indicator, then library page loads (happy path works)

---

## Phase 4: User Story 2 - Graceful Shutdown (Priority: P1)

**Goal**: Close app → all background processes terminate cleanly within 3 seconds

**Independent Test**: Launch app, close with Cmd+Q, verify no orphan python/uvicorn processes in Activity Monitor

### Implementation for User Story 2

- [ ] T021 [US2] Implement shutdown_sidecar() in src-tauri/src/sidecar.rs
  - SIGTERM → 2 second wait → SIGKILL escalation per research.md
  - Use nix crate for signal handling
- [ ] T022 [US2] Register on_window_event handler in src-tauri/src/main.rs
  - Intercept CloseRequested event
  - Call shutdown_sidecar() before allowing window close
- [ ] T023 [US2] Add shutdown logging (SIGTERM sent, process exited/killed)
- [ ] T024 [US2] Handle force-quit scenario (ensure sidecar tracked for cleanup)

**Checkpoint**: App closes cleanly, no orphan processes remain

---

## Phase 5: User Story 3 - Startup Failure Recovery (Priority: P2)

**Goal**: Show clear error dialogs when startup fails (port conflict, spawn failure, health timeout)

**Independent Test**: Block port 8123 before launch, verify error dialog appears within 3 seconds

### Implementation for User Story 3

- [ ] T025 [US3] Implement port availability check before spawn in src-tauri/src/sidecar.rs
  - Detect if port 8123 is already in use
  - Return StartupError::PortInUse
- [ ] T026 [US3] Create show_error_dialog() function in src-tauri/src/error.rs
  - Use tauri::api::dialog for native macOS alerts
  - Map StartupError variants to title/message per contracts/error-dialogs.yaml
- [ ] T027 [US3] Handle sidecar spawn failure in src-tauri/src/sidecar.rs
  - Catch spawn errors, return StartupError::SidecarSpawnFailed
- [ ] T028 [US3] Handle health check timeout in src-tauri/src/health.rs
  - After 30s, return StartupError::HealthCheckTimeout
- [ ] T029 [US3] Handle sidecar crash during startup in src-tauri/src/sidecar.rs
  - Monitor process, detect unexpected exit
  - Return StartupError::SidecarCrashed
- [ ] T030 [US3] Wire error dialogs into startup flow in src-tauri/src/main.rs
  - On any StartupError, show dialog then exit gracefully
- [ ] T031 [US3] Add "View Logs" button functionality to error dialogs
  - Open ~/Library/Application Support/Reader3/logs/ in Finder

**Checkpoint**: All startup failures show helpful error dialogs

---

## Phase 6: User Story 4 - Network Access for AI Features (Priority: P3)

**Goal**: AI features work - outbound network access to AI provider APIs not blocked

**Independent Test**: Open book, use AI panel, send question to external provider

### Implementation for User Story 4

- [ ] T032 [US4] Verify CSP connect-src includes all AI providers in src-tauri/tauri.conf.json (verification only - T012 sets base CSP)
  - Confirm: api.anthropic.com, api.openai.com, generativelanguage.googleapis.com
  - Confirm: openrouter.ai, localhost:11434 (Ollama)
- [ ] T033 [US4] Disable network sandbox restrictions in Tauri capabilities
  - Ensure no capability blocks outbound HTTPS
- [ ] T034 [US4] Test AI panel communication end-to-end (manual verification)

**Checkpoint**: AI features work identically to web version

---

## Phase 7: Edge Cases & Single Instance

**Purpose**: Handle edge cases from spec.md

- [ ] T035 Add tauri-plugin-single-instance to Cargo.toml in src-tauri/Cargo.toml
- [ ] T036 Configure single instance plugin in src-tauri/src/main.rs
  - Show "Reader3 Already Running" dialog
  - Focus existing window on second launch attempt
- [ ] T037 Implement auto-restart on mid-session crash in src-tauri/src/sidecar.rs
  - Detect sidecar process exit during runtime
  - Attempt one automatic restart
  - Show "Reconnecting..." indicator in WebView
- [ ] T038 Implement manual restart overlay in src/index.html
  - Show "Connection Lost. Click to restart." after failed auto-restart
  - Button triggers restart attempt via Tauri command
- [ ] T039 [P] Add Tauri command for manual sidecar restart in src-tauri/src/main.rs

**Checkpoint**: All edge cases handled per spec.md

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Final cleanup and validation

- [ ] T040 [P] Create app data directories on first launch in src-tauri/src/config.rs
  - ~/Library/Application Support/Reader3/books/
  - ~/Library/Application Support/Reader3/config/
  - ~/Library/Application Support/Reader3/logs/
- [ ] T041 [P] Configure log rotation (daily) in src-tauri/src/logging.rs
- [ ] T042 Run full quickstart.md validation (all test cases TC-001 through TC-005)
- [ ] T043 Verify success criteria from spec.md
  - SC-001: Launch < 15 seconds
  - SC-002: Shutdown < 5 seconds
  - SC-003: No orphan processes
  - SC-004: Port conflict detected < 3 seconds
  - SC-005: Health timeout < 35 seconds
  - SC-006: AI features work
  - SC-007: Second instance blocked
- [ ] T044 Code cleanup and documentation comments in src-tauri/src/

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Story 1 (Phase 3)**: Depends on Foundational - MVP milestone
- **User Story 2 (Phase 4)**: Depends on Foundational - Can start after/parallel with US1
- **User Story 3 (Phase 5)**: Depends on US1 (needs startup flow to add error handling)
- **User Story 4 (Phase 6)**: Depends on Foundational only - Can parallel with US1
- **Edge Cases (Phase 7)**: Depends on US1, US2 (needs basic launch/shutdown working)
- **Polish (Phase 8)**: Depends on all user stories complete

### User Story Dependencies

```
Phase 1 (Setup)
    │
    ▼
Phase 2 (Foundational)
    │
    ├─────────────┬─────────────┬─────────────┐
    ▼             ▼             ▼             ▼
Phase 3 (US1)  Phase 4 (US2)  Phase 6 (US4)  (parallel possible)
    │             │
    └──────┬──────┘
           ▼
    Phase 5 (US3) - needs startup flow from US1
           │
           ▼
    Phase 7 (Edge Cases) - needs US1 + US2
           │
           ▼
    Phase 8 (Polish)
```

### Within Each User Story

- Foundation modules must exist before implementation
- State/error types before sidecar management
- Sidecar spawn before health check
- Health check before WebView navigation
- Error handling after happy path works

### Parallel Opportunities

**Setup Phase (all [P] tasks):**
```
T003: capabilities/default.json
T004: tauri.conf.json
T005: icons/
```

**Foundational Phase:**
```
T007: error.rs     ──┐
T008: health.rs    ──┼── Can run in parallel (different files)
T011: logging.rs   ──┘
```

**User Story 1:**
```
T014: src/index.html  ──┐
T015: src/styles.css  ──┴── Can run in parallel
```

**Cross-Story Parallelism:**
Once Phase 2 completes, US1, US2, and US4 can start simultaneously if team capacity allows.

---

## Parallel Example: Phase 2 Foundational

```bash
# These can run in parallel (different files):
Task T007: "Define StartupError enum in src-tauri/src/error.rs"
Task T008: "Define HealthCheckResult struct in src-tauri/src/health.rs"
Task T011: "Setup tracing logging infrastructure in src-tauri/src/logging.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (T001-T005)
2. Complete Phase 2: Foundational (T006-T013)
3. Complete Phase 3: User Story 1 (T014-T020)
4. **STOP and VALIDATE**: Run TC-001 from quickstart.md
5. App launches and shows library page = MVP complete!

### Incremental Delivery

1. Setup + Foundational → Foundation ready
2. Add User Story 1 → Test launch → **MVP!**
3. Add User Story 2 → Test shutdown → No orphan processes
4. Add User Story 3 → Test error handling → Robust startup
5. Add User Story 4 → Test AI → Full feature parity
6. Add Edge Cases → Production-ready

### Suggested MVP Scope

**User Story 1 only** = Tasks T001-T020 (20 tasks)

This delivers the core value: "Double-click app, library loads automatically"

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story
- Each user story is independently testable
- Manual testing via quickstart.md test cases
- Commit after each task or logical group
- Stop at any checkpoint to validate story
- Python sidecar (reader3-pro-python/) requires NO modifications
