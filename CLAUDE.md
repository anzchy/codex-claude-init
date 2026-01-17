# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Reader-Tauri is a macOS desktop application (Tauri + Python sidecar) migrating from the web-based Reader3 Pro. The goal is to wrap the existing Python EPUB reader as a native macOS app while preserving all functionality.

**Current State**: Migration Phase 0-1 (scaffolding). The Python backend in `reader3-pro-python/` is fully functional. The Tauri app shell is being set up.

## Repository Structure

```
reader-tauri/
├── reader3-pro-python/    # Existing Python backend (FastAPI + epub.js)
│   ├── src/reader3_app/   # Python source code
│   ├── books/             # Processed EPUBs (gitignored)
│   ├── config/            # Settings (ai_settings.json, library_collections.json)
│   ├── specs/             # Feature specifications
│   └── CLAUDE.md          # Detailed Python codebase guidance
├── src/                   # Tauri frontend (empty, to be scaffolded)
├── src-tauri/             # Rust sidecar launcher (to be created)
└── migration_plan.md      # Detailed migration strategy
```

## Migration Architecture

Using **Option A: Tauri shell + Python sidecar**:

1. **Tauri app** launches and manages the Python sidecar process
2. **Python sidecar** (FastAPI server) runs on `127.0.0.1:8123`
3. **WebView** loads the Python server's pages
4. All existing functionality (EPUB rendering, search, AI, annotations) stays in Python

Key preservation requirements:
- `window.Reader3Ctx` bootstrapping for epub.js
- Iframe DOM injection for styles/annotations
- `book.epub` accessible via fetch/URL
- `book.pkl` serialization format (Python pickle)

## Development Commands

### Python Backend (in `reader3-pro-python/`)

```bash
cd reader3-pro-python

# Install dependencies
uv sync

# Process an EPUB
uv run reader3 ./books/<file.epub>

# Index for semantic search
uv run reader3 --index <book_folder>

# Start web server (http://127.0.0.1:8123)
uv run reader3-server
```

### Tauri App (once scaffolded)

```bash
# Install Tauri CLI
cargo install tauri-cli

# Development mode
cargo tauri dev

# Build for production
cargo tauri build
```

## Migration Phases

| Phase | Description | Status |
|-------|-------------|--------|
| 0 | Lock constraints (sidecar, network access, storage paths) | Current |
| 1 | Tauri app shell + sidecar launcher + health check | Next |
| 2 | EPUB rendering bridge (keep existing pipeline) | Planned |
| 3 | Move storage to app data directory | Planned |
| 4 | API layer (keep FastAPI as-is) | Planned |
| 5 | Search (keyword + semantic in Python) | Planned |
| 6 | AI stack (multi-provider, streaming, RAG) | Planned |
| 7 | Packaging (PyInstaller or embedded Python) | Planned |

## Key Decisions

- **Port strategy**: Fixed `8123` for simplicity (can switch to dynamic if conflicts arise)
- **Sidecar packaging**: TBD (PyInstaller vs `uv` + embedded Python runtime)
- **Storage location**: Tauri app data root for `books/` and `config/`
- **Network access**: Required for AI providers (Anthropic, OpenAI, Gemini, Ollama, OpenRouter)

## Python Codebase Reference

The Python backend has detailed documentation:
- `reader3-pro-python/CLAUDE.md` - Architecture, data structures, API endpoints
- `reader3-pro-python/AGENTS.md` - Development guidelines
- `reader3-pro-python/.specify/memory/constitution.md` - Core principles

### Four-Layer Architecture (Python)

```
Layer 1: EPUB Processing (reader3.py) → book.pkl
Layer 2: Search & Vector Services (search_service.py, vector_search.py)
Layer 3: AI Assistant (ai/)
Layer 4: Web Server (web/server.py) on :8123
```

### Critical Concepts

- **Spine vs TOC**: Spine is linear reading order (Previous/Next); TOC is navigation tree (sidebar)
- **Book identity**: `book_folder` (filesystem) vs `book_uuid` (in `ai_meta.json`)
- **File-based state**: Each book is a `{bookname}_data/` directory with `book.pkl`, `book.epub`, `images/`

## Tauri-Specific Considerations

### CSP Configuration (`tauri.conf.json`)
```json
{
  "security": {
    "csp": "default-src 'self' http://127.0.0.1:8123; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline' https://fonts.googleapis.com; font-src https://fonts.gstatic.com; img-src 'self' http://127.0.0.1:8123 data: blob:; connect-src 'self' http://127.0.0.1:8123"
  }
}
```

### Sidecar Health Check
Before loading WebView, poll `GET http://127.0.0.1:8123/` until 200 response (max 30s timeout).

### Environment Variables for Sidecar
```bash
READER3_BOOKS_DIR=<app_data>/books
READER3_AI_SETTINGS=<app_data>/config/ai_settings.json
READER3_LIBRARY_UI=modern
```

## Portable Modules

Can reuse with minimal changes:
- `static/**` (JS/CSS assets)
- `templates/*.html` (Jinja2 templates)
- `vendor/epub.min.js`, `vendor/jszip.min.js`
- Notes/collections JSON schemas

Requires adaptation:
- Environment variable handling for app data paths
- Sidecar process management
- Possible PyInstaller bundling

## Active Technologies
- Rust (Tauri 2.x) + Python 3.10+ (existing FastAPI sidecar) (001-tauri-sidecar-shell)
- File-based (existing `books/*_data/` + `config/` under Tauri app data) (001-tauri-sidecar-shell)

## Recent Changes
- 001-tauri-sidecar-shell: Added Rust (Tauri 2.x) + Python 3.10+ (existing FastAPI sidecar)
