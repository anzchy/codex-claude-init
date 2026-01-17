# Quickstart Guide: Tauri App Shell with Python Sidecar

**Feature**: 001-tauri-sidecar-shell
**Date**: 2026-01-17

## Prerequisites

### System Requirements

- macOS 13+ (Ventura or later)
- Apple Silicon (ARM64) Mac
- 8GB RAM minimum
- 2GB free disk space

### Development Tools

1. **Rust Toolchain** (via rustup)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   rustc --version  # Should be 1.70+
   ```

2. **Tauri CLI**
   ```bash
   cargo install tauri-cli
   cargo tauri --version  # Should be 2.x
   ```

3. **Python 3.10+** (via uv or pyenv)
   ```bash
   # Using uv (recommended)
   curl -LsSf https://astral.sh/uv/install.sh | sh
   uv python install 3.11

   # Verify
   python3 --version  # Should be 3.10+
   ```

4. **Node.js 18+** (for Tauri frontend build)
   ```bash
   # Using nvm
   nvm install 18
   nvm use 18
   node --version  # Should be 18+
   ```

---

## Project Setup

### 1. Clone and Navigate

```bash
cd /Users/jackcheng/Documents/01_Coding/mac-app/reader-tauri
```

### 2. Python Sidecar Setup

```bash
cd reader3-pro-python

# Create virtual environment and install dependencies
uv sync

# Verify sidecar runs standalone
uv run reader3-server
# Visit http://127.0.0.1:8123 - should show library page
# Press Ctrl+C to stop
```

### 3. Tauri App Setup

```bash
cd ../src-tauri

# Install Rust dependencies
cargo build

# Return to project root
cd ..
```

---

## Development Workflow

### Running the App

```bash
# From project root
cargo tauri dev
```

This will:
1. Build the Rust Tauri app
2. Spawn the Python sidecar automatically
3. Open the app window with the library page

### Building for Release

```bash
cargo tauri build
```

Output: `src-tauri/target/release/bundle/macos/Reader3.app`

---

## Testing Procedures

### Manual Test Checklist

#### TC-001: App Launch (Happy Path)
1. Run `cargo tauri dev`
2. ✓ App window appears within 5 seconds with loading indicator
3. ✓ Library page loads within 15 seconds total
4. ✓ Existing books are displayed (if any)

#### TC-002: Graceful Shutdown
1. With app running, press Cmd+Q
2. ✓ App window closes
3. ✓ Open Activity Monitor, search for "python" or "uvicorn"
4. ✓ No orphan processes remain

#### TC-003: Port Conflict Detection
1. In terminal: `python3 -m http.server 8123`
2. Run `cargo tauri dev`
3. ✓ Error dialog appears: "Port Already in Use"
4. Stop the http.server (Ctrl+C)

#### TC-004: Second Instance Prevention
1. Run `cargo tauri dev` (first instance)
2. In another terminal, run `cargo tauri dev` again
3. ✓ Dialog appears: "Reader3 Already Running"
4. ✓ First instance window receives focus

#### TC-005: Network Access (AI Features)
1. Launch app and open a book
2. Open AI panel
3. Send a question (requires configured API key)
4. ✓ AI response streams back successfully

---

## Troubleshooting

### Sidecar Won't Start

**Symptom**: Loading indicator spins indefinitely

1. Check if port 8123 is in use:
   ```bash
   lsof -i :8123
   ```

2. Try running sidecar manually:
   ```bash
   cd reader3-pro-python
   uv run reader3-server
   ```

3. Check logs:
   ```bash
   cat ~/Library/Application\ Support/Reader3/logs/reader3.log
   ```

### Orphan Processes After Crash

If python processes remain after a crash:

```bash
# Find and kill orphan processes
pkill -f "uvicorn reader3_app"
pkill -f "reader3-server"
```

### Build Errors

**Rust compilation errors**:
```bash
cd src-tauri
cargo clean
cargo build
```

**Missing Python dependencies**:
```bash
cd reader3-pro-python
uv sync --reinstall
```

---

## Log Locations

| Log Type | Location |
|----------|----------|
| Tauri app logs | `~/Library/Application Support/Reader3/logs/reader3.log` |
| macOS Console | Console.app → Filter by "com.reader3app" |
| Python sidecar | stdout/stderr captured by Tauri |

---

## Environment Variables Reference

| Variable | Description | Default |
|----------|-------------|---------|
| `READER3_BOOKS_DIR` | Book storage path | `~/Library/Application Support/Reader3/books` |
| `READER3_AI_SETTINGS` | AI config file path | `~/Library/Application Support/Reader3/config/ai_settings.json` |
| `READER3_LIBRARY_UI` | Library UI mode | `modern` |

---

## Next Steps

After completing this feature:

1. Proceed to **Phase 2** (Native Book Storage) - Use Tauri file APIs for book management
2. Test with real EPUB files from previous Reader3 Pro usage
3. Configure AI API keys in `ai_settings.json` for AI feature testing
