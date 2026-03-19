# Universal Claude Config Upgrade Guide (Electron + TypeScript)

## Purpose

Use this as a reusable playbook to upgrade any Electron + TypeScript project's `.claude/` config, using `xiaolai/vmark/.claude` as a high-quality baseline while avoiding Tauri/Rust stack mismatch.

Reference baseline:
- https://github.com/xiaolai/vmark/tree/main/.claude

---

## 1) Project Profile Inputs (Fill Before Migration)

Set these placeholders first:

- `<PROJECT_NAME>`
- `<GATE_COMMAND>` (example: `pnpm lint && pnpm test && pnpm typecheck && pnpm build`)
- `<MAIN_PROCESS_PATH>` (example: `src/main`)
- `<PRELOAD_PATH>` (example: `src/preload`)
- `<RENDERER_PATH>` (example: `src/renderer`)
- `<DOCS_PATH>` (example: `docs/` or `website/guide/`)
- `<E2E_STACK>` (example: Playwright)
- `<PACKAGE_TOOL>` (example: `electron-builder` or `electron-forge`)

---

## 2) Universal Keep/Edit/Replace Matrix (From `vmark/.claude`)

## Keep As-Is (Usually Safe)

- `agents/*.md`
- `commands/feature-workflow.md`
- `commands/fix.md`
- `commands/fix-issue.md`
- `commands/merge-prs.md`
- `rules/00-engineering-principles.md` (or your current equivalent)
- `rules/10-tdd.md` (keep generic version)
- `skills/planning/SKILL.md`
- `skills/plan-audit/SKILL.md`
- `skills/plan-verify/SKILL.md`
- `skills/release-gate/SKILL.md`
- `skills/mcp-server-manager/SKILL.md`

## Keep But Adapt For Electron + TypeScript

- `.claude/README.md`
- `.claude/settings.json`
- `rules/20-logging-and-docs.md`
- `rules/21-website-docs.md`
- `rules/22-comment-maintenance.md`
- `rules/30-ui-consistency.md` (if UI-heavy project)
- `rules/31-design-tokens.md` (if tokenized design system exists)
- `rules/32-component-patterns.md` (if frontend design system exists)
- `rules/33-focus-indicators.md` (if accessibility rules are needed)
- `rules/34-dark-theme.md` (if theme support exists)
- `rules/40-version-bump.md`
- `rules/41-keyboard-shortcuts.md`
- `rules/50-codebase-conventions.md`
- `commands/bump.md`
- `commands/test-guide.md`
- `hooks/refine_prompt.sh` (optional)
- `hooks/refine_prompt.txt` (optional)

## Replace / Skip (Tauri/Rust-Specific)

- `skills/rust-tauri-backend`
- `skills/tauri-app-dev`
- `skills/tauri-mcp-test-runner`
- `skills/tauri-mcp-testing`
- `skills/tauri-v2-integration`

Conditional:

- `skills/tiptap-dev`, `skills/tiptap-editor` only if you actually use Tiptap/ProseMirror.
- `skills/react-app-dev` only if renderer uses React.
- `skills/css-design-tdd` only if your CSS architecture matches that workflow.

---

## 3) Recommended Universal `.claude` Structure

```text
.claude/
  README.md
  settings.json
  settings.local.json
  agents/
  commands/
    feature-workflow.md
    fix.md
    fix-issue.md
    merge-prs.md
    bump.md
    test-guide.md
    ...project-specific commands...
  hooks/
    refine_prompt.sh
    refine_prompt.txt
  rules/
    00-engineering-principles.md
    10-tdd.md
    20-logging-and-docs.md
    21-website-docs.md
    22-comment-maintenance.md
    30-ui-consistency.md
    31-design-tokens.md
    32-component-patterns.md
    33-focus-indicators.md
    34-dark-theme.md
    40-version-bump.md
    41-keyboard-shortcuts.md
    50-codebase-conventions.md
  skills/
    planning/
    plan-audit/
    plan-verify/
    release-gate/
    mcp-server-manager/
    electron-main-dev/
    electron-ipc-security/
    electron-e2e/
    ...optional skills...
  scripts/
    codex-preflight.sh
```

---

## 4) Universal Adaptation Rules

Apply these edits to imported `vmark` content:

1. Replace stack terms:
- `Tauri`, `src-tauri`, `cargo`, `tauri.conf.json`, `invoke_handler`
- with Electron equivalents: `Electron`, `<MAIN_PROCESS_PATH>`, `ipcMain/ipcRenderer`, `<PACKAGE_TOOL>` config paths.

2. Replace gate commands:
- any `pnpm check:all` + Rust checks
- with `<GATE_COMMAND>`.

3. Rewrite path mappings in rules:
- VMark-specific paths in rules `21/40/41/50`
- with paths from `<MAIN_PROCESS_PATH>`, `<PRELOAD_PATH>`, `<RENDERER_PATH>`, `<DOCS_PATH>`.

4. Rewrite keyboard shortcut governance:
- Tauri menu + store rules
- into Electron `Menu` template + renderer shortcut handler + docs sync rules.

5. Rewrite E2E sections:
- Tauri MCP testing flows
- into `<E2E_STACK>` flows for desktop Electron app.

6. Rewrite examples in TDD and conventions:
- remove stack-specific examples you do not use (for example, Rust/Tauri/Zustand/Tiptap if not present).

---

## 5) Migration Procedure (Reusable)

1. Create a migration branch:
```bash
git checkout -b chore/claude-config-upgrade
```

2. Backup current config:
```bash
cp -R .claude .claude.backup.$(date +%Y%m%d-%H%M%S)
```

3. Import selected baseline files from `vmark/.claude` by the matrix in section 2.

4. Apply all adaptations from section 4.

5. Validate config integrity:
- no Tauri/Rust commands remain
- every command references real project files
- all gate commands exist
- all doc paths exist or are intentionally created later

6. Run `<GATE_COMMAND>`.

7. Update root docs (`README.md`, `AGENTS.md`, and relevant docs in `<DOCS_PATH>`).

---

## 6) How To Use This Setup (From Zero to Delivery)

## Feature Flow

1. Create task or issue.
2. Run `/feature-workflow <feature-slug>`.
3. Ensure each work item contains edge cases, RED tests, acceptance criteria, rollback notes.

## Implementation Flow

1. RED: write failing tests first.
2. GREEN: implement minimum passing behavior.
3. REFACTOR: clean structure without behavior change.

## Verification Flow

1. Run `<GATE_COMMAND>`.
2. Run `/audit-fix` for modified scope.
3. Confirm docs sync for user-visible changes.

## Release Flow

1. Use `/fix-issue #<id>` for issue-driven delivery.
2. Use `/merge-prs` for controlled merges.
3. Use adapted `/bump` for versioning.

---

## 7) Universal Electron Skills To Add

Create these reusable skills for any Electron + TypeScript project:

1. `skills/electron-main-dev/SKILL.md`
- window lifecycle, menus, tray, app events, packaging hooks.

2. `skills/electron-ipc-security/SKILL.md`
- preload bridge contracts, channel allowlists, validation, security defaults (`contextIsolation`, `nodeIntegration: false`).

3. `skills/electron-e2e/SKILL.md`
- app launch, UI flows, IPC verification, crash/log capture with `<E2E_STACK>`.

---

## 8) Universal Quick-Start Template

When starting a new Electron + TypeScript repo, copy this checklist:

1. Set placeholders in section 1.
2. Import baseline files by section 2 matrix.
3. Run section 4 adaptation rules.
4. Add section 7 Electron skills.
5. Execute section 5 migration procedure.
6. Enforce section 6 daily workflow.

---

## 9) Practical Guidance

Do not copy `vmark/.claude` blindly into Electron projects. Keep the process architecture, replace stack-bound implementation rules. This gives you disciplined workflows with minimal technical debt across future Electron + TypeScript repos.
