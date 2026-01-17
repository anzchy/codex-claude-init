# Migration Plan: reader3-pro-python -> Tauri (macOS 13+ ARM)

## 1) Current Architecture Summary (reader3-pro-python)

### Ingestion and storage
- `reader3_app/reader3.py` parses EPUB using ebooklib + BeautifulSoup.
- Outputs per-book folder `books/<book_id>_data/` with:
  - `book.pkl` (pickled Book dataclass with metadata, spine, toc, text)
  - `book.epub` (original EPUB copy for rendering)
  - `images/` (extracted assets)
  - Optional: `notes.json`, `ai_conversations/`, `chroma_db/`

### Web server (FastAPI)
- `reader3_app/web/server.py` serves:
  - Library page (`/`) and reader (`/read/{book_id}/{chapter_index}`) via Jinja templates.
  - JSON APIs for search, notes, collections, AI, indexing.
  - Static assets under `/static`.
- `book.pkl` is used for metadata, TOC, search indexing.
- `book.epub` is served via `/read/{book_id}/book.epub` for front-end rendering.

### Frontend
- Templates: `reader3_app/web/templates/*.html`
- JS/CSS assets: `reader3_app/web/static/**`
- EPUB rendering: `static/vendor/epub.min.js` + `jszip.min.js`.
- Reader view uses `static/js/epub-renderer.js` to render EPUB in an iframe.
- Annotations, search, notes, AI panels are pure JS and use HTTP APIs.

### Search and AI
- Keyword search: `search_service.py` over `book.pkl` text.
- Semantic search: `vector_search.py` with ChromaDB + sentence-transformers.
- AI: provider adapters in `ai/providers/*` with streaming + RAG helpers.


## 2) EPUB Rendering Pipeline and Injection Points (critical)

### Current rendering flow
1) Backend provides `reader.html` and injects `window.Reader3Ctx` (bookId, chapterIndex, epubUrl, etc.).
2) Frontend `epub-renderer.js`:
   - `book = ePub(ctx.epubUrl)` loads the `book.epub` file.
   - `rendition = book.renderTo(viewer, { flow, spread, width, height })` creates iframe.
   - `rendition.on("rendered")` updates TOC, chapter index, and dispatches events.
3) DOM injection inside iframe:
   - `ensureContentStyles(contents)` adds a `<style>` tag (`#reader3-epub-theme`) and Google font link.
   - `annotations.js` uses `window.Reader3Epub.onContentsReady()` to:
     - scan and mark text
     - attach selection listeners
     - render highlights and notes
4) Search highlight injection:
   - `search.js` calls `/api/highlight/...` for ranges and applies `<mark>` tags.

### Injection that must be preserved in Tauri
- `Reader3Ctx` bootstrapping must exist before `epub-renderer.js` runs.
- Ability to access iframe document and inject style and annotations.
- `book.epub` must be accessible via fetch/URL by epub.js.


## 3) Feasibility Analysis

### What is easy to move
- Frontend static assets (HTML/JS/CSS) can be reused with minimal changes.
- EPUB rendering (epub.js + jszip) works inside a Tauri WebView.
- Notes and collections JSON formats can be kept with a Rust or JS store layer.

### Hard parts
- `book.pkl` is Python pickle. Tauri (Rust/JS) cannot read it.
  - Must switch to JSON or keep Python in the loop.
- EPUB parsing (ebooklib + BeautifulSoup) has no direct Rust equivalent with full parity.
- Semantic search relies on Python libraries and model assets.
- AI providers and streaming logic are implemented in Python.

### EPUB rendering in Tauri (most critical)
- epub.js expects a URL or Blob. In Tauri you can:
  1) Serve `book.epub` over a local HTTP server (Rust or Python sidecar). Easiest.
  2) Read the file via Tauri FS API and pass a Blob URL to epub.js.
- If you use Google Fonts, allow network access or package fonts locally.
- CSP must allow `blob:` and local HTTP (if used).

### Overall feasibility
- High feasibility if you keep a Python sidecar and reuse the existing server.
- Medium feasibility for a full Rust rewrite due to EPUB parsing, pickle format, and AI/RAG dependencies.


## 4) Migration Strategy Options

### Option A: Tauri shell + Python sidecar (fastest, lowest risk)
- Bundle Python backend (FastAPI) as a sidecar binary.
- Tauri loads `http://127.0.0.1:8123` in the WebView.
- Minimal frontend changes. No rewrite of EPUB pipeline or AI/RAG.
- Tradeoff: larger app size and packaging complexity.

### Option B: Full Rust backend
- Replace FastAPI endpoints with Tauri commands or a Rust HTTP server.
- Replace EPUB parsing with Rust crates + HTML cleanup.
- Replace `book.pkl` with `book.json` (new data format).
- Requires reimplementing semantic search and AI client logic.

### Option C: Hybrid
- Rust core for filesystem, notes, collections, keyword search.
- Python sidecar only for EPUB ingestion and semantic search / AI.


## 5) Chosen Approach

- Use Option A: Tauri shell + Python sidecar.
- Keep `book.pkl` and all Python-based pipelines.
- Network access is required for AI providers (and optional fonts).


## 6) Recommended Path (phased, low risk, sidecar)

### Phase 0: Lock in constraints
- Sidecar is required (keeps `book.pkl` and Python logic).
- Allow outbound network calls for AI providers.
- Keep existing folder layout under app data (`books/`, `config/`).

### Phase 1: Tauri app shell + sidecar start
- Create/confirm Tauri app and add a Rust-side launcher for the Python server.
- Start sidecar on app boot (prefer fixed port `8123` or discover a free port and pass it to the webview).
- Health check: poll `GET http://127.0.0.1:8123/` before loading the webview.
- Configure `tauri.conf.json`:
  - allow `http://127.0.0.1:8123` in `security.csp`
  - allow `blob:` and `tauri://` schemes
  - allow Google Fonts if not bundling fonts locally

### Phase 2: EPUB rendering bridge (sidecar)
- Keep existing Jinja injection + `window.Reader3Ctx` from FastAPI templates.
- Ensure epub.js fetches `book.epub` via `/read/{book_id}/book.epub`.
- Keep iframe injection hooks in `epub-renderer.js` and `annotations.js` unchanged.

### Phase 3: Data storage
- Move storage to app data:
  - `books/` and `config/` live under Tauri app data root.
- Pass env vars to sidecar:
  - `READER3_BOOKS_DIR=<app_data>/books`
  - `READER3_AI_SETTINGS=<app_data>/config/ai_settings.json`
  - Optional: `READER3_LIBRARY_UI=modern`

### Phase 4: API layer
- Keep FastAPI API surface as-is (no `fetch` changes required).
- Tauri only manages app lifecycle and file paths.

### Phase 5: Search
- Keep keyword + semantic search in Python.
- Highlighting stays in the browser (unchanged).

### Phase 6: AI
- Keep Python AI stack and settings file in app data.
- Allow network access for providers (Anthropic/OpenAI/Gemini/OpenRouter/Ollama).

### Phase 7: Packaging and testing
- Package sidecar (PyInstaller or uv + embedded Python) for macOS 13 ARM.
- Verify permissions for reading local EPUBs and writing app data.
- Test: EPUB rendering, notes/annotations, search (keyword + semantic), AI streaming.


## 7) Directly Portable Modules

### Can be reused with minimal changes
- `reader3_app/web/static/**` (JS/CSS) except for API endpoint base URLs.
- `reader3_app/web/templates/*.html` (if still using server rendering).
- `static/vendor/epub.min.js`, `static/vendor/jszip.min.js`.
- Notes and collections JSON schemas.

### Requires rewrite or adaptation
- `reader3_app/reader3.py` (EPUB parsing, pickle output).
- `book.pkl` format (replace with JSON or keep Python to read).
- `reader3_app/web/server.py` (FastAPI server).
- `vector_search.py` (ChromaDB + sentence-transformers).
- `ai/*` providers (if removing Python).


## 8) Open Questions (resolved)

- Option: sidecar.
- Keep `book.pkl`: yes.
- Network access: required.


## 9) Immediate Next Steps (sidecar)

- Confirm whether a Tauri app already exists under `src-tauri/`.
- Decide port strategy:
  - fixed `8123` (simpler), or
  - dynamic port + env + webview URL (safer for conflicts).
- Define sidecar packaging approach (PyInstaller vs uv + embedded runtime).
- Add a minimal health check loop before loading the webview.
