# AGENTS.md

Shared instructions for all AI agents (Claude Code, Codex CLI, Gemini CLI, etc.).

- You are an AI assistant working on this project.
- Use English unless another language is requested.
- Follow the working agreement below.

## Working Agreement

- Run `git status -sb` at session start.
- Read relevant files before editing.
- Keep diffs focused; avoid drive-by refactors.
- Do not commit unless explicitly requested.
- Keep code files under ~300 lines (split proactively).
- No `TODO` or `FIXME` committed without a linked task ID.
- **Research before building**: For new features, search for industry best practices, established conventions, and proven solutions (web search, official docs, prior art in popular open-source projects). Don't invent when a well-tested pattern exists.
- **Edge cases are not optional**: Brainstorm as many edge cases as possible — empty input, null/undefined, max values, concurrent access, Unicode/CJK, rapid repeated actions, network failures, permission denials. Write tests for every one.
- **Test-first is mandatory** for new behavior (see "Test-Driven Development" below).
- Run the project's gate command before declaring work complete.

## Core Principles

### I. Atomic Commits

Every git commit MUST represent exactly one logical change — one feature, one bug fix, or one refactor. Unrelated changes MUST NOT be bundled in a single commit.

Commit messages MUST follow the format `type(scope): description` with types: `feat`, `fix`, `refactor`, `test`, `docs`, `chore`, `build`.

If a task touches multiple sub-projects (e.g., frontend + backend, app + sidecar), changes SHOULD be split into separate commits per sub-project when independently meaningful.

### II. Test-Driven Development

Tests MUST be written BEFORE implementation code (Red → Green → Refactor).

- Every new function or method containing logic MUST have at least one corresponding test.
- The relevant test suite MUST pass before any commit is created.
- See `.claude/rules/10-tdd.md` for the full TDD workflow, pattern catalog, and anti-patterns.

### III. Changelog Discipline (MANDATORY)

Every commit MUST have a corresponding entry in `CHANGELOG.md` at the project root. Entries follow [Keep a Changelog](https://keepachangelog.com/) format with categories: `Added`, `Changed`, `Deprecated`, `Removed`, `Fixed`, `Security`.

During development, entries accumulate under `## [Unreleased]` and are moved to a versioned section on release.

Rules:
1. For every feature added: Add entry under appropriate version section
2. For every bug/error fixed: Add entry with brief description of the fix
3. For every git commit: Reference the relevant commit hash in the changelog entry

Entry format: `- Description of change (commit_hash)`

Example:
```markdown
## [1.2.0] - 2026-02-03

### Added
- User authentication system (a1b2c3d)
- Dashboard analytics widget (e4f5g6h)

### Fixed
- Login timeout issue on slow networks (i7j8k9l)
```

### IV. Pre-Commit Documentation Check

Before every git commit, scan these files for accuracy and update them if the change affects user-facing behavior (new env vars, new tools, changed defaults, new parameters, config format changes):

- **Root-level**: `README.md`, `AGENTS.md`
- **docs/**: all documentation files

Ask: "Does this change affect any documented defaults, config examples, parameter tables, or setup instructions?" If yes, update the relevant docs in the same commit.

## Development Workflow

### Plan Before Execute

For any non-trivial task, use Plan Mode first to:
- Clarify intent and scope
- Identify affected files and potential side effects
- Propose approach for user review

Do NOT start writing code for complex features without a plan being approved first. This significantly improves task success rate by catching misunderstandings early.

Use the `/feature-workflow` command for medium-to-large features. Use `/fix` for focused bug fixes. See `.claude/commands/` for all available workflows.

### Task Lifecycle

1. **Claim task**: Pick from task queue (todo list or task file)
2. **Create working branch**: `git checkout -b type/task-description`
3. **Plan**: For non-trivial tasks, create a plan in `docs/plans/`
4. **Implement**: Work in isolated branch following TDD (RED → GREEN → REFACTOR)
5. **Test**: Run full gate command — all tests must pass
6. **Audit**: Review diffs for correctness, edge cases, rule compliance
7. **Commit**: `git commit` following atomic commit rules + changelog entry
8. **Merge + Test**: `git fetch origin && git merge origin/main` then run tests
9. **Mark complete**: Update task status
10. **Cleanup**: Delete task branch after successful merge
11. **Log lessons**: Record any issues encountered in `PROGRESS.md`

### Conflict Resolution

#### Rebase/Merge Failure
1. If "unstaged changes" error: commit or stash current changes first
2. If merge conflicts:
   - Check conflicting files with `git status`
   - Read conflict content, understand both sides' intent
   - Resolve manually (keep correct code)
   - `git add <resolved-files>` then `git rebase --continue`
3. Repeat until rebase/merge completes

#### Test Failure
1. Run tests, analyze error messages
2. Fix the bug in code
3. Re-run tests until all pass
4. Commit fix: `git commit -m "fix(scope): ..."`

**Never abandon a task** — resolve rebase/test failures before moving on. If stuck after multiple attempts, log the issue in `PROGRESS.md` and escalate to the user.

## Experience Log (PROGRESS.md)

After encountering a problem or completing a significant change, record in `PROGRESS.md` at the project root:
- What problem was encountered
- How it was resolved
- How to avoid it in the future
- **Must include the git commit ID**

Before starting any task, check `PROGRESS.md` for relevant past lessons. **The same mistake must not be repeated twice.**

`AGENTS.md` stores **rules** (static, rarely changed). `PROGRESS.md` accumulates **lessons** (growing, updated frequently). Do not mix the two.

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

## AI Coding Tool Auth

- **Prefer subscription auth over API keys** for all AI coding tools (Claude Code, Codex CLI, Gemini CLI). Subscription plans are dramatically cheaper for sustained coding sessions — API billing can cost 10-30x more.
- Claude Code: log in with Claude Max subscription. Codex CLI: `codex login` with ChatGPT Plus/Pro. Gemini CLI: Google account login.
- API keys work as a fallback for light or automated usage.
