## Session Context Management

**When context is running low (< 10% before auto-compact):**

1. Run `/save-context` to save current session state

2. Context saved to `.claude/contexts/YYYY-MM-DD_HH-MM-SS.md`

3. Includes: completed tasks, in-progress work, pending tasks, key decisions

**When starting a new session after compaction:**

1. Run `/load-context` to restore previous session state

2. Review the loaded context summary

3. Continue with in-progress or pending tasks

**Context files location:** `.claude/contexts/`

## Core Principles

When using Gemini MCP tools, prefer gemini-3-pro-preview unless a different model is specifically needed.

### I. Atomic Commits

Every git commit MUST represent exactly one logical change — one feature, one bug fix, or one refactor. Unrelated changes MUST NOT be bundled in a single commit. Commit messages MUST follow the format `type(scope): description` with types: `feat`, `fix`, `refactor`, `test`, `docs`, `chore`, `build`. If a task touches both the Swift app and Python sidecar, changes SHOULD be split into separate commits per sub-project when independently meaningful.

### II. Test-Driven Development

Tests MUST be written BEFORE implementation code (Red → Green → Refactor). Every new function or method containing logic MUST have at least one corresponding test. Python tests use `pytest` in `sidecar/tests/`. Swift tests use `XCTest` in `OpenPDF/Tests/`. Integration tests live in `tests/integration/`. The relevant test suite MUST pass before any commit is created.

### III. Changelog Discipline

Every commit MUST have a corresponding entry in `CHANGELOG.md` at the project root. Entries follow [Keep a Changelog](https://keepachangelog.com/) format with categories: `Added`, `Changed`, `Deprecated`, `Removed`, `Fixed`, `Security`. During development, entries accumulate under `## [Unreleased]` and are moved to a versioned section on release. Entry format: `- Description of change (commit_hash)`.

### IV. Privacy & Local-First

All ML inference and document processing MUST run on-device. No document content, embeddings, or user data may be transmitted to external servers. All data is stored locally in `~/Library/Application Support/OpenPDF/`. The application MUST function fully offline after initial model download.

### V. Changelog Tracking (MANDATORY)

**Every feature added, error fixed, or git commit MUST be documented in ******\`\`****** at the project root.**

Rules:

1. Maintain a `CHANGELOG.md` file in the project root directory

2. For every feature added: Add entry under appropriate version section with description

3. For every bug/error fixed: Add entry with brief description of the fix

4. For every git commit: Reference the relevant commit hash in the changelog entry

5. Follow [Keep a Changelog](https://keepachangelog.com/) format:

   - `Added` for new features

   - `Changed` for changes in existing functionality

   - `Deprecated` for soon-to-be removed features

   - `Removed` for now removed features

   - `Fixed` for any bug fixes

   - `Security` for vulnerability fixes

Example entry:

```markdown
## [1.2.0] - 2026-02-03

### Added
- User authentication system (a1b2c3d)
- Dashboard analytics widget (e4f5g6h)

### Fixed
- Login timeout issue on slow networks (i7j8k9l)
```

## VI Pre-Commit Documentation Check

Before every git commit, scan these files for accuracy and update them if the change affects user-facing behavior (new env vars, new tools, changed defaults, new parameters, model changes, config format changes):

- **Root-level**: `README.md`, `CLAUDE.md`

- **docs/**: `all md files in it.`

Ask: "Does this change affect any documented defaults, config examples, parameter tables, or setup instructions?" If yes, update the relevant docs in the same commit.

## Development Workflow

### Code Quality

- Swift: strict concurrency checking, SwiftUI previews for UI components

- Python: PEP 8 style, type hints on all functions, `structlog` for logging

- Both: no `TODO` or `FIXME` committed without a linked task ID

### Testing Strategy

- `XCTest` for Swift unit and UI tests

- `pytest` for Python unit tests (`src/tests/`)

- Integration tests in `tests/integration/` for end-to-end UDS protocol validation

- Tests MUST pass before committing (TDD principle II)

##

