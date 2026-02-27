# codex-claude-init + xiaolai Plugin Marketplace

A production-grade AI development toolkit for Claude Code. This project provides gated, agent-driven workflows for feature development, bug fixing, and code auditing — with optional integration of [xiaolai's plugin marketplace](https://github.com/xiaolai/claude-plugin-marketplace) for deeper enforcement, review, and cross-session intelligence.

---

## Table of Contents

- [What This Project Is](#what-this-project-is)
- [What You Get Out of the Box](#what-you-get-out-of-the-box)
  - [Commands](#commands)
  - [Agents](#agents)
  - [Skills](#skills)
  - [Rules](#rules)
- [What xiaolai's Plugins Add](#what-xiaolais-plugins-add)
- [Plugin Installation](#plugin-installation)
- [Integration Setup](#integration-setup)
  - [Step 1: Install codex-claude-init](#step-1-install-codex-claude-init)
  - [Step 2: Install xiaolai's Plugins](#step-2-install-xiaolais-plugins)
  - [Step 3: Install the Agent SDK Skill](#step-3-install-the-agent-sdk-skill)
  - [Step 4: External Dependencies](#step-4-external-dependencies)
- [Integrated Workflow Recipes](#integrated-workflow-recipes)
  - [Recipe 1: New Feature (Full Pipeline)](#recipe-1-new-feature-full-pipeline)
  - [Recipe 2: Bug Fix with Dual-Model Verification](#recipe-2-bug-fix-with-dual-model-verification)
  - [Recipe 3: Codebase Health Check](#recipe-3-codebase-health-check)
  - [Recipe 4: Inherited Codebase Onboarding](#recipe-4-inherited-codebase-onboarding)
  - [Recipe 5: Pre-Release Gate](#recipe-5-pre-release-gate)
  - [Recipe 6: GitHub Issue Resolution (End-to-End)](#recipe-6-github-issue-resolution-end-to-end)
  - [Recipe 7: Agent SDK Development](#recipe-7-agent-sdk-development)
- [Architecture: How Everything Connects](#architecture-how-everything-connects)
  - [Layer Diagram](#layer-diagram)
  - [Integration Map](#integration-map)
  - [Which Tool Covers What](#which-tool-covers-what)
- [Command Reference](#command-reference)
  - [Local Commands (codex-claude-init)](#local-commands-codex-claude-init)
  - [Plugin Commands (xiaolai marketplace)](#plugin-commands-xiaolai-marketplace)
- [When to Use What](#when-to-use-what)
- [Tips & Best Practices](#tips--best-practices)

---

## What This Project Is

`codex-claude-init` is a `.claude/` configuration scaffold — commands, agents, skills, and rules — that turns Claude Code into a structured development environment. It enforces:

- **Plan-before-code** — every non-trivial task starts with research, edge-case brainstorming, and a modular plan
- **TDD discipline** — RED → GREEN → REFACTOR, enforced by rules and agent gating
- **Atomic commits** — one logical change per commit, with mandatory changelog entries
- **Multi-agent orchestration** — 9 specialized agents handle different phases of development
- **Dual-model verification** — Codex CLI serves as an independent auditor catching Claude's blind spots

xiaolai's plugins extend this foundation with **automated enforcement** (hooks that block bad commits), **deeper analysis** (5-agent architecture interrogation), **conversation intelligence** (mining past sessions for lessons), and **file size enforcement** (tokei-based LOC counting).

---

## What You Get Out of the Box

### Commands

| Command | Purpose | When to Use |
|---------|---------|-------------|
| `/feature-workflow [name]` | 9-stage gated pipeline: Plan → Spec Check → Impact → Implement → Test → Audit → Manual Test Guide → Verify → Release | Medium-to-large features |
| `/fix [description]` | Root-cause bug fixing with TDD | Any bug, no half-measures |
| `/audit-fix [scope]` | Audit → fix → verify loop (Claude-only, no Codex needed) | Quick quality sweep |
| `/codex-toolkit:audit --full [scope]` | 9-dimension audit via Codex | Thorough code review |
| `/codex-toolkit:audit --mini [scope]` | Fast 5-dimension audit via Codex | Quick sanity check |
| `/codex-toolkit:audit-fix [scope]` | Audit → fix → verify loop via Codex | Auto-fix with dual-model verification |
| `/codex-toolkit:review-plan` | 5-dimension architectural review of a plan | Before implementing a plan |
| `/codex-toolkit:implement [plan]` | Delegate implementation to Codex | When Codex should build autonomously |
| `/codex-toolkit:verify [report]` | Verify fixes from previous audit | After fixing audit findings |
| `/codex-toolkit:bug-analyze [desc]` | Root cause analysis via Codex | Complex bugs needing fresh perspective |
| `/codex-toolkit:continue [threadId]` | Continue a previous Codex session | Iterate on prior findings |
| `/codex-toolkit:preflight` | Check Codex CLI connectivity | First-time setup verification |
| `/codex-toolkit:init` | Generate `.codex-toolkit.md` config | Project-specific Codex defaults |
| `/fix-issue #N` | End-to-end GitHub issue resolver | Fetch → fix → audit → PR |
| `/merge-prs` | Review and merge open PRs safely | After PRs are created |

### Agents

9 specialized subagents orchestrated by `/feature-workflow`:

| Agent | Phase | Role |
|-------|-------|------|
| **planner** | 1. Plan | Research, edge cases, modular work items with acceptance criteria |
| **spec-guardian** | 2. Spec Check | Validates plan against `AGENTS.md` and `.claude/rules/*.md` |
| **impact-analyst** | 3. Impact | Maps minimal file set, dependency edges, risks per work item |
| **implementer** | 4. Implement | Preflight investigation → RED → GREEN → REFACTOR |
| **test-runner** | 5. Test | Runs unit/integration tests, reports failures |
| **auditor** | 6. Audit | Reviews diffs for correctness, compliance, scope creep |
| **manual-test-author** | 7. Manual Test | Maintains step-by-step manual testing guides |
| **verifier** | 8. Verify | Final checklist: gates passed, criteria met, no data-loss paths |
| **release-steward** | 9. Release | Proposes atomic commits, only commits on explicit user approval |

### Skills

| Skill | Purpose |
|-------|---------|
| `planning` | Creates implementation plans with work items, acceptance criteria, edge cases |
| `plan-audit` | Audits implementation against planned acceptance criteria |
| `plan-verify` | Verifies completed work: runs gates, checks acceptance matrix |
| `release-gate` | Executes full quality gate (lint + test + build) |
| `save-context` | Saves session state when context runs low |
| `load-context` | Restores previous session state |

### Rules

| Rule | Enforces |
|------|----------|
| `00-engineering-principles.md` | Read before edit, focused diffs, ~300 line limit, no bare TODOs, research-first, mandatory edge cases |
| `10-tdd.md` | RED → GREEN → REFACTOR, pattern catalog, assertion quality hierarchy, anti-patterns |

---

## What xiaolai's Plugins Add

Five plugins from [xiaolai/claude-plugin-marketplace](https://github.com/xiaolai/claude-plugin-marketplace), each filling a gap that the local commands/agents don't fully cover:

### grill — Deep Codebase Interrogation

**Gap it fills**: The local `auditor` agent reviews diffs (changed code). `grill` reviews the **entire codebase** architecture from 5 angles simultaneously.

| Component | What It Does |
|-----------|-------------|
| `/grill:grill` command | Orchestrates full codebase interrogation |
| 5 agents | `recon` (survey), `architecture`, `error-handling`, `security`, `testing` |
| 4 review styles | Architecture rewrite, Hard-nosed critique, Multi-perspective panel, ADR style |
| 8 pressure tests | Scale stress, hidden costs, principle violations, strangler fig, success metrics, before/after diagrams, assumptions audit, compact & optimize |

**When to use**: Inheriting codebases, periodic health checks, pre-release architecture reviews, before major refactors.

### tdd-guardian — TDD Enforcement with Hooks

**Gap it fills**: The local `10-tdd.md` rule is a guideline. `tdd-guardian` makes it **structural** — hooks physically block commits without passing gates.

| Component | What It Does |
|-----------|-------------|
| `/tdd-guardian:init` | Auto-detects stack, generates enforcement config |
| `/tdd-guardian:workflow` | 6-agent TDD pipeline (plan → test → implement → coverage → mutation → review) |
| PreToolUse hook | Blocks `git commit`, `git push`, `gh pr create` without fresh gate pass |
| TaskCompleted hook | Auto-runs tests + coverage + mutation when tasks complete |
| Mutation testing | Validates test robustness by introducing code mutations |
| Coverage gates | Default 100% coverage threshold (configurable) |

**When to use**: Projects requiring strict TDD enforcement, teams transitioning to test-first, fintech/healthcare/infrastructure with compliance requirements.

### echo-sleuth — Conversation History Mining

**Gap it fills**: `PROGRESS.md` captures manually-recorded lessons. `echo-sleuth` mines **raw conversation transcripts** for decisions, mistakes, and patterns you forgot to write down.

| Command | What It Does |
|---------|-------------|
| `/echo-sleuth:recall <topic>` | Search past conversations for decisions/mistakes |
| `/echo-sleuth:lessons [topic]` | Extract accumulated wisdom |
| `/echo-sleuth:recap [N]` | Summarize recent sessions |
| `/echo-sleuth:timeline` | Chronological timeline combining sessions + git history |

**When to use**: Onboarding, decision archaeology ("why did we choose X?"), avoiding repeated mistakes, context recovery after breaks.

### loc-guardian — Lines-of-Code Enforcement

**Gap it fills**: `AGENTS.md` says "keep files under ~300 lines." `loc-guardian` **measures and enforces** it with tokei, and suggests concrete refactoring strategies.

| Command | What It Does |
|---------|-------------|
| `/loc-guardian:init` | Configure threshold and extraction rules |
| `/loc-guardian:scan [lang\|path]` | Count pure LOC, flag violations, suggest refactoring |

**When to use**: Proactive file size enforcement, before code reviews, during refactoring sprints.

### codex-toolkit — Codex CLI Integration

**What it provides**: All Codex CLI commands (audit, review, implement, verify, etc.) as plugin commands. Previously these were local command files in `.claude/commands/codex-*.md`, but they have been **migrated to the `codex-toolkit` plugin** (`@xiaolai`) for easier maintenance and updates.

| Command | What It Does |
|---------|-------------|
| `/codex-toolkit:preflight` | Check Codex connectivity and discover models |
| `/codex-toolkit:init` | Generate `.codex-toolkit.md` project config |
| `/codex-toolkit:audit --mini` | Fast 5-dimension audit |
| `/codex-toolkit:audit --full` | Full 9-dimension audit |
| `/codex-toolkit:audit-fix` | Audit → fix → verify loop |
| `/codex-toolkit:review-plan` | 5-dimension architectural review |
| `/codex-toolkit:implement` | Delegate implementation to Codex |
| `/codex-toolkit:verify` | Verify fixes from previous audit |
| `/codex-toolkit:bug-analyze` | Root cause analysis |
| `/codex-toolkit:continue` | Continue a previous Codex session |

**Migration note**: The local `/codex-*` command files have been removed. Use the `/codex-toolkit:*` plugin commands instead. The following local commands remain unique and are **not** covered by any plugin:

| Local Command | Purpose |
|---------------|---------|
| `/audit-fix` | Claude-only audit → fix → verify loop (no Codex needed) |
| `/fix` | Root-cause bug fixing with TDD |
| `/feature-workflow` | 9-stage gated orchestration pipeline |
| `/fix-issue` | End-to-end GitHub issue resolver |
| `/merge-prs` | Safe sequential PR merge |

---

## Plugin Installation

**Install all 5 plugins** — they each add distinct capabilities:

| Plugin | Install? | Reason |
|--------|----------|--------|
| **codex-toolkit** | Yes | Codex CLI integration (audit, review, implement, verify) |
| **grill** | Yes | Whole-codebase architecture review (local auditor only reviews diffs) |
| **tdd-guardian** | Yes | Hook-based enforcement (local rules are guidelines only) |
| **echo-sleuth** | Yes | Conversation mining (no local equivalent) |
| **loc-guardian** | Yes | Automated LOC enforcement with tokei (local rule is manual) |

---

## Integration Setup

### Step 1: Install codex-claude-init

Copy the `.claude/` directory, `CLAUDE.md`, and `AGENTS.md` into your project root:

```bash
# Clone this repo
git clone https://github.com/your-repo/codex-claude-init /tmp/codex-claude-init

# Copy configuration files into your project
cp -r /tmp/codex-claude-init/.claude /path/to/your/project/
cp /tmp/codex-claude-init/CLAUDE.md /path/to/your/project/
cp /tmp/codex-claude-init/AGENTS.md /path/to/your/project/
```

### Step 2: Install xiaolai's Plugins

Inside Claude Code:

```bash
# Add the marketplace (one-time)
/plugin marketplace add xiaolai/claude-plugin-marketplace

# Install all 5 recommended plugins
claude plugin install codex-toolkit@xiaolai --scope user
claude plugin install grill@xiaolai --scope user
claude plugin install tdd-guardian@xiaolai --scope user
claude plugin install echo-sleuth@xiaolai --scope user
claude plugin install loc-guardian@xiaolai --scope user
```

**Scope guidance**:

| Plugin | Recommended Scope | Why |
|--------|-------------------|-----|
| codex-toolkit | `--scope user` | Codex CLI integration, useful across projects |
| grill | `--scope user` | Useful for any codebase |
| echo-sleuth | `--scope user` | Mines your personal session history |
| tdd-guardian | `--scope project` | Enforcement policy should be a team decision |
| loc-guardian | `--scope project` | LOC threshold is project-specific |

### Step 3: Install the Agent SDK Skill (Optional)

Only needed if you build Claude Agent SDK applications:

```bash
git clone https://github.com/xiaolai/claude-agent-sdk-skill-autoupdated \
  ~/.claude/skills/claude-agent-sdk-skill-autoupdated

# Optional: auto-update daily
(crontab -l 2>/dev/null; echo "0 9 * * * cd ~/.claude/skills/claude-agent-sdk-skill-autoupdated && git pull -q") | crontab -
```

### Step 4: External Dependencies

```bash
# Codex CLI (required for /codex-* commands)
npm install -g @openai/codex
codex login  # Prefer ChatGPT Plus/Pro subscription over API key

# tokei (required for loc-guardian)
brew install tokei  # macOS
# or: cargo install tokei, apt install tokei, etc.
```

Verify everything works:

```bash
claude  # Restart Claude Code

# Inside Claude Code:
/codex-toolkit:preflight        # Check Codex connectivity
/tdd-guardian:init              # Set up TDD enforcement for your project
/loc-guardian:init              # Set up LOC thresholds
```

---

## Integrated Workflow Recipes

### Recipe 1: New Feature (Full Pipeline)

The most thorough path, combining the local 9-agent workflow with plugin enforcement and review.

```
# 1. Check past lessons on similar work
/echo-sleuth:recall <feature-area>

# 2. Run the 9-stage gated workflow
/feature-workflow <feature-name>
   │
   ├── Stage 1: Planner agent (research + plan + edge cases)
   ├── Stage 2: Spec Guardian agent (validate plan vs rules)
   ├── Stage 3: Impact Analyst agent (minimal change set)
   ├── Stage 4: Implementer agent (RED → GREEN → REFACTOR)
   │     └── tdd-guardian hooks auto-run tests on task completion
   ├── Stage 5: Test Runner agent
   ├── Stage 6: Auditor agent (diff review)
   ├── Stage 7: Manual Test Author agent
   ├── Stage 8: Verifier agent (final checklist)
   └── Stage 9: Release Steward agent (atomic commits)

# 3. Ask user: "Run /codex-toolkit:review-plan for architectural review?" (checkpoint)
/codex-toolkit:review-plan

# 4. Ask user: "Run /codex-toolkit:audit --mini for quality scan?" (checkpoint)
/codex-toolkit:audit --mini

# 5. LOC enforcement check
/loc-guardian:scan

# 6. Create PR and merge
/merge-prs
```

**What each layer contributes**:

| Layer | Component | Contribution |
|-------|-----------|--------------|
| Local agents | planner, impact-analyst | Plan + impact analysis (no plugin equivalent) |
| Local agents | implementer, auditor | TDD implementation + diff review |
| tdd-guardian hooks | PreToolUse, TaskCompleted | Blocks commits without passing gates |
| codex-toolkit plugin | `/codex-toolkit:audit --mini` | Independent 5-dimension quality check |
| loc-guardian | `/loc-guardian:scan` | File size enforcement |

### Recipe 2: Bug Fix with Dual-Model Verification

```
# 1. Check if this has happened before
/echo-sleuth:recall <bug-description>

# 2. Root-cause fix with TDD
/fix <bug-description>
   │
   ├── Reproduce → Diagnose → RED (test) → GREEN (fix) → REFACTOR → Verify
   └── tdd-guardian hooks block commit until tests pass

# 3. Independent audit
/codex-toolkit:audit-fix

# 4. LOC check (did the fix bloat any file?)
/loc-guardian:scan

# 5. Record the lesson
# (PROGRESS.md is updated manually per AGENTS.md rules)
```

### Recipe 3: Codebase Health Check

Combines grill's architecture review with local auditing and LOC enforcement.

```
# 1. Deep architecture review (5 agents, 4 styles, 8 pressure tests)
/grill:grill
   │
   ├── recon agent → survey
   ├── Choose review style (e.g., "Hard-Nosed Critique + Roadmap")
   ├── Select add-ons (e.g., "Scale stress" + "Hidden costs" + "Compact & optimize")
   └── 4 deep-dive agents in parallel → synthesized report

# 2. File size audit
/loc-guardian:scan

# 3. Full 9-dimension code audit on problem areas
/codex-toolkit:audit --full

# 4. Compile action items from all three reports
```

### Recipe 4: Inherited Codebase Onboarding

When joining a project you didn't build:

```
# 1. Architecture understanding
/grill:grill
   └── Choose "Multi-Perspective Panel" for diverse expert opinions

# 2. What happened before (if prior Claude Code sessions exist)
/echo-sleuth:timeline
/echo-sleuth:lessons

# 3. File size landscape
/loc-guardian:scan

# 4. Security-focused audit
/codex-toolkit:audit --full

# 5. Document findings
# → Create PROGRESS.md with initial observations
# → Create docs/architecture/ with grill findings
```

### Recipe 5: Pre-Release Gate

```
# 1. TDD enforcement check
/tdd-guardian:workflow
   └── coverage-auditor + mutation-auditor validate test quality

# 2. Full codebase audit
/codex-toolkit:audit --full

# 3. Architecture review of recent changes
/grill:grill
   └── Choose "ADR Style" for decision documentation

# 4. LOC compliance
/loc-guardian:scan

# 5. Verify against plan
# (Uses local plan-verify skill automatically)
```

### Recipe 6: GitHub Issue Resolution (End-to-End)

```
# 1. Check context from past sessions
/echo-sleuth:recall <issue-keywords>

# 2. Resolve the issue (full pipeline: fetch → classify → fix → audit → PR)
/fix-issue #123
   │
   ├── Fetch & classify (bug/feature/question)
   ├── Branch setup
   ├── Resolve with TDD
   │     └── tdd-guardian hooks enforce test-first
   ├── Codex audit loop (up to 3 iterations)
   ├── Gate (tests must pass)
   └── Create PR

# 3. LOC check on changed files
/loc-guardian:scan

# 4. Merge
/merge-prs
```

### Recipe 7: Agent SDK Development

Requires the [Agent SDK Skill](https://github.com/xiaolai/claude-agent-sdk-skill-autoupdated) to be installed.

```
# 1. The SDK skill activates automatically when editing *agent*.ts or *agent*.py
#    - Provides current API reference (TS v0.2.59, Python v0.1.44)
#    - Auto-corrects deprecated patterns
#    - Warns about known issues

# 2. Plan the agent
/feature-workflow my-agent

# 3. During implementation, tdd-guardian enforces test-first
# 4. After implementation, audit with Codex
/codex-toolkit:audit --full

# 5. LOC check
/loc-guardian:scan
```

---

## Architecture: How Everything Connects

### Layer Diagram

```
┌────────────────────────────────────────────────────────────────┐
│                         Your Project                            │
│                                                                  │
│  ┌────────────────────────────── Layer 1: Enforcement ────────┐ │
│  │  tdd-guardian hooks         loc-guardian thresholds          │ │
│  │  (block commits/pushes)     (flag oversized files)          │ │
│  └─────────────────────────────────────────────────────────────┘ │
│                                                                  │
│  ┌────────────────────────────── Layer 2: Workflows ──────────┐ │
│  │  /feature-workflow  (9 agents, gated)                       │ │
│  │  /fix               (root-cause TDD)                        │ │
│  │  /fix-issue         (GitHub issue → PR)                     │ │
│  │  /merge-prs         (safe sequential merge)                 │ │
│  └─────────────────────────────────────────────────────────────┘ │
│                                                                  │
│  ┌────────────────────────────── Layer 3: Auditing ───────────┐ │
│  │  /codex-toolkit:audit  (5 or 9 dimensions via Codex)       │ │
│  │  /audit-fix            (Claude-only fallback)               │ │
│  │  /grill:grill         (5-agent architecture interrogation)  │ │
│  │  /loc-guardian:scan   (pure LOC measurement)                │ │
│  └─────────────────────────────────────────────────────────────┘ │
│                                                                  │
│  ┌────────────────────────────── Layer 4: Intelligence ───────┐ │
│  │  /echo-sleuth:recall   (search past decisions)              │ │
│  │  /echo-sleuth:lessons  (extract accumulated wisdom)         │ │
│  │  /echo-sleuth:timeline (project history)                    │ │
│  │  PROGRESS.md           (manually-recorded lessons)          │ │
│  └─────────────────────────────────────────────────────────────┘ │
│                                                                  │
│  ┌────────────────────────────── Layer 5: Rules ──────────────┐ │
│  │  00-engineering-principles.md   (working agreement)         │ │
│  │  10-tdd.md                      (TDD patterns + catalog)    │ │
│  │  AGENTS.md                      (shared AI instructions)    │ │
│  └─────────────────────────────────────────────────────────────┘ │
│                                                                  │
│  ┌────────────── Optional ────────────────────────────────────┐ │
│  │  Agent SDK Skill  (auto-updating SDK reference + rules)     │ │
│  └─────────────────────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────────────────────┘
```

### Integration Map

How the pieces call each other:

```
/feature-workflow
  ├── planner agent ─────────── uses planning skill
  ├── spec-guardian agent ───── reads .claude/rules/*.md, AGENTS.md
  ├── impact-analyst agent ──── reads codebase, maps dependencies
  ├── implementer agent ─────── writes code (RED → GREEN → REFACTOR)
  │     └── tdd-guardian hooks ── auto-validate on task completion
  ├── test-runner agent ─────── runs project gate command
  ├── auditor agent ─────────── reviews diffs
  ├── manual-test-author agent
  ├── verifier agent ────────── uses plan-verify skill, release-gate skill
  └── release-steward agent ─── proposes atomic commits

/fix
  ├── TDD process (RED → GREEN → REFACTOR)
  │     └── tdd-guardian hooks ── block commit without passing gates
  └── runs project gate command

/fix-issue #N
  ├── calls /fix (for bugs) or /feature-workflow (if too large)
  ├── codex-toolkit:audit loop (3 iterations max)
  │     └── falls back to manual audit if Codex unavailable
  └── creates PR via gh

/grill:grill
  ├── recon agent (survey)
  └── 4 deep-dive agents in parallel
        ├── architecture
        ├── error-handling
        ├── security
        └── testing

/echo-sleuth:*
  └── reads Claude Code session history (~/.claude/projects/*)
```

### Which Tool Covers What

| Concern | Local (codex-claude-init) | Plugin (xiaolai) | Combined Effect |
|---------|--------------------------|-------------------|-----------------|
| **Planning** | planner agent, planning skill, impact-analyst | *(none)* | Plan-driven development with impact analysis |
| **TDD** | 10-tdd.md rule, implementer agent | tdd-guardian hooks + mutation testing | Guidelines become hard enforcement |
| **Code audit (diffs)** | auditor agent, /codex-toolkit:audit | *(none — grill audits whole codebase)* | Changed-code review |
| **Code audit (whole codebase)** | /codex-toolkit:audit --full | /grill:grill (5 agents, 4 styles) | Architecture + code-level dual analysis |
| **File size** | AGENTS.md ~300 line rule | /loc-guardian:scan (tokei) | Guideline becomes measured enforcement |
| **Commit discipline** | release-steward agent, AGENTS.md rules | tdd-guardian PreToolUse hook | Agent proposes + hook blocks bad commits |
| **Bug investigation** | /fix, /codex-toolkit:bug-analyze | /echo-sleuth:recall | Root-cause fix + historical context |
| **Session memory** | save-context/load-context skills, PROGRESS.md | /echo-sleuth:recall/lessons/timeline | Manual + automated memory |
| **Test quality** | 10-tdd.md assertion hierarchy | tdd-guardian mutation testing + coverage gates | Assertion rules + mutation proof |
| **Spec compliance** | spec-guardian agent | *(none)* | Blocks drift from project rules |
| **Manual testing** | manual-test-author agent | *(none)* | Living manual test documentation |
| **GitHub integration** | /fix-issue, /merge-prs | *(none)* | End-to-end issue → PR → merge |
| **Agent SDK** | *(none)* | Agent SDK Skill (auto-updated) | Current API reference + auto-correction |

---

## Command Reference

### Local Commands (codex-claude-init)

#### Core Workflows

| Command | Arguments | Description |
|---------|-----------|-------------|
| `/feature-workflow` | `[work-name]` | 9-stage gated pipeline with specialized agents |
| `/fix` | `[issue description]` | Root-cause bug fixing, no half-measures |
| `/audit-fix` | `[scope]` | Audit → fix → verify loop (Claude-only, no Codex) |

#### GitHub Integration

| Command | Arguments | Description |
|---------|-----------|-------------|
| `/fix-issue` | `#N [#M ...]` | End-to-end: fetch → classify → fix → audit → PR |
| `/merge-prs` | `[#N \| --mine \| --pattern]` | Safe sequential PR merge with rebase handling |

### Plugin Commands (xiaolai marketplace)

#### codex-toolkit

| Command | Arguments | Description |
|---------|-----------|-------------|
| `/codex-toolkit:preflight` | — | Verify Codex connectivity |
| `/codex-toolkit:init` | — | Generate `.codex-toolkit.md` config |
| `/codex-toolkit:audit` | `[scope] [--full \| --mini]` | Code audit via Codex (5 or 9 dimensions) |
| `/codex-toolkit:audit-fix` | `[scope]` | Audit → fix → verify loop via Codex |
| `/codex-toolkit:review-plan` | — | 5-dimension plan review via Codex |
| `/codex-toolkit:implement` | `[plan]` | Delegate implementation to Codex |
| `/codex-toolkit:verify` | `[report]` | Verify fixes from previous audit |
| `/codex-toolkit:bug-analyze` | `[description]` | Root cause analysis via Codex |
| `/codex-toolkit:continue` | `[threadId]` | Continue previous Codex session |

#### grill

| Command | Description |
|---------|-------------|
| `/grill:grill` | Deep multi-angle codebase interrogation (interactive: choose style + add-ons) |

#### tdd-guardian

| Command | Description |
|---------|-------------|
| `/tdd-guardian:init` | Auto-detect stack, generate enforcement config |
| `/tdd-guardian:workflow` | Full TDD pipeline: plan → test → implement → coverage → mutation → review |

#### echo-sleuth

| Command | Arguments | Description |
|---------|-----------|-------------|
| `/echo-sleuth:recall` | `<topic> [--scope current\|all]` | Search past conversations |
| `/echo-sleuth:lessons` | `[topic] [--category decisions\|mistakes\|patterns\|all]` | Extract accumulated wisdom |
| `/echo-sleuth:recap` | `[N-sessions-or-days]` | Summarize recent sessions |
| `/echo-sleuth:timeline` | `[--since YYYY-MM-DD]` | Chronological project timeline |

#### loc-guardian

| Command | Arguments | Description |
|---------|-----------|-------------|
| `/loc-guardian:init` | — | Configure LOC threshold and extraction rules |
| `/loc-guardian:scan` | `[language\|path]` | Count pure LOC, flag violations, suggest refactoring |

---

## When to Use What

| Situation | Command(s) | Why This Combination |
|-----------|------------|----------------------|
| New feature (small) | `/fix` or direct implementation | Overkill to run 9-agent workflow for small changes |
| New feature (medium-large) | `/feature-workflow` + `/codex-toolkit:audit --mini` + `/loc-guardian:scan` | Full planning + implementation + quality gate |
| Bug fix | `/echo-sleuth:recall` → `/fix` → `/codex-toolkit:audit-fix` | Check history → fix properly → verify independently |
| "Is this codebase any good?" | `/grill:grill` + `/loc-guardian:scan` + `/codex-toolkit:audit --full` | Architecture + file size + code quality |
| "Why did we do X?" | `/echo-sleuth:recall X` | Mines conversation history |
| "What happened last week?" | `/echo-sleuth:recap 7d` | Session summaries |
| Before release | `/tdd-guardian:workflow` + `/codex-toolkit:audit --full` + `/grill:grill` | Enforce tests + audit code + review architecture |
| GitHub issue | `/fix-issue #N` | End-to-end pipeline including Codex audit |
| File too long | `/loc-guardian:scan` | Measures pure LOC, suggests extraction |
| Plan review | `/codex-toolkit:review-plan` | 5-dimension architectural review |
| Agent SDK work | Just code (skill activates automatically) | Auto-correction for current SDK patterns |

---

## Tips & Best Practices

### 1. Let Hooks Do the Enforcement

After running `/tdd-guardian:init`, the hooks are active. You don't need to manually check test coverage — attempts to `git commit` without passing gates will be blocked automatically.

### 2. Workflow Checkpoints Are Non-Blocking

The system prompts you at key moments (e.g., "Run `/codex-audit-mini` for quality scan?"). These are suggestions, not requirements. Skip any checkpoint that doesn't apply.

### 3. Use echo-sleuth Before Starting Work

Make it a habit to run `/echo-sleuth:recall <topic>` before tackling any non-trivial task. Past sessions often contain relevant decisions and lessons that save time.

### 4. grill for Architecture, codex-audit for Code

They serve different purposes:
- `/grill:grill` — asks "is the architecture sound?" (reviews structure, patterns, design decisions)
- `/codex-audit` — asks "is the code correct?" (reviews individual files for bugs, security, dead code)

Use both for comprehensive reviews.

### 5. Codex Commands Now Use Plugin Namespace

All Codex commands are provided by the `codex-toolkit` plugin. Use the `/codex-toolkit:*` prefix (e.g., `/codex-toolkit:audit --mini` instead of the old `/codex-audit-mini`).

### 6. TDD Guardian Config Is Project-Specific

Adjust `.claude/tdd-guardian/config.json` per project:
- Lower coverage thresholds for legacy projects (e.g., 70% instead of 100%)
- Enable mutation testing only for critical paths
- Set `gateFreshnessMinutes` based on your iteration speed

### 7. LOC Thresholds Should Match Your Rules

The default in `AGENTS.md` is ~300 lines. The default in loc-guardian is 350. Pick one number and configure both to match:

```bash
/loc-guardian:init
# Set max_pure_loc: 300 to match AGENTS.md
```

### 8. Bypass for Emergency

If tdd-guardian hooks are blocking a hotfix:

```bash
TDD_GUARD_BYPASS=1 claude
```

Use sparingly. Fix tests after the emergency.

---

## Prerequisites Summary

| Dependency | Required For | Install |
|------------|-------------|---------|
| Claude Code v1.0.33+ | Everything | `npm install -g @anthropic-ai/claude-code` |
| Codex CLI v0.101.0+ | `/codex-toolkit:*` commands | `npm install -g @openai/codex && codex login` |
| tokei | loc-guardian | `brew install tokei` (macOS) |
| gh CLI | `/fix-issue`, `/merge-prs` | `brew install gh && gh auth login` |

---

## License

MIT
