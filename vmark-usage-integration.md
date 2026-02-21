# VMark `.claude/` Integration Guide

A comprehensive analysis of the [xiaolai/vmark](https://github.com/xiaolai/vmark) repository's `.claude/` folder, real-world usage examples traced from git history, and a practical guide for setting up a new project from scratch.

---

## Table of Contents

1. [Research Findings — Q&A](#research-findings)
2. [VMark `.claude/` Architecture Overview](#architecture-overview)
3. [New Project Setup Procedure (From Scratch)](#new-project-setup-procedure)
4. [Real-World Example: Document History Feature](#real-world-example-document-history-feature)
5. [Real-World Example: Audio/Video/YouTube Feature (PR #70)](#real-world-example-audiovideoyoutube-feature-pr-70)
6. [How VMark's `.claude/` Was Built — Git History Evolution](#how-vmarks-claude-was-built)
7. [File Inventory — What to Copy, What to Modify](#file-inventory)
8. [Best Practices Summary](#best-practices-summary)

---

## Research Findings

### Q1: Do the agents use Claude's Agent Teams SDK to work in parallel?

**No.** The agents described in `.claude/agents/` are **not** built with the Claude Agent SDK (the `@anthropic-ai/claude-agent-sdk` npm package for building custom multi-agent systems). They are **prompt-based role definitions** — markdown files that instruct Claude Code's built-in orchestration to behave as specialized subagents.

How they actually work:

- Each agent file (e.g., `planner.md`, `implementer.md`) is a **YAML-frontmatter + markdown** document with a `name`, `description`, `tools` list, and `skills` list.
- The `/feature-workflow` command (a slash command in `.claude/commands/feature-workflow.md`) acts as a **sequential orchestrator** that invokes these agents **one at a time, in a fixed order** (Plan → Spec Check → Impact → Implement → Test → Audit → Manual Test Guide → Verify → Release).
- The orchestrator uses Claude Code's native `Task` tool to delegate to subagents. Each agent receives its role prompt, the allowed tools, and the relevant skills. This is **Claude Code's built-in subagent delegation** — not the Agent SDK's multi-process parallel execution.
- The **only place** the Agent SDK is used in the entire repo is in `.claude/hooks/refine_prompt.mjs`, where it calls `query()` from `@anthropic-ai/claude-agent-sdk` to send a prompt to Claude Haiku for translation/refinement. This is a single-turn utility call, not a multi-agent team.

**Key distinction:**
| Feature | VMark's Approach | Claude Agent SDK |
|---------|-----------------|------------------|
| Agent definition | Markdown role prompts | Programmatic agent classes |
| Orchestration | Sequential slash command | Code-driven parallel/sequential |
| Parallel execution | Not used (serial pipeline) | Supported natively |
| Runtime | Claude Code CLI session | Standalone Node.js/Python process |
| Tool access | Claude Code's built-in tools | SDK-provided tool system |

The implementer agent's docs do mention "subagent delegation is encouraged" for large diffs, meaning Claude Code can recursively spawn `Task` subagents within a single agent step — but this is still Claude Code's native capability, not the Agent SDK.

### Q2: Best practices for using `/feature-workflow`

Based on the workflow definition and agent descriptions, here are the best practices:

1. **Start with a clear, scoped goal** — Provide a `work-name` slug and either a rough plan or a problem statement. The planner agent does best when the scope is bounded (e.g., "file-mgmt-rebuild-phase0", not "make the app better").

2. **Let each gate do its job — don't skip steps:**
   - Planner creates modular Work Items with acceptance criteria
   - Spec Guardian validates against project rules (catches spec drift early)
   - Impact Analyst maps the minimal change set (prevents over-scoping)
   - These three steps together prevent the most common failure mode: implementing the wrong thing

3. **Review and approve at gate boundaries** — The workflow is "gated" — it pauses for human review at key points. The acceptance contract states: *"If uncertain: stop and ask rather than guessing."* Treat each agent's output as a proposal, not a fait accompli.

4. **Provide an existing plan when possible** — The workflow accepts "an existing plan doc to refine." Starting with even a rough plan document accelerates the Planner step significantly.

5. **One commit per Work Item** — The Release Steward enforces atomic commits. Never bundle multiple Work Items into one commit.

6. **Don't skip TDD** — The Implementer follows strict RED → GREEN → REFACTOR. The Test Runner verifies with `pnpm check:all`. Coverage thresholds are ratcheted — they can only go up.

7. **Use it for medium-to-large features** — For small bug fixes, use `/fix` or `/fix-issue` instead. The feature workflow's 9-step pipeline has overhead that only pays off for multi-file, multi-commit work.

8. **Keep the feedback loop tight** — When the Auditor finds issues, it loops back to Implement. When the Verifier fails, it loops back to the relevant step. Don't try to "finish" — iterate until gates pass.

### Q3: Does the workflow automatically handle everything from rough plan to git commit?

**Partially yes, partially no.** Here's what's automatic vs. what requires user intervention:

#### Automatic (handled by agents):

| Step | Agent | What it does automatically |
|------|-------|---------------------------|
| Plan refinement | Planner | Takes a rough plan → creates modular Work Items with acceptance criteria, tests, edge cases |
| Spec validation | Spec Guardian | Validates plan against `AGENTS.md` and `.claude/rules/*.md` |
| Impact analysis | Impact Analyst | Maps minimal file set, dependency edges, risks per Work Item |
| Implementation | Implementer | Writes failing tests (RED), implements (GREEN), refactors |
| Test execution | Test Runner | Runs `pnpm check:all`, reports pass/fail |
| Code review | Auditor | Reviews diffs for correctness, architecture drift, rule violations |
| Test guide | Manual Test Author | Creates/updates manual testing documentation |
| Verification | Verifier | Re-runs gates, produces final checklist |
| Commit prep | Release Steward | Proposes commit messages (one per Work Item) |

#### Requires user action:

| Action | Why |
|--------|-----|
| **Trigger the workflow** | User must invoke `/feature-workflow [work-name]` |
| **Approve/refine the plan** | Planner outputs a plan for review; user should validate scope and priorities |
| **Resolve spec conflicts** | If Spec Guardian finds conflicts, user decides how to resolve |
| **Run the app for E2E testing** | VMark is a Tauri app — automated E2E requires the user to launch it |
| **Accept commits** | Release Steward proposes commits but **never commits without explicit user "accept + commit"** |
| **Handle audit failures** | If Auditor finds critical issues, user may need to make judgment calls |
| **Push/PR** | The workflow stops at local commits; pushing and creating PRs is a separate step |

So: **the workflow automates ~80% of the work** (planning, coding, testing, reviewing, commit message crafting), but it's designed as a **human-in-the-loop system** where the user approves at key gates.

---

## Architecture Overview

The VMark `.claude/` folder is organized into 6 functional areas:

```
.claude/
├── README.md                  # Documentation for the .claude folder itself
├── settings.json              # Team-shared settings (plugins, hooks)
├── settings.local.json        # Personal settings (gitignored)
│
├── agents/                    # 9 subagent role definitions
│   ├── planner.md             #   Research → modular Work Items
│   ├── spec-guardian.md       #   Validates plan vs rules
│   ├── impact-analyst.md      #   Maps minimal file changes
│   ├── implementer.md         #   TDD code changes
│   ├── test-runner.md         #   Runs tests + gates
│   ├── auditor.md             #   Reviews diffs
│   ├── manual-test-author.md  #   Manual testing docs
│   ├── verifier.md            #   Final pre-release check
│   └── release-steward.md     #   Commits + release notes
│
├── commands/                  # Slash commands (user-invocable workflows)
│   ├── feature-workflow.md    #   9-step gated pipeline orchestrator
│   ├── fix.md                 #   Root-cause TDD bug fixing
│   ├── fix-issue.md           #   GitHub issue → branch → fix → PR
│   ├── audit-fix.md           #   Audit + fix + verify loop
│   ├── bump.md                #   Version bump procedure
│   ├── merge-prs.md           #   PR review + merge
│   ├── test-guide.md          #   Manual test guide generation
│   ├── codex-*.md             #   Cross-model audit via Codex CLI
│   └── shared/                #   Shared command fragments
│
├── rules/                     # Auto-loaded project rules (always active)
│   ├── 00-engineering-principles.md
│   ├── 10-tdd.md
│   ├── 20-logging-and-docs.md
│   ├── 30-ui-consistency.md   # (+ 31, 32, 33, 34)
│   ├── 40-version-bump.md     # (+ 41)
│   └── 50-codebase-conventions.md
│
├── skills/                    # Extended capabilities (loaded on demand)
│   ├── planning/              #   Plan creation with templates
│   ├── plan-audit/            #   Audit work against a plan
│   ├── plan-verify/           #   Verify completion
│   ├── release-gate/          #   Quality gate scripts
│   ├── ai-coding-agents/      #   Multi-tool reference guide
│   └── (domain-specific)/     #   react, tauri, tiptap, rust, mcp, etc.
│
├── hooks/                     # Lifecycle hooks
│   ├── package.json           #   @anthropic-ai/claude-agent-sdk
│   ├── refine_prompt.mjs      #   :: or >> prompt translation
│   └── project-context.txt    #   Context for prompt refinement
│
└── scripts/
    └── codex-preflight.sh     # Pre-flight checks for Codex commands

Root files:
├── AGENTS.md                  # Single source of truth for ALL AI tools
├── CLAUDE.md                  # Entry point — just @AGENTS.md
└── .mcp.json                  # MCP server registrations
```

---

## New Project Setup Procedure

### Phase 1: Foundation (Day 1 — Before writing any code)

This is the order the vmark author followed, based on the git history (commit `837e247` on 2026-02-12 was the big-bang setup).

#### Step 1: Create `AGENTS.md` at the project root

This is the **single source of truth** that every AI tool reads. VMark's CLAUDE.md is just 2 lines — it simply says `@AGENTS.md`.

**Why `AGENTS.md` instead of putting everything in `CLAUDE.md`?**
- `AGENTS.md` is tool-agnostic — works with Claude Code, Codex CLI, Gemini CLI, or any future tool
- `CLAUDE.md` becomes a thin pointer — easy to maintain
- Each tool can have its own entry point (`CLAUDE.md`, `.codex/` config) but they all read the same shared rules

```markdown
# AGENTS.md

Shared instructions for all AI agents (Claude, Codex, etc.).

- You are an AI assistant working on the [PROJECT_NAME] project.
- Use English unless another language is requested.

## Working Agreement
- Run `git status -sb` at session start.
- Read relevant files before editing.
- Keep diffs focused; avoid drive-by refactors.
- Do not commit unless explicitly requested.
- Keep code files under ~300 lines (split proactively).
- **Research before building**: search for best practices before inventing.
- **Edge cases are not optional**: brainstorm exhaustively.
- **Test-first is mandatory**: RED → GREEN → REFACTOR.
- Run `[YOUR_GATE_COMMAND]` for gates.

## Tech Stack
- [List your stack here]

## Key Patterns
- [List architecture patterns specific to your project]
```

#### Step 2: Create `CLAUDE.md` as a pointer

```markdown
# CLAUDE.md

@AGENTS.md

## Claude-specific notes
- Add Claude-only guidance here if needed (keep rules in AGENTS.md).
```

#### Step 3: Create the `.claude/` directory structure

```bash
mkdir -p .claude/{agents,commands,commands/shared,rules,skills/planning/templates,hooks,scripts}
```

#### Step 4: Create `.claude/settings.json`

```json
{
  "permissions": {
    "allow": []
  }
}
```

Add to `.gitignore`:
```
.claude/settings.local.json
```

#### Step 5: Create the rules (auto-loaded by Claude Code)

Rules are **numbered by category** and **auto-loaded every session**. Start with just the essentials:

**`.claude/rules/00-engineering-principles.md`**:
```markdown
# 00 - Engineering Principles

Follow the shared rules in `AGENTS.md`.

Key points:
- Read before editing; keep diffs focused.
- Keep features local; avoid cross-module imports unless shared.
- Keep code files under ~300 lines.
```

**`.claude/rules/10-tdd.md`**:
```markdown
# 10 - TDD Workflow

## Core: RED → GREEN → REFACTOR

1. RED — Write a failing test describing expected behavior.
2. GREEN — Write minimum code to make it pass.
3. REFACTOR — Clean up without behavior change.

## When Tests Are Required
| Category | Required? |
|----------|-----------|
| Business logic | ALWAYS |
| Utils/helpers | ALWAYS |
| Bug fixes | ALWAYS (regression test) |
| Edge cases | ALWAYS |
| CSS-only | No |
| Docs/config | No |

## Pattern Catalog
[Add your stack-specific test patterns here — see VMark's 10-tdd.md for examples]

## Running Tests
[YOUR_TEST_COMMANDS_HERE]
```

### Phase 2: Agent Definitions (Day 1 — Copy + Adapt)

#### Step 6: Create the 9 agent definitions

Copy each from vmark's `.claude/agents/` and adapt. The key changes per file:

**`.claude/agents/planner.md`** — Replace `skills:` line:
```yaml
---
name: planner
description: Turns a goal into modular work items with tests and acceptance gates.
tools: Read, Grep
skills: []
---
```

**`.claude/agents/implementer.md`** — Replace test commands and patterns:
```yaml
---
name: implementer
description: Implements scoped changes with tests and minimal diffs.
tools: Read, Edit, Bash
skills: []
---
```
In the body, replace `pnpm check:all` with your gate command, and remove Zustand/ProseMirror-specific patterns.

**`.claude/agents/test-runner.md`** — Replace commands:
```yaml
---
name: test-runner
description: Runs tests and reports failures clearly.
tools: Read, Bash
skills: []
---
```
Replace `pnpm test` / `pnpm check:all` / `cargo test` with your commands.

The other 6 agents (spec-guardian, impact-analyst, auditor, manual-test-author, verifier, release-steward) are mostly generic — copy with minimal changes.

#### Step 7: Create the feature workflow command

**`.claude/commands/feature-workflow.md`** — Copy from vmark, replace:
- `pnpm check:all` → your gate command
- `cargo test` → your backend test command (or remove)
- Tauri MCP E2E → your E2E approach (or "manual testing")
- Keep the 9-step structure intact

#### Step 8: Create the `/fix` command

**`.claude/commands/fix.md`** — Copy from vmark, replace:
- Test commands
- Project-specific conventions references
- File-size references (Rust, shortcuts, etc.)

### Phase 3: Skills (Day 1-2 — As Needed)

#### Step 9: Copy the planning skill

This is generic and works for any project:

```
.claude/skills/planning/
├── SKILL.md          # Planning workflow (quick-plan vs full-plan)
└── templates/
    └── TEMPLATE.md   # Work Item template
```

Copy these directly from vmark. They define how plans are structured and where they're saved (e.g., `dev-docs/plans/YYYYMMDD-HHMM-<topic>.md`).

#### Step 10: Copy plan-audit and plan-verify skills

```
.claude/skills/plan-audit/SKILL.md    # Audit work against a plan
.claude/skills/plan-verify/SKILL.md   # Verify completion with gates
```

These are generic — copy as-is, only replace gate commands in plan-verify.

#### Step 11: Create domain-specific skills (later, as needed)

Only create these when you repeatedly need Claude to know about a specific technology. Examples:
- `skills/nextjs-app/SKILL.md` — Next.js patterns, App Router conventions
- `skills/supabase-backend/SKILL.md` — RLS policies, auth patterns
- `skills/react-native/SKILL.md` — Expo workflow, platform-specific code

### Phase 4: Optional Enhancements (Day 2+)

#### Step 12: (Optional) Prompt refinement hook

If you work in a multilingual environment or want prompt optimization:

```bash
cd .claude/hooks
npm init -y
npm install @anthropic-ai/claude-agent-sdk
```

Copy `refine_prompt.mjs` from vmark. Create `project-context.txt` with a brief description of your project.

Add to `settings.json`:
```json
{
  "hooks": {
    "UserPromptSubmit": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "node \"$CLAUDE_PROJECT_DIR\"/.claude/hooks/refine_prompt.mjs",
            "timeout": 60
          }
        ]
      }
    ]
  }
}
```

#### Step 13: (Optional) Cross-model audit with Codex

If you want a second AI opinion:
```bash
npm install -g @openai/codex
codex login
```

Create `.mcp.json` at project root:
```json
{
  "mcpServers": {
    "codex": {
      "command": "codex",
      "args": ["mcp-server"]
    }
  }
}
```

#### Step 14: Create `.claude/README.md`

Document your `.claude/` folder structure so future contributors understand it.

---

## Real-World Example: Document History Feature

This traces a real feature through the vmark commit history, showing how the `.claude` tools, agents, and workflow were used in practice.

### The Feature

**Goal**: Improve VMark's document history system — add merge window for auto-save snapshots, file size guard, and 4-level history clearing.

**Origin**: The vmark author had a **research conversation** with Claude (preserved in `dev-docs/important-history/a conversation of features.md`). The conversation followed this exact pattern:

### Step 1: Research Phase (User → Planner agent role)

The user asked Claude to **evaluate VMark's file history against industry conventions**. Claude:

1. Launched an **Explore** subagent to trace VMark's current history implementation
2. Launched a **coding-researcher** subagent to research VSCode, JetBrains, Sublime patterns
3. Produced a detailed comparison table and **5 prioritized gap recommendations**:
   - **High**: Add merge window (30s) for auto-save snapshots
   - **Medium**: Add `historyMaxFileSize` setting (512KB)
   - **Low**: Focus-change auto-save, timeline integration, etc.

**Key insight**: The Planner agent's "Research Phase (mandatory for new features)" rule was followed — industry best practices were researched before any code was written.

### Step 2: Plan Phase (User says "devise a plan, implement all")

The user gave a single instruction: "First devise a plan, implement all, with highest quality."

This triggered the planning workflow. The plan was written to a local plan file (these are in `dev-docs/plans/` but gitignored for privacy).

### Step 3: Implementation — Commit by Commit

Each commit maps to a Work Item from the plan:

| Commit | Type | What It Did | Agent Role |
|--------|------|-------------|------------|
| `6a260c0` | `feat:` | Add merge window and file size guard to document history | **Implementer** — core feature, new settings UI |
| `0bf94ad` | `fix:` | Harden history snapshot — byte-size guard, safe merge window, unique IDs | **Auditor** → **Implementer** loop — 17 test cases added covering CJK bytes, boundary, clock jump |
| `dcc4e2f` | `feat:` | Add 4-level history clearing — single snapshot, document, workspace, all | **Implementer** — second Work Item |
| `936ffe0` | `fix:` | Harden history clearing — shared mutation guard, re-entry protection | **Auditor** → **Implementer** loop — concurrent access, cross-platform paths |

### What This Reveals About the Workflow

1. **Each `feat:` commit = one Work Item** — The Release Steward pattern: one commit per WI.
2. **Each `fix:` commit = an audit loop result** — The Auditor found issues (XSS, race conditions, edge cases), which triggered the Implementer to fix them.
3. **Tests came first** — The hardening commit (`0bf94ad`) explicitly mentions "Expand test coverage to 17 cases" — this is the RED phase.
4. **Edge cases were exhaustive** — CJK byte sizes, clock jumps, concurrent access, cross-platform paths — matching the Planner's requirement to "brainstorm exhaustively."

### Timeline

```
Day 1 (Feb 15):
  1. Research conversation (Planner research phase)
  2. User says "plan + implement"
  3. Plan created with Work Items
  4. WI-1: feat: merge window + file size guard (6a260c0)
  5. Audit → fix: harden snapshot (0bf94ad)
  6. WI-2: feat: 4-level clearing (dcc4e2f)
  7. Audit → fix: harden clearing (936ffe0)
```

All 4 commits landed on the same day. The pattern is:
**Research → Plan → Implement WI → Audit → Fix → Implement next WI → Audit → Fix → Done**

---

## Real-World Example: Audio/Video/YouTube Feature (PR #70)

This is a larger feature (95 files changed, +7790/-2468 lines, 13 commits) that demonstrates the full workflow at scale.

### The Feature

**Goal**: Add audio, video, and YouTube embed support to VMark's editor — block-level nodes, drag-and-drop, MCP tools, source mode decorations, and comprehensive security hardening.

### The 13 Commits — Mapped to Workflow Steps

| # | Commit | Workflow Step | Description |
|---|--------|--------------|-------------|
| 1 | `da15510` | **Implementer (WI-1)** | Core feature: block_video/block_audio nodes, NodeViews, drag-drop, popup editing, YouTube embed, markdown pipeline, sanitizer, toolbar, 163 tests |
| 2 | `fd9d82d` | **Implementer (WI-2)** | Source mode decorations, MCP media tools, documentation |
| 3 | `4679022` | **Meta** | Add Code Comments audit dimension to codex plugin (tooling improvement) |
| 4 | `1e3a8a5` | **Implementer (WI-3)** | Upgrade media popup quality: deferred close, tab-trapping, IME guard |
| 5 | `f44dc15` | **Implementer (refactor)** | Consolidate ~700 lines of duplicated code across image/audio/video |
| 6 | `6471ace` | **Auditor → Implementer** | Harden: XSS prevention, input validation, load race conditions (18 files, 24 high-severity fixes) |
| 7 | `7a82179` | **Auditor → Implementer** | Fix round-trip serialization (CommonMark inline vs block issue) |
| 8 | `f51cdba` | **Auditor → Implementer** | UX fix: single-click → double-click for media popup |
| 9 | `d8f77e8` | **Auditor → Implementer** | Fix native controls scrubber sticking after mouse release |
| 10 | `b94ab6c` | **Implementer (refactor)** | Remove redundant image tooltip plugin |
| 11 | `4df075b` | **Implementer (WI-4)** | Extend source mode preview for audio/video files |
| 12 | `f2b2852` | **Implementer (refactor)** | Unify image popup into media popup for all media types |
| 13 | `458205c` | **Auditor → Implementer** | Final audit fixes: 36 issues (7 High, 13 Medium, 16 Low) — sanitization, path traversal, OOM guards |

### Key Patterns Visible in This PR

1. **Implement → Audit → Fix cycles are real** — Commits 6, 7, 8, 9, 13 are all audit-driven fixes. The Codex mini audit found 36 issues in the final pass alone.

2. **Multiple refactor commits** — Commits 5, 10, 12 are pure refactors (the REFACTOR step of TDD). They extract shared logic, remove duplication, and consolidate APIs — all without behavior changes.

3. **Security hardening is a first-class step** — Commit 6 fixes 24 high-severity issues across 18 files. Commit 13 fixes 36 more. The Auditor agent specifically checks for XSS, path traversal, injection, and resource exhaustion.

4. **Each commit is atomic** — Every commit has a clear scope described in its message. No mixed-purpose commits.

5. **Tests are mentioned explicitly** — Commit 1 mentions "163 tests across 6 test suites, all passing". Commit 13 mentions bumping `EXPECTED_TOOL_COUNT`.

6. **The PR summary includes a manual test plan** — Even after automated tests, the PR includes checkboxes for manual verification (insert video, drag-and-drop, YouTube URL detection).

### The User's Role Throughout

Based on the PR structure, the user likely:
1. Described the feature goal ("add audio, video, YouTube support")
2. Ran `/feature-workflow media-support` (or equivalent)
3. Reviewed the plan and approved
4. Watched 4 implementation WIs execute
5. Approved multiple audit→fix cycles
6. Said "accept + commit" for each commit
7. Pushed the branch and created the PR

---

## How VMark's `.claude/` Was Built

Traced from git history — the `.claude/` folder was **not built incrementally**. It was published as a big-bang commit:

### The Big Bang: 2026-02-12

**Commit `837e247`**: `feat: add AI config, coding guide, and TDD enforcement for public repo`

> Publish shared AI configuration so all contributors — human or AI — work with the same rules, conventions, and quality gates.

This single commit created:
- `AGENTS.md` as single source of truth
- `.claude/` with **12 rules, 9 agents, 18 skills, 7 commands**
- `CODING_GUIDE.md` for contributor onboarding
- Coverage thresholds in `vitest.config.ts`
- `/fix` command adapted for VMark conventions
- `.mcp.json` with Tauri E2E and Codex MCP servers

**Key insight**: The author built the entire `.claude/` infrastructure **in one session** after the project was already mature (v0.4.20+, hundreds of source files). The rules codified existing conventions rather than inventing new ones.

### Evolution After the Big Bang (Feb 12-16)

| Date | Commit | What Changed |
|------|--------|-------------|
| Feb 12 | `9a46ef5` | Added `/fix-issue`, `/audit-fix`, `/merge-prs` commands |
| Feb 13 | `d396075` | Added `/bump` command |
| Feb 13 | `8520a8f` | Added Codex availability ping to all audit commands |
| Feb 13 | `32c7540` | Added explicit high reasoning effort to Codex commands |
| Feb 13 | `710aa95` | Added subscription auth recommendation to AGENTS.md |
| Feb 13 | `29f278c` | **Added prompt refinement hook** (first use of Agent SDK) |
| Feb 13 | `86a72e2` | Improved hook with project context and few-shot examples |
| Feb 14 | `591c21c` | Added comment maintenance rule, project history |
| Feb 14 | `249bd22` | Overhauled slash commands with shared model selection |
| Feb 14 | `02ccf4e` | Unignored dev-docs, tracked documentation in git |
| Feb 14 | `758e91e` | Reformatted AGENTS.md for readability |
| Feb 15 | `5ee4497` | Updated codex commands and Cargo.lock |
| Feb 16 | `491bf0f` | PR #70 added Code Comments audit dimension |
| Feb 16 | `835032d` | Added rule to leave issues open for reporter verification |

**Pattern**: The core was built in one day. Subsequent days added **incremental refinements** — new commands, improved hooks, additional rules. The agents and feature-workflow have been stable since day 1.

### Parallel Worktree Strategy (Advanced)

One particularly notable usage from the history: when the author needed to add AI-maintenance documentation comments to ~400 source files, they used **6 parallel git worktrees** with 6 background Claude agents:

```
vmark-p3a/   ← plugins first half (34 dirs)
vmark-p3b/   ← plugins second half (34 dirs)
vmark-p4a/   ← utils first half (~50 files)
vmark-p4b/   ← utils second half (~65 files) + lib
vmark-p5/    ← components (~67 files) + contexts
vmark-p678/  ← Rust + MCP + export (~70 files)
```

This is **not** the Agent SDK — it's 6 separate Claude Code sessions running simultaneously in different worktrees, each with `max_turns: 150`. The worktree approach provides true filesystem isolation without merge conflicts.

---

## File Inventory

### Files to Copy Directly (universal, not project-specific)

| Source File | Notes |
|------------|-------|
| `.claude/agents/planner.md` | Remove vmark-specific skills references |
| `.claude/agents/spec-guardian.md` | Generic — works as-is |
| `.claude/agents/impact-analyst.md` | Generic — works as-is |
| `.claude/agents/implementer.md` | Replace rules paths, test commands |
| `.claude/agents/test-runner.md` | Replace test commands |
| `.claude/agents/auditor.md` | Generic — works as-is |
| `.claude/agents/verifier.md` | Replace gate commands |
| `.claude/agents/release-steward.md` | Generic — works as-is |
| `.claude/agents/manual-test-author.md` | Replace test paths, remove Tauri refs |
| `.claude/commands/feature-workflow.md` | Replace test/gate commands |
| `.claude/commands/fix.md` | Replace test/gate commands, conventions |
| `.claude/skills/planning/SKILL.md` | Generic planning skill |
| `.claude/skills/planning/templates/TEMPLATE.md` | Plan file template |
| `.claude/skills/plan-audit/SKILL.md` | Generic — works as-is |
| `.claude/skills/plan-verify/SKILL.md` | Replace gate commands |
| `.claude/skills/release-gate/SKILL.md` | Replace gate commands |

### Files to Skip (VMark-specific)

| File | Why |
|------|-----|
| `.claude/rules/20-*` through `50-*` | VMark UI/CSS/component/keyboard patterns |
| `.claude/skills/tiptap-*/`, `tauri-*/`, `rust-*` | Domain-specific to VMark's stack |
| `.claude/commands/codex-*.md` | Only if you don't use Codex CLI |
| `.claude/commands/bump.md` | VMark's 5-file version bump |
| `.claude/hooks/refine_prompt.mjs` | Optional — only for multilingual teams |
| `.claude/scripts/codex-preflight.sh` | Codex CLI-specific |

### Files to Create New

| File | Purpose |
|------|---------|
| `AGENTS.md` | Your project's single source of truth |
| `.claude/rules/00-engineering-principles.md` | Your engineering rules |
| `.claude/rules/10-tdd.md` | Your TDD workflow + test patterns |
| `.claude/README.md` | Document your `.claude/` structure |

---

## Best Practices Summary

### Setup Order (Priority)

1. **`AGENTS.md`** — The single most important file. Without it, agents have no shared context.
2. **`CLAUDE.md`** — Thin pointer (`@AGENTS.md`).
3. **`.claude/rules/`** — Auto-loaded every session. Start with 00 (principles) and 10 (TDD).
4. **`.claude/agents/`** — 9 role definitions for the feature workflow.
5. **`.claude/commands/feature-workflow.md`** — The orchestrator.
6. **`.claude/commands/fix.md`** — For quick bug fixes.
7. **`.claude/skills/planning/`** — Plan creation and templates.
8. **`.claude/settings.json`** — Team settings.
9. **(Later)** Domain-specific skills, hooks, scripts, additional commands.

### When to Use Which Tool

| Scenario | Tool | Example from VMark |
|----------|------|-------------------|
| Small bug fix | `/fix [description]` | `fix: harden history snapshot` (0bf94ad) |
| GitHub issue | `/fix-issue #N` | End-to-end: fetch → branch → fix → audit → PR |
| New feature | `/feature-workflow [name]` | Audio/video support (PR #70, 13 commits) |
| Plan a feature | Planning skill | Research conversation → plan file |
| Audit completed work | Plan-audit skill | Check implementation vs plan's WIs |
| Verify before release | Plan-verify skill | Run gates, check acceptance criteria |
| Code audit (cross-model) | `/codex-audit` | 36 issues found in media support |
| Version bump | `/bump` | `chore: bump version to 0.4.30` |

### Key Lessons from VMark's Git History

1. **Build the infrastructure once, refine incrementally** — The entire `.claude/` was created in one commit (12 rules, 9 agents, 18 skills, 7 commands). Daily refinements added commands and improved hooks.

2. **Audit loops are where quality comes from** — In PR #70, 5 out of 13 commits were audit-driven fixes. The Codex mini audit alone found 36 issues. Without the Auditor agent, these would have shipped.

3. **Research before implementation** — The document history feature started with a structured comparison of VSCode, JetBrains, and Sublime before any code was written.

4. **One commit per Work Item, always** — Every `feat:` commit in the history maps to exactly one logical feature. Every `fix:` commit maps to one audit finding or bug report.

5. **Rules codify existing conventions** — The `.claude/rules/` files were written *after* the project had established patterns, not before. They captured what was already working.

6. **Skills are for repeated domain knowledge** — VMark has skills for TipTap, Tauri, React, Rust, and MCP because those technologies are used daily. Don't create skills preemptively — create them when you find yourself explaining the same patterns repeatedly.

### Complete File Tree for a New Project

```
Project Root:
├── AGENTS.md                              # CREATE: single source of truth
├── CLAUDE.md                              # MODIFY: add @AGENTS.md at top
├── .mcp.json                              # OPTIONAL: MCP server registrations
│
├── .claude/
│   ├── README.md                          # CREATE: describe your .claude/ structure
│   ├── settings.json                      # CREATE: team settings
│   │
│   ├── agents/                            # COPY + MODIFY from vmark
│   │   ├── planner.md
│   │   ├── spec-guardian.md
│   │   ├── impact-analyst.md
│   │   ├── implementer.md
│   │   ├── test-runner.md
│   │   ├── auditor.md
│   │   ├── manual-test-author.md
│   │   ├── verifier.md
│   │   └── release-steward.md
│   │
│   ├── commands/
│   │   ├── feature-workflow.md            # COPY + MODIFY from vmark
│   │   ├── fix.md                         # COPY + MODIFY from vmark
│   │   └── (keep existing speckit.*.md)
│   │
│   ├── rules/
│   │   ├── 00-engineering-principles.md   # CREATE for your project
│   │   ├── 10-tdd.md                      # CREATE for your stack
│   │   └── (add more numbered rules as conventions emerge)
│   │
│   ├── skills/
│   │   ├── planning/                      # COPY from vmark (generic)
│   │   │   ├── SKILL.md
│   │   │   └── templates/
│   │   │       └── TEMPLATE.md
│   │   ├── plan-audit/                    # COPY from vmark (generic)
│   │   │   └── SKILL.md
│   │   ├── plan-verify/                   # COPY + MODIFY from vmark
│   │   │   └── SKILL.md
│   │   └── (keep existing save-context.md, load-context.md)
│   │
│   └── hooks/                             # OPTIONAL
│       ├── package.json
│       ├── refine_prompt.mjs
│       └── project-context.txt
│
├── .gitignore                             # ADD: .claude/settings.local.json
└── dev-docs/plans/                        # CREATE: where plan files are stored
```
