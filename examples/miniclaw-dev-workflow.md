# MiniClaw Development Workflow Guide

A practical guide to using the AI toolkit (`codex-claude-init`) for developing MiniClaw. This document explains **when to use what**, maps every tool to a concrete workflow, and gives you a clear path from "I want to build X" to "it's committed and verified."

---

## Table of Contents

1. [Quick Reference: What Do I Use?](#1-quick-reference-what-do-i-use)
2. [The Two Development Tracks](#2-the-two-development-tracks)
3. [Track A: Feature Workflow (Medium-Large Work)](#3-track-a-feature-workflow)
4. [Track B: Fix Workflow (Bugs & Small Changes)](#4-track-b-fix-workflow)
5. [The Speckit Pipeline (Formal Spec-to-Code)](#5-the-speckit-pipeline)
6. [Agent Reference: Who Does What](#6-agent-reference-who-does-what)
7. [Skills Reference: Background Capabilities](#7-skills-reference-background-capabilities)
8. [The Audit-Fix Loop](#8-the-audit-fix-loop)
9. [Session Continuity](#9-session-continuity)
10. [Codex: Your Second-Opinion Coding Consultant](#10-codex-your-second-opinion-coding-consultant)
11. [MiniClaw Implementation Roadmap](#11-miniclaw-implementation-roadmap)
12. [Daily Development Checklist](#12-daily-development-checklist)
13. [Command Cheat Sheet](#13-command-cheat-sheet)

---

## 1. Quick Reference: What Do I Use?

| I want to... | Use this | Section |
|---|---|---|
| Build a new module (memory, skills, etc.) | `/feature-workflow memory-store` | [Section 3](#3-track-a-feature-workflow) |
| Fix a bug or test failure | `/fix [description]` | [Section 4](#4-track-b-fix-workflow) |
| Audit code quality after changes | `/audit-fix` | [Section 8](#8-the-audit-fix-loop) |
| Write a formal spec before building | `/speckit.specify [description]` | [Section 5](#5-the-speckit-pipeline) |
| Generate a technical plan from a spec | `/speckit.plan` | [Section 5](#5-the-speckit-pipeline) |
| Break a plan into ordered tasks | `/speckit.tasks` | [Section 5](#5-the-speckit-pipeline) |
| Execute tasks from a task file | `/speckit.implement` | [Section 5](#5-the-speckit-pipeline) |
| Check spec/plan/task consistency | `/speckit.analyze` | [Section 5](#5-the-speckit-pipeline) |
| Validate requirements quality | `/speckit.checklist [domain]` | [Section 5](#5-the-speckit-pipeline) |
| Clarify ambiguous requirements | `/speckit.clarify` | [Section 5](#5-the-speckit-pipeline) |
| Update project principles | `/speckit.constitution` | [Section 5](#5-the-speckit-pipeline) |
| Convert tasks to GitHub issues | `/speckit.taskstoissues` | [Section 5](#5-the-speckit-pipeline) |
| Run quality gates (pytest+mypy+black) | Use `release-gate` skill | [Section 7](#7-skills-reference-background-capabilities) |
| Save session before context compaction | `/save-context` | [Section 9](#9-session-continuity) |
| Resume after context compaction | `/load-context` | [Section 9](#9-session-continuity) |
| Get a second opinion on code | `/oracle` | [Section 7](#7-skills-reference-background-capabilities) |
| Get Codex to audit my code (full 10-dim) | `/codex-audit [scope]` | [Section 10](#10-codex-your-second-opinion-coding-consultant) |
| Quick code quality check (6-dim) | `/codex-audit-mini [scope]` | [Section 10](#10-codex-your-second-opinion-coding-consultant) |
| Audit → fix → verify loop via Codex | `/codex-audit-fix [scope]` | [Section 10](#10-codex-your-second-opinion-coding-consultant) |
| Send a plan to Codex for review | `/codex-review-plan [plan-file]` | [Section 10](#10-codex-your-second-opinion-coding-consultant) |
| Delegate implementation to Codex | `/codex-implement [plan-file]` | [Section 10](#10-codex-your-second-opinion-coding-consultant) |
| Analyze a bug with Codex | `/codex-bug-analyze [description]` | [Section 10](#10-codex-your-second-opinion-coding-consultant) |
| Continue a Codex session | `/codex-continue [threadId]` | [Section 10](#10-codex-your-second-opinion-coding-consultant) |
| Verify previous audit fixes | `/codex-verify [report-file]` | [Section 10](#10-codex-your-second-opinion-coding-consultant) |
| Check Codex connectivity | `/codex-preflight` | [Section 10](#10-codex-your-second-opinion-coding-consultant) |
| Initialize Codex project config | `/codex-init` | [Section 10](#10-codex-your-second-opinion-coding-consultant) |
| Resolve a GitHub issue end-to-end | `/fix-issue #123` | [Section 10](#10-codex-your-second-opinion-coding-consultant) |
| Merge open PRs safely | `/merge-prs` | [Section 10](#10-codex-your-second-opinion-coding-consultant) |

---

## 2. The Two Development Tracks

The toolkit provides two primary tracks, chosen by the size of the work:

```
                     ┌──────────────────────┐
                     │   What am I doing?   │
                     └──────────┬───────────┘
                                │
              ┌─────────────────┼─────────────────┐
              │                 │                 │
     ┌────────▼────────┐ ┌─────▼──────┐ ┌────────▼────────┐
     │  New module or   │ │  Bug fix   │ │  Large feature   │
     │  feature work    │ │  or tweak  │ │  with spec needs │
     └────────┬────────┘ └─────┬──────┘ └────────┬────────┘
              │                │                 │
     ┌────────▼────────┐ ┌────▼───────┐ ┌───────▼─────────┐
     │ /feature-workflow│ │   /fix     │ │ Speckit Pipeline │
     │  (Track A)       │ │ (Track B)  │ │ (Track A+)      │
     └─────────────────┘ └────────────┘ └─────────────────┘
```

**Rule of thumb:**
- **1-3 files changed, clear scope** → `/fix`
- **New module, multi-file, needs a plan** → `/feature-workflow`
- **Complex feature needing formal spec** → Speckit pipeline, then `/feature-workflow`

---

## 3. Track A: Feature Workflow

**Command:** `/feature-workflow [work-name]`

This is the primary workflow for building MiniClaw modules. It orchestrates 9 specialized agents in a gated pipeline. You don't call the agents individually — the workflow does it for you.

### How It Works

```
/feature-workflow memory-store
        │
        ▼
   ┌─────────┐     ┌──────────────┐     ┌────────────┐
   │ Planner  │────▶│ Spec Guardian │────▶│  Impact    │
   │          │     │              │     │  Analyst   │
   │ Creates  │     │ Validates vs │     │ Maps file  │
   │ plan in  │     │ AGENTS.md,   │     │ boundaries │
   │ docs/    │     │ rules, specs │     │ & risks    │
   │ plans/   │     │              │     │            │
   └─────────┘     └──────────────┘     └──────┬─────┘
                                                │
        ┌───────────────────────────────────────┘
        ▼
   ┌──────────────┐     ┌─────────────┐     ┌──────────┐
   │ Implementer  │────▶│ Test Runner  │────▶│ Auditor  │
   │              │     │              │     │          │
   │ TDD per      │     │ Runs pytest  │     │ Reviews  │
   │ work item:   │     │ + mypy +     │     │ diffs    │
   │ RED→GREEN→   │     │ black gate   │     │ for      │
   │ REFACTOR     │     │              │     │ quality  │
   └──────────────┘     └─────────────┘     └────┬─────┘
                                                  │
                            ┌─────────────────────┘
                            ▼                 If issues found:
                   ┌─────────────────┐        loop back to
                   │ Manual Test     │        Implementer
                   │ Author          │
                   │                 │
                   │ Updates docs/   │
                   │ testing/ guide  │
                   └────────┬────────┘
                            │
                   ┌────────▼────────┐     ┌────────────────┐
                   │ Verifier        │────▶│ Release Steward │
                   │                 │     │                 │
                   │ Final checklist │     │ Proposes commit │
                   │ + gate re-run   │     │ per work item   │
                   └─────────────────┘     └─────────────────┘
```

### Step-by-Step Usage

1. **Start the workflow:**
   ```
   /feature-workflow memory-store
   ```

2. **The Planner creates a plan** in `docs/plans/YYYYMMDD-HHMM-memory-store.md` with:
   - Work Items (WI-001, WI-002, ...) each with goals, tests, acceptance criteria
   - You review and approve before it proceeds

3. **Spec Guardian validates** the plan against `AGENTS.md` rules and `.claude/rules/*.md`

4. **Impact Analyst maps** which files each Work Item touches and identifies risks

5. **Implementer builds each Work Item** using TDD:
   - First does a preflight investigation (traces call chains, identifies test seams)
   - Writes failing tests (RED)
   - Implements minimally (GREEN)
   - Refactors without behavior change (REFACTOR)

6. **Test Runner runs the gate:**
   ```bash
   python3 -m pytest tests/ && python3 -m mypy src/ && python3 -m black --check .
   ```

7. **Auditor reviews** diffs for correctness, edge cases, rule compliance

8. **Manual Test Author** updates `docs/testing/` with human-runnable test steps

9. **Verifier** produces a final pass/fail checklist

10. **Release Steward** proposes atomic commits — one per Work Item — and waits for your explicit "commit" approval

### When to Use

- Implementing `memory.py` (MemoryStore Protocol)
- Implementing `skills.py` (SkillRegistry Protocol)
- Implementing `channels.py` (Channel Protocol + Telegram adapter)
- Implementing `agent.py` (Agent core)
- Implementing `scheduler.py` (Heartbeat + cron)
- Any work touching 3+ files

---

## 4. Track B: Fix Workflow

**Command:** `/fix [description]`

For focused bug fixes and small changes. No plan phase, no multi-agent pipeline — just disciplined root cause analysis with TDD.

### How It Works

```
/fix "FTS5 search returns duplicate results for hyphenated words"
        │
        ▼
   1. Reproduce → Read code, trace the call chain to root cause
   2. Diagnose  → Find the actual bug, check for similar patterns elsewhere
   3. Test (RED) → Write a failing test that captures the bug
   4. Fix (GREEN) → Fix the root cause, not the symptom
   5. Refactor   → Clean up without changing behavior
   6. Verify     → Run full gate command
```

### Principles

- **Fix the cause, not the symptom** — no try/catch wrappers, no special-casing
- **Rewrite if the code is fundamentally flawed** — patching bad code makes worse code
- **Test-first is mandatory** — write the failing test before writing the fix
- **Zero regressions** — full gate must pass before done

### When to Use

- A specific test is failing
- A bug was found during manual testing
- A type error from mypy
- A formatting issue from black
- Small refactors (rename, extract function, simplify logic)

---

## 5. The Speckit Pipeline

The Speckit commands form a linear pipeline for formal specification-driven development. Use this for features that need business-level requirements before coding.

```
/speckit.specify → /speckit.clarify → /speckit.plan → /speckit.tasks → /speckit.analyze → /speckit.implement
       │                 │                  │                │                 │                  │
       ▼                 ▼                  ▼                ▼                 ▼                  ▼
   Write spec      Reduce           Technical plan     Ordered task      Cross-artifact      Execute
   (what, not      ambiguity        (how, data         list with         consistency          tasks
   how)            via Q&A          model, APIs)       dependencies      check                phase by
                                                                                              phase
```

### When to Use the Full Pipeline vs. Feature Workflow

| Situation | Recommendation |
|---|---|
| Clear requirements, known implementation path | Skip Speckit, use `/feature-workflow` directly |
| Vague requirements ("make memory work") | Start with `/speckit.specify`, then proceed through pipeline |
| Multiple possible approaches | Use `/speckit.specify` + `/speckit.clarify` to narrow scope, then `/feature-workflow` |
| Large feature with formal review needs | Full Speckit pipeline |
| Implementing a Protocol from the blueprint | `/feature-workflow` directly — the blueprint IS your spec |

### Command Details

#### `/speckit.specify [description]`
Creates a feature specification from a natural language description. Focuses on **what** (user value, business needs), not **how** (implementation).

- Generates a spec file in `specs/<feature>/spec.md`
- Creates visual diagrams (wireframes, architecture diffs)
- Validates spec quality automatically
- Asks max 3 clarification questions for critical ambiguities

#### `/speckit.clarify`
Refines an existing spec by detecting ambiguities and asking targeted questions (max 5). Answers are integrated directly into the spec file. Run this **before** `/speckit.plan`.

#### `/speckit.plan`
Generates a technical implementation plan from the spec:
- Phase 0: Research and resolve unknowns
- Phase 1: Data model, API contracts, quickstart guide
- Outputs to `specs/<feature>/plan.md`

#### `/speckit.tasks`
Breaks the plan into ordered, dependency-aware tasks:
- Organized by user story (from spec)
- Each task has an ID (T001, T002, ...), file paths, parallel markers
- Outputs to `specs/<feature>/tasks.md`

#### `/speckit.analyze`
**Read-only** consistency check across spec.md, plan.md, and tasks.md:
- Detects duplications, ambiguities, coverage gaps, terminology drift
- Validates against the project constitution
- Produces a severity-ranked findings report
- Run this before `/speckit.implement` to catch problems early

#### `/speckit.checklist [domain]`
Generates requirements quality checklists ("unit tests for English"):
- Not "does the button work?" but "is the button behavior specified clearly?"
- Domains: `ux`, `api`, `security`, `performance`, etc.
- Outputs to `specs/<feature>/checklists/<domain>.md`

#### `/speckit.implement`
Executes all tasks from tasks.md phase by phase:
- Respects dependencies and parallel markers
- Follows TDD (tests before code if configured)
- Marks tasks complete as they finish
- Halts on failure with clear error context

#### `/speckit.constitution`
Updates the project constitution (`.specify/memory/constitution.md`) — the non-negotiable principles governing all development. The current constitution defines:
- Atomic commits, TDD, changelog tracking
- Local-first principle, Python 3.11+ standards
- Protocol contracts as module interfaces

#### `/speckit.taskstoissues`
Converts tasks.md into GitHub Issues (requires GitHub remote and MCP server).

---

## 6. Agent Reference: Who Does What

The 9 agents in `.claude/agents/` are specialized roles used by the `/feature-workflow` orchestrator. You rarely call them directly — but understanding them helps you understand the workflow's feedback.

| Agent | Role | Tools | When Active |
|---|---|---|---|
| **Planner** | Creates modular plans with Work Items | Read, Grep | Step 1 of feature-workflow |
| **Spec Guardian** | Validates plans against rules and specs | Read, Grep | Step 2 — blocks if rules violated |
| **Impact Analyst** | Maps minimal correct file changes per WI | Read, Grep | Step 3 — before implementation |
| **Implementer** | TDD implementation (RED→GREEN→REFACTOR) | Read, Edit, Bash | Step 4 — the coding step |
| **Test Runner** | Runs pytest/mypy/black gate | Read, Bash | Step 5 — after implementation |
| **Auditor** | Reviews diffs for quality & rule compliance | Read, Grep | Step 6 — loops back on issues |
| **Manual Test Author** | Writes human-runnable test guides | Read, Edit, Grep | Step 7 — docs/testing/ |
| **Verifier** | Final checklist before commit | Read, Bash | Step 8 — last gate |
| **Release Steward** | Proposes commits, waits for approval | Read, Bash | Step 9 — never auto-commits |

### Key Behaviors

- **Planner** always does a **research phase** first — searches for best practices, prior art, edge cases before writing a plan.
- **Implementer** always does a **preflight investigation** — traces call chains and identifies test seams before writing code. It can delegate subtasks to subagents.
- **Auditor** checks 7 dimensions: correctness, edge cases, security, duplication, dead code, shortcuts, and rule compliance.
- **Release Steward** never commits without your explicit "commit" — this is a hard rule.

---

## 7. Skills Reference: Background Capabilities

Skills are background capabilities that Claude uses automatically or that you invoke directly:

| Skill | Invocation | What It Does |
|---|---|---|
| **planning** | Auto (used by Planner agent) | Structures Work Items with goals, tests, acceptance criteria, rollback plans |
| **plan-audit** | Ask Claude: "audit the implementation against the plan" | Compares code to plan, finds gaps and logic errors with file:line references |
| **plan-verify** | Ask Claude: "verify the work items" | Runs gates and checks each acceptance criterion as Pass/Fail/Blocked |
| **release-gate** | Ask Claude: "run the release gate" | Runs `python3 -m pytest tests/ && python3 -m mypy src/ && python3 -m black --check .` and reports results |
| **save-context** | `/save-context` | Saves session state to `.claude/contexts/` before context compaction |
| **load-context** | `/load-context` | Restores previous session state after compaction |
| **oracle** | `/oracle` | Sends code + prompt to a second LLM for a second opinion (debugging, architecture review) |

### When to Use plan-audit vs plan-verify

- **plan-audit**: Inspection-only. Reads code and plan, reports gaps. Does NOT run tests. Use for "did we build what we planned?"
- **plan-verify**: Runs tests and gates. Produces Pass/Fail/Blocked matrix. Use for "is the work item actually done?"

Typical flow: audit first (catch logic errors), then verify (confirm gates pass).

---

## 8. The Audit-Fix Loop

**Command:** `/audit-fix [scope]`

A tight loop for code quality: audit → fix all findings → verify → repeat until clean.

### Scope Options

| Input | What Gets Audited |
|---|---|
| *(empty)* | Uncommitted changes |
| `staged` | Staged changes only |
| `commit -1` | Last commit |
| `commit -3` | Last 3 commits |
| `src/miniclaw/memory.py` | Specific file or directory |

### The Loop

```
Phase 1: Scope     → Determine which files to audit
Phase 2: Audit     → Read files, find issues across 7 dimensions
Phase 3: Fix All   → Fix EVERY finding (Critical through Low)
Phase 4: Verify    → Re-read fixed files, check for regressions
Phase 5: Loop/Exit → Zero findings = clean. Findings remain = loop (max 3 iterations)
Phase 6: Gate      → Optionally run full pytest+mypy+black gate
```

### Audit Dimensions

1. **Correctness & logic** — is the code logically sound?
2. **Edge cases** — boundary conditions, None, Unicode, concurrent access
3. **Security** — injection, path traversal, SQL injection
4. **Duplicate code** — copy-paste patterns that should be unified
5. **Dead code** — unused imports, unreachable branches
6. **Shortcuts & patches** — workarounds, TODOs, band-aids
7. **Project compliance** — adherence to `.claude/rules/*.md` and `AGENTS.md`

### When to Use

- After finishing a feature, before committing
- Before creating a pull request
- After a complex refactor
- When you suspect code quality issues in a specific area

---

## 9. Session Continuity

Claude Code sessions have limited context. When context runs low, use these commands to avoid losing progress:

### `/save-context`
**When:** You see context approaching low levels (< 10%).

Saves to `.claude/contexts/YYYY-MM-DD_HH-MM-SS.md`:
- Completed tasks
- In-progress work and next steps
- Key decisions made
- Files modified
- Resume instructions

### `/load-context`
**When:** Starting a new session after compaction.

Loads the most recent context file and presents:
- What was completed
- What's in progress
- Pending tasks
- How to continue

### Best Practice

```
1. Working on memory.py implementation...
2. Context getting low → /save-context
3. [Context compacts]
4. /load-context → picks up where you left off
5. Continue implementing
```

---

## 10. Codex: Your Second-Opinion Coding Consultant

**Philosophy:** Claude Code is your primary coding tool — it writes, reads, and refactors your code with full project context. **Codex** (OpenAI's autonomous coding CLI) serves as an **independent second brain** that runs in isolation. Because Codex has no shared context with Claude, it catches hallucinations, blind spots, and assumptions that Claude might miss.

Think of it as a code review from a colleague who hasn't been staring at the same code all day.

### Prerequisites

1. **Install Codex CLI**: `npm install -g @openai/codex`
2. **Authenticate**: `codex login`
3. **Verify**: `/codex-preflight` — checks connectivity, auth, and available models

### The Dual-LLM Workflow

```
┌────────────────────────────────────────────────────────────┐
│                    YOUR DEVELOPMENT FLOW                    │
│                                                            │
│  Claude Code (Primary)          Codex (Second Opinion)     │
│  ─────────────────────          ─────────────────────      │
│  ● Full project context         ● Isolated sandbox         │
│  ● Reads/writes your files      ● Read-only by default     │
│  ● Knows your architecture      ● Fresh eyes, no bias      │
│  ● Interactive (you guide it)   ● Autonomous (fire & wait) │
│                                                            │
│  Use for:                       Use for:                   │
│  ├── Writing code               ├── Auditing Claude's code │
│  ├── Refactoring                ├── Reviewing your plans   │
│  ├── Debugging                  ├── Independent bug analysis│
│  ├── Planning                   ├── Verifying fixes        │
│  └── Day-to-day dev             └── Catching hallucinations│
└────────────────────────────────────────────────────────────┘
```

### When to Call Codex (Decision Guide)

| Situation | What to Do |
|---|---|
| Just finished implementing a module | `/codex-audit-mini` — quick 6-dim sanity check |
| About to commit a large change | `/codex-audit` — full 10-dim deep audit |
| Wrote a plan, want a second opinion | `/codex-review-plan docs/plans/my-plan.md` |
| Found a bug, need root cause analysis | `/codex-bug-analyze "description of the bug"` |
| Want Codex to fix audit findings | `/codex-audit-fix` — full audit→fix→verify loop |
| Have a plan, want Codex to build it | `/codex-implement plan.md` |
| Previous audit found issues, fixes applied | `/codex-verify audit-report.md` |
| Want to continue a Codex conversation | `/codex-continue <threadId> "follow-up"` |

### Command Reference

#### `/codex-preflight` — Check Codex Readiness

Run this first. Probes available models, checks authentication, reports what's accessible.

```
/codex-preflight
```

Output: Available models, auth mode, Codex version. If anything is wrong, tells you exactly how to fix it.

#### `/codex-init` — Initialize Project Config

Generates `.codex-toolkit-for-claude.md` with project-specific defaults (stack detection, audit focus, skip patterns). Optional — commands work without it.

```
/codex-init
```

#### `/codex-audit [scope]` — Full 10-Dimension Audit

The comprehensive audit. Sends your code to Codex with a 10-dimension checklist:

| Dimension | What It Checks |
|---|---|
| 1. Redundant Code | Dead code, duplicate code, unused imports |
| 2. Security | SQL injection, XSS, path traversal, hard-coded secrets |
| 3. Correctness | Logic errors, race conditions, resource leaks |
| 4. Compliance | Coding standards, framework conventions |
| 5. Maintainability | Complexity, function size, magic numbers |
| 6. Performance | Algorithm efficiency, N+1 queries, blocking I/O |
| 7. Testing | Coverage gaps, flaky tests |
| 8. Dependencies | Known CVEs, outdated packages |
| 9. Documentation | Missing docs, outdated comments |
| 10. Code Comments | Missing headers, stale TODOs, misleading docs |

```
/codex-audit                     # Audit uncommitted changes
/codex-audit commit -1           # Audit last commit
/codex-audit --full              # Audit entire codebase
/codex-audit src/miniclaw/memory.py  # Audit specific file
```

Produces a severity-ranked report with `file:line` locations and suggested fixes. Includes a **Thread ID** for follow-up via `/codex-continue`.

#### `/codex-audit-mini [scope]` — Fast 6-Dimension Audit

Lighter version for quick checks. Focuses on code quality rather than security/performance:

1. Logic & Correctness
2. Duplication
3. Dead Code
4. Refactoring Debt
5. Shortcuts & Patches
6. Code Comments

```
/codex-audit-mini                # Quick check on uncommitted changes
/codex-audit-mini staged         # Check staged changes before commit
```

**When to use mini vs full:**
- Mini: small changes, routine checks, CI-like validation
- Full: security-sensitive code, major features, pre-release

#### `/codex-audit-fix [scope]` — Audit→Fix→Verify Loop

The power command. Combines audit + fix + verification in a loop:

```
/codex-audit-fix                 # Mini audit + fix loop on uncommitted changes
/codex-audit-fix --full          # Full audit + fix loop
/codex-audit-fix src/miniclaw/   # Target specific directory
```

1. Audits your code (mini or full — you choose)
2. Shows findings, asks who should fix: **Claude** (recommended — has full context) or **Codex** (sandboxed, autonomous)
3. Fixes are applied
4. Codex re-verifies that fixes actually resolved the issues
5. Repeats up to 3 times until clean

#### `/codex-review-plan [plan-file]` — Independent Plan Review

Sends a plan document to Codex for architectural review across 5 dimensions:

1. **Internal Consistency** — Do decisions contradict each other?
2. **Completeness** — Missing error paths, edge cases, migration steps?
3. **Feasibility** — Can this actually be built? API mismatches?
4. **Ambiguity** — Where would an implementer get stuck?
5. **Risk & Sequencing** — Is the build order correct?

```
/codex-review-plan docs/plans/20250218-memory-store.md
```

Returns a verdict: **READY TO BUILD / NEEDS REVISION / MAJOR GAPS**.

#### `/codex-implement [plan-file]` — Delegate Implementation to Codex

Hands a plan to Codex for autonomous execution. Codex works in a sandboxed environment.

```
/codex-implement docs/plans/my-plan.md
```

After completion, runs `git status`, `git diff --stat`, and your tests to verify results.

**When to use:** When you want to see how a different model interprets and implements your plan. Compare its output with what Claude would do.

#### `/codex-bug-analyze [description]` — Root Cause Analysis

Describes a bug to Codex and lets it trace through your codebase independently:

```
/codex-bug-analyze "FTS5 search returns duplicate results when query contains hyphens"
```

Codex traces data flow, identifies root cause, finds related bugs with the same pattern, and suggests specific fixes with `file:line` references.

#### `/codex-verify [audit-report]` — Verify Previous Fixes

After fixing issues from a previous audit, run this to confirm they're actually resolved:

```
/codex-verify audit-report.md
```

Reports each issue as: FIXED / NOT FIXED / PARTIAL / MOVED.

#### `/codex-continue [threadId] [prompt]` — Continue Codex Session

Every Codex command returns a **Thread ID**. Use it to continue the conversation:

```
/codex-continue abc-123 "Now fix the 3 Critical issues you found"
/codex-continue abc-123 "Explain the race condition in more detail"
```

> **Note:** Codex threads are in-memory only. They're lost when the MCP server restarts.

#### `/fix-issue #N` — End-to-End GitHub Issue Resolver

Fetches a GitHub issue, classifies it (bug/feature/question), creates a branch, fixes with TDD, runs Codex audit loop, gates, and creates a PR:

```
/fix-issue #42                   # Single issue
/fix-issue #42 #43 #44           # Multiple issues (parallel worktrees)
```

Integrates the full Codex audit loop (Phase 4) into the issue resolution pipeline.

#### `/merge-prs` — Safe PR Merging

Reviews and merges open PRs with rebase handling:

```
/merge-prs                       # Merge your open PRs
/merge-prs #12 #34               # Specific PRs
/merge-prs --pattern fix/issue-* # PRs matching pattern
```

### Recommended Codex Integration Points in Your Workflow

Here's how Codex fits into the existing MiniClaw development workflow:

```
Daily Development with Codex
├── Start: /load-context (if resuming)
│
├── Implement Module
│   ├── /feature-workflow memory-store        # Claude leads implementation
│   ├── /codex-audit-mini                     # Quick Codex sanity check
│   └── Fix any findings
│
├── Before Committing
│   ├── /codex-audit-fix                      # Full audit→fix→verify loop
│   ├── Gate: python3 -m pytest && mypy && black
│   └── Commit
│
├── Planning a New Module
│   ├── Write plan in docs/plans/
│   ├── /codex-review-plan docs/plans/...     # Codex reviews architecture
│   ├── Address findings
│   └── /feature-workflow [name]              # Proceed with confidence
│
├── Debugging a Tricky Bug
│   ├── /codex-bug-analyze "description"      # Codex traces root cause
│   ├── /fix [based on Codex analysis]        # Claude applies the fix
│   └── /codex-verify                         # Codex confirms fix works
│
└── Context Running Low
    └── /save-context
```

### Anti-Hallucination Strategy

The dual-LLM approach specifically targets hallucination:

1. **Claude writes code** → it might hallucinate an API, invent a method, or miss an edge case
2. **Codex audits independently** → fresh eyes, no shared context, catches things Claude assumed
3. **Disagreements are signals** → if Codex flags something Claude wrote confidently, investigate it
4. **Thread continuity** → use `/codex-continue` to drill into specific findings

This is not about replacing Claude — it's about having a skeptical colleague who double-checks the work.

---

## 11. MiniClaw Implementation Roadmap

Based on the blueprint's implementation order, here's the recommended sequence using the toolkit workflows:

### Phase 1: Foundation

```bash
# Step 1: types.py + config.py (already bootstrapped with Protocol stubs)
/feature-workflow types-and-config
# Fill in config loading (env vars, YAML parsing)
# types.py is already done — just verify tests pass

# Step 2: memory.py — the core module, everything else depends on it
/feature-workflow memory-store
# Implement: FTS5 indexing, search, file I/O, pre-compaction flush
# This is the largest and most critical module

# Step 3: skills.py
/feature-workflow skill-registry
# Implement: SKILL.md parsing, directory discovery, gate checks
```

### Phase 2: Communication

```bash
# Step 4: channels.py — Telegram adapter
/feature-workflow telegram-channel
# Implement: Channel protocol, Telegram polling, message normalization

# Step 5: agent.py — connects everything
/feature-workflow agent-core
# Implement: LLM interface, tool dispatch, system prompt assembly
```

### Phase 3: Proactivity

```bash
# Step 6: scheduler.py
/feature-workflow heartbeat-scheduler
# Implement: heartbeat loop, cron jobs, active hours, HEARTBEAT_OK

# Step 7: __main__.py — CLI
/feature-workflow cli-entrypoint
# Implement: miniclaw run, miniclaw memory search
```

### Phase 4: Extension

```bash
# Step 8: WhatsApp adapter (optional)
/feature-workflow whatsapp-channel
```

### Between Modules

After completing each module, run:
```
/audit-fix commit -1    # Audit the last commit
```

Check `PROGRESS.md` before starting the next module — it contains lessons from previous work.

---

## 12. Daily Development Checklist

```
Start of Session
├── git status -sb                          # Check current state
├── /load-context                           # If resuming after compaction
├── Read PROGRESS.md                        # Review past lessons
│
During Development
├── /feature-workflow [name]                # For new modules
│   OR /fix [description]                   # For bugs
├── Follow TDD: RED → GREEN → REFACTOR
├── Gate: python3 -m pytest tests/ && python3 -m mypy src/ && python3 -m black --check .
│
Before Committing
├── /audit-fix                              # Audit uncommitted changes
├── Update CHANGELOG.md                     # Under [Unreleased]
├── Update PROGRESS.md                      # If lessons learned
│
Context Running Low
└── /save-context                           # Before compaction
```

---

## 13. Command Cheat Sheet

### Core Workflows

```
/feature-workflow [name]     # Full gated pipeline for features
/fix [description]           # Disciplined bug fix with TDD
/audit-fix [scope]           # Audit → fix → verify loop
```

### Speckit Pipeline (in order)

```
/speckit.specify [desc]      # Write feature spec (what, not how)
/speckit.clarify             # Reduce spec ambiguity via Q&A
/speckit.plan                # Generate technical plan from spec
/speckit.checklist [domain]  # Validate requirements quality
/speckit.tasks               # Break plan into ordered tasks
/speckit.analyze             # Cross-artifact consistency check
/speckit.implement           # Execute tasks phase by phase
/speckit.taskstoissues       # Push tasks to GitHub Issues
/speckit.constitution        # Update project principles
```

### Session Management

```
/save-context                # Save session state before compaction
/load-context                # Restore session state after compaction
```

### Quality Gate (manual)

```bash
python3 -m pytest tests/ && python3 -m mypy src/ && python3 -m black --check .
```

### Codex Second-Opinion Commands

```
/codex-preflight             # Check Codex connectivity and available models
/codex-init                  # Generate project-specific Codex config
/codex-audit [scope]         # Full 10-dimension audit via Codex
/codex-audit-mini [scope]    # Fast 6-dimension audit via Codex
/codex-audit-fix [scope]     # Audit → fix → verify loop (max 3 rounds)
/codex-review-plan [file]    # Independent plan review (5 dimensions)
/codex-implement [plan]      # Delegate implementation plan to Codex
/codex-bug-analyze [desc]    # Root cause analysis via Codex
/codex-verify [report]       # Verify fixes from a previous audit
/codex-continue [id] [msg]   # Continue a Codex thread
```

### GitHub Integration

```
/fix-issue #123              # End-to-end issue resolver (fetch→fix→audit→PR)
/merge-prs                   # Safely merge open PRs with rebase handling
```

### Second Opinion

```
/oracle                      # Send files to a second LLM for review
```

---

## Appendix: File Map

```
.claude/
├── agents/                  # 9 specialized agent definitions
│   ├── planner.md           # Creates modular plans
│   ├── spec-guardian.md     # Validates plans vs rules
│   ├── impact-analyst.md   # Maps file boundaries
│   ├── implementer.md      # TDD implementation
│   ├── test-runner.md      # Runs quality gates
│   ├── auditor.md          # Code quality review
│   ├── manual-test-author.md # Human test guides
│   ├── verifier.md         # Final verification
│   └── release-steward.md  # Proposes commits
├── commands/                # Slash commands
│   ├── feature-workflow.md  # /feature-workflow
│   ├── fix.md               # /fix
│   ├── audit-fix.md         # /audit-fix
│   ├── speckit.*.md         # /speckit.* pipeline commands
│   ├── codex-audit.md       # /codex-audit (full 10-dim)
│   ├── codex-audit-mini.md  # /codex-audit-mini (fast 6-dim)
│   ├── codex-audit-fix.md   # /codex-audit-fix (audit→fix→verify)
│   ├── codex-review-plan.md # /codex-review-plan
│   ├── codex-implement.md   # /codex-implement
│   ├── codex-bug-analyze.md # /codex-bug-analyze
│   ├── codex-verify.md      # /codex-verify
│   ├── codex-continue.md    # /codex-continue
│   ├── codex-preflight.md   # /codex-preflight
│   ├── codex-init.md        # /codex-init
│   ├── fix-issue.md         # /fix-issue (GitHub issue resolver)
│   ├── merge-prs.md         # /merge-prs (safe PR merging)
│   ├── _model-selection.md  # Shared: dynamic model selection
│   └── shared/
│       └── model-selection.md # Shared partial (non-invocable)
├── rules/                   # Enforced rules
│   ├── 00-engineering-principles.md
│   └── 10-tdd.md            # TDD patterns for Python/pytest
├── skills/                  # Background capabilities
│   ├── planning/            # Plan creation skill
│   ├── plan-audit/          # Plan vs implementation audit
│   ├── plan-verify/         # Gate-based verification
│   ├── release-gate/        # Full quality gate
│   ├── save-context.md      # Session save
│   └── load-context.md      # Session restore
├── scripts/
│   └── codex-preflight.sh   # Probes Codex models and auth
└── settings.json            # Permission config

.specify/
├── memory/
│   └── constitution.md      # Project principles (non-negotiable)
├── templates/               # Spec, plan, task, checklist templates
└── scripts/bash/            # Helper scripts for speckit commands

docs/
├── plans/                   # Planner agent output
└── testing/                 # Manual test guides
```
