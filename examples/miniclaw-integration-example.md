# Plan: Integrate codex-claude-init Toolkit into MiniClaw

## Context

MiniClaw has a completed architecture blueprint (`MINICLAW_BLUEPRINT.md`) and 4 research reports but no code, no git repo, and no AI development workflow tooling. The `codex-claude-init` toolkit provides a battle-tested scaffold of agents, commands, rules, and skills for AI-assisted development. We need to clone the toolkit into the miniclaw root, customize it for a Python 3.11+ project, bootstrap the Python package structure with Protocol stubs from the blueprint, and initialize git — so that `/feature-workflow` and TDD workflows work from day one.

---

## Phase 1: Clone Toolkit (Shallow Clone + Copy + Remove .git)

Since the repo is on GitHub and authenticated via `gh`, clone it to a temp directory, copy contents (excluding `.git/`, `codex-claude-init-manual.md`, and `.DS_Store`) into miniclaw root, then remove the temp clone.

```bash
git clone https://github.com/anzchy/codex-claude-init.git /tmp/codex-claude-init
# Copy all contents except .git, the manual, and .DS_Store
cp -r /tmp/codex-claude-init/.claude /Users/jackcheng/Documents/01_Coding/Front-end-projects/miniclaw/
cp -r /tmp/codex-claude-init/.codex /Users/jackcheng/Documents/01_Coding/Front-end-projects/miniclaw/
cp -r /tmp/codex-claude-init/.specify /Users/jackcheng/Documents/01_Coding/Front-end-projects/miniclaw/
cp /tmp/codex-claude-init/.mcp.json /tmp/codex-claude-init/.gitignore /tmp/codex-claude-init/AGENTS.md /tmp/codex-claude-init/CLAUDE.md /tmp/codex-claude-init/parallel-coding-guide.md miniclaw/
rm -rf /tmp/codex-claude-init
```

**Keep as-is (no edits needed):**
- `.claude/agents/*.md` (all 9) — language-agnostic
- `.claude/commands/*.md` (all) — orchestration procedures
- `.claude/skills/` (all 6 skills) — language-agnostic
- `.claude/rules/00-engineering-principles.md` — universal
- `.claude/settings.json` — permission config
- `.codex/prompts/*.md` (all) — mirror of claude commands
- `.specify/templates/*.md` (all) — generic templates
- `.specify/scripts/bash/` (all) — generic scripts
- `.mcp.json` — already user's personal config
- `parallel-coding-guide.md` — valuable for multi-module parallel dev

---

## Phase 2: Customize Toolkit Files for Python/MiniClaw

### 2.1 `AGENTS.md` — Add Tech Stack, Project Structure, Update Gate Command

**Changes:**
- Add `## Tech Stack` section after Working Agreement listing Python 3.11+, asyncio, SQLite FTS5, click, python-telegram-bot, croniter, pyyaml, anthropic. Reference `MINICLAW_BLUEPRINT.md`.
- Add `## Project Structure` section with the `src/miniclaw/` layout from the blueprint
- Update gate command references to: `python3 -m pytest tests/ && python3 -m mypy src/ && python3 -m black --check .`
- Add note: "Always use `python3` (not `python`). Activate venv before pip operations."
- Keep all generic sections unchanged (Atomic Commits, Changelog, Conflict Resolution, Experience Log, Session Context, AI Auth)

### 2.2 `CLAUDE.md` — Add MiniClaw Project Notes

Add a section pointing to `MINICLAW_BLUEPRINT.md` as architecture reference, listing implementation order, noting workspace files are runtime data not source code, and the venv path.

### 2.3 `.claude/rules/10-tdd.md` — Replace JS Examples with Python/pytest

- **Running Tests**: Replace npm/vitest with `python3 -m pytest`, `python3 -m mypy`, `python3 -m black`
- **Pattern Catalog**: Replace 3 JS/Jest patterns with Python/pytest equivalents:
  - Unit tests: `@pytest.mark.parametrize` table-driven tests
  - State/Store: `@pytest.fixture` with `tmp_path` for isolation
  - Async/Integration: `@pytest.mark.asyncio` for coroutine testing
- **File Placement**: `tests/test_<module>.py` beside `tests/conftest.py`
- **When Tests Required table**: Remove React-specific rows, add asyncio coroutines row
- Keep RED/GREEN/REFACTOR discipline and anti-patterns unchanged

### 2.4 `.specify/memory/constitution.md` — Rewrite for MiniClaw

Replace AIC2/TypeScript content entirely:
- Project: MiniClaw (Python 3.11+)
- TDD: pytest, mypy, black (not Vitest/TypeScript)
- Local-First principle: `~/.miniclaw/workspace/` is source of truth, SQLite is derived
- Code Quality: PEP 8, type hints, ~300 line limit per module
- Constraints: 8 Protocol definitions from blueprint are module contracts; asyncio single-process, no blocking I/O

### 2.5 `.codex/config.toml` — Fix Placeholder Tokens

Replace `"xx"` placeholder values with `${GITHUB_TOKEN}` and `${MCPR_TOKEN}` env var references.

### 2.6 `.gitignore` — Expand for Python

Add: `__pycache__/`, `*.py[cod]`, `venv/`, `.venv/`, `*.egg-info/`, `dist/`, `build/`, `.pytest_cache/`, `.coverage`, `.mypy_cache/`, `*.db`, `*.db-shm`, `*.db-wal`, `.env`, `.env.local`, `.claude/contexts/`

---

## Phase 3: Bootstrap Python Project Structure

### 3.1 Create directories

```
src/miniclaw/           # Package root
tests/                  # Test suite
docs/plans/             # Planner agent output
docs/testing/           # Manual test author output
```

### 3.2 Create `pyproject.toml`

Modern Python packaging with:
- `requires-python = ">=3.11"`
- Dependencies: anthropic, python-telegram-bot, aiohttp, croniter, pyyaml, click
- `[project.scripts]` miniclaw = "miniclaw.__main__:cli"
- `[tool.pytest.ini_options]` asyncio_mode = "auto"
- `[tool.mypy]` strict = true
- `[tool.black]` line-length = 88

### 3.3 Create `requirements.txt` + `requirements-dev.txt`

Runtime (6 packages matching blueprint): anthropic, python-telegram-bot, aiohttp, croniter, PyYAML, click

Dev: pytest, pytest-asyncio, pytest-cov, mypy, black, types-PyYAML

### 3.4 Create `src/miniclaw/types.py` — Real Content

Copy all Protocol/dataclass definitions from `MINICLAW_BLUEPRINT.md` sections 4.1-4.4 verbatim:
- `MemorySearchResult`, `MemoryContext`, `MemoryStore` Protocol
- `ActiveHours`, `CronJob`, `HeartbeatResult`, `Scheduler` Protocol
- `ChatType`, `InboundMessage`, `OutboundMessage`, `MessageCallback`, `Channel` Protocol
- `SkillRequirements`, `SkillMetadata`, `Skill` Protocol, `SkillRegistry` Protocol

This is the only module with real content at bootstrap. It gives mypy something to check and tests something to verify.

### 3.5 Create module stubs

Each of `memory.py`, `skills.py`, `channels.py`, `agent.py`, `scheduler.py`, `config.py`, `__main__.py`, `__init__.py` gets a docstring and class skeleton with `NotImplementedError` bodies. Just enough for imports to work and tests to fail meaningfully (RED state).

### 3.6 Create test stubs

`tests/conftest.py` with shared fixtures (tmp_path workspace, mock channel).
`tests/test_types.py` — verify dataclasses are frozen, Protocols have expected methods.
`tests/test_memory.py`, `test_skills.py`, `test_channels.py`, `test_agent.py`, `test_scheduler.py` — Protocol compliance tests that will fail until implementations exist.

### 3.7 Create supporting files

- `.env.example` — template for ANTHROPIC_API_KEY, TELEGRAM_BOT_TOKEN, etc.
- `CHANGELOG.md` — initial entry under [Unreleased]
- `PROGRESS.md` — empty (lessons learned log)
- `README.md` — brief project overview referencing the blueprint

---

## Phase 4: Git Init + Initial Commits

```bash
cd /Users/jackcheng/Documents/01_Coding/Front-end-projects/miniclaw
git init

# Commit 1: Research artifacts (already exist)
git add MINICLAW_BLUEPRINT.md research/ nanoclaw-vs-openclaw.md refined_prompt.md OpenClaw-architecture.png
git commit -m "docs: add MiniClaw blueprint and research reports from 4-agent swarm"

# Commit 2: AI toolkit scaffold
git add AGENTS.md CLAUDE.md parallel-coding-guide.md .gitignore .mcp.json
git add .claude/ .codex/ .specify/
git commit -m "chore: integrate codex-claude-init AI toolkit scaffold"

# Commit 3: Python project bootstrap
git add pyproject.toml requirements.txt requirements-dev.txt .env.example
git add src/ tests/ docs/ CHANGELOG.md PROGRESS.md README.md
git commit -m "chore: bootstrap Python project structure with Protocol stubs"
```

---

## Phase 5: Python Environment Setup + Gate Verification

```bash
python3 -m venv venv
source venv/bin/activate
python3 -m pip install -r requirements.txt
python3 -m pip install -r requirements-dev.txt
python3 -m pip install -e .

# Verify gate (expect: mypy+black pass, pytest has RED tests)
python3 -m mypy src/                    # Should pass (stubs are typed)
python3 -m black --check .              # Should pass (freshly formatted)
python3 -m pytest tests/ -v             # Some tests FAIL (NotImplementedError) — correct RED state
```

---

## Verification

After all phases complete:
1. `git log --oneline` shows 3 clean commits
2. `ls .claude/agents/` shows 9 agent files
3. `ls .claude/commands/` shows feature-workflow.md, fix.md, audit-fix.md + speckit commands
4. `python3 -c "from miniclaw.types import MemoryStore, Channel, Scheduler, Skill"` succeeds
5. `python3 -m mypy src/` passes with no errors
6. `python3 -m black --check .` passes
7. `python3 -m pytest tests/ -v` shows RED tests (expected failures on NotImplementedError)
8. `/feature-workflow` command is available in Claude Code session
