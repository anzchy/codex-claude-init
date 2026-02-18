# Codex-Claude-Init Manual

A step-by-step guide for using this starter toolkit to professionally vibe-code a project from zero to finish with AI coding agents.

---

## Table of Contents

1. [What Is This Toolkit?](#what-is-this-toolkit)
2. [Prerequisites](#prerequisites)
3. [Quick Start (5 Steps)](#quick-start)
4. [Project Structure Explained](#project-structure-explained)
5. [Tutorial: Building a Project From Zero to Finish](#tutorial-building-a-project-from-zero-to-finish)
   - [Phase 1: Initialize Your Project](#phase-1-initialize-your-project)
   - [Phase 2: Customize for Your Stack](#phase-2-customize-for-your-stack)
   - [Phase 3: Plan Your Feature](#phase-3-plan-your-feature)
   - [Phase 4: Discuss & Refine the Plan](#phase-4-discuss--refine-the-plan)
   - [Phase 5: Generate Tasks](#phase-5-generate-tasks)
   - [Phase 6: Implement Code (TDD)](#phase-6-implement-code-tdd)
   - [Phase 7: Test & Fix Errors](#phase-7-test--fix-errors)
   - [Phase 8: Audit & Harden](#phase-8-audit--harden)
   - [Phase 9: Commit & Ship](#phase-9-commit--ship)
6. [Codex Integration Guide](#codex-integration-guide)
   - [Why Dual-Model?](#why-dual-model)
   - [Setup](#codex-setup)
   - [Command Map: When to Use Which](#command-map-when-to-use-which)
   - [Workflow 1: Quick Audit After Changes](#workflow-1-quick-audit-after-changes)
   - [Workflow 2: Full Audit Before Release](#workflow-2-full-audit-before-release)
   - [Workflow 3: Audit-Fix-Verify Loop](#workflow-3-audit-fix-verify-loop)
   - [Workflow 4: Bug Root Cause Analysis](#workflow-4-bug-root-cause-analysis)
   - [Workflow 5: Plan Review Before Building](#workflow-5-plan-review-before-building)
   - [Workflow 6: Delegate Implementation](#workflow-6-delegate-implementation)
   - [Workflow 7: End-to-End Issue Resolution](#workflow-7-end-to-end-issue-resolution)
   - [Thread Continuation](#thread-continuation)
   - [Project-Level Configuration](#project-level-configuration)
7. [Workflow Reference](#workflow-reference)
8. [Agent Reference](#agent-reference)
9. [Command Reference](#command-reference)
10. [Customization Guide](#customization-guide)
11. [FAQ & Troubleshooting](#faq--troubleshooting)

---

## What Is This Toolkit?

`codex-claude-init` is a **starter template** for AI-assisted software development. It provides:

- **`AGENTS.md`** — A shared source-of-truth file that instructs all AI coding tools (Claude Code, Codex CLI, Gemini CLI) how to work on your project
- **9 specialized agents** — Markdown role definitions for planning, implementing, testing, auditing, and releasing code
- **15 slash commands** — `/feature-workflow` (full feature pipeline), `/fix` (bug fixing), `/audit-fix` (Claude-only auditing), 10 `/codex-*` commands (dual-model workflows), `/fix-issue` (GitHub issue resolver), `/merge-prs` (PR management)
- **2 project rules** — Auto-loaded engineering principles and TDD enforcement
- **4 skills** — Planning, plan-audit, plan-verify, and release-gate
- **Codex dual-model integration** — Use Codex CLI as an independent auditor/reviewer running in an isolated sandbox to catch hallucinations and blind spots
- **SpecKit integration** — Pre-existing spec-driven development commands

Clone this repo, customize for your project, and start building with professional AI-assisted workflows.

---

## Prerequisites

### Required

| Tool | How to Install | Purpose |
|------|---------------|---------|
| **Claude Code CLI** | `npm install -g @anthropic-ai/claude-code` | Primary AI coding agent |
| **Git** | Pre-installed on macOS/Linux | Version control |
| **Node.js 18+** | `brew install node` or [nodejs.org](https://nodejs.org) | Runtime for tools |

### Recommended

| Tool | How to Install | Purpose |
|------|---------------|---------|
| **Codex CLI** | `npm install -g @openai/codex` | Second-opinion code audits |
| **GitHub CLI** | `brew install gh` | Issue/PR management from `/fix-issue` |

### Authentication

```bash
# Claude Code — use subscription (cheaper than API keys)
claude
# Follow the login prompts, or:
claude setup-token

# Codex CLI (optional)
codex login                    # Use ChatGPT Plus/Pro subscription
# Or fallback:
codex login --with-api-key
```

---

## Quick Start

```bash
# 1. Clone the toolkit
git clone https://github.com/YOUR_USERNAME/codex-claude-init.git my-project
cd my-project

# 2. Remove the toolkit's git history and start fresh
rm -rf .git
git init
git add -A
git commit -m "chore: initialize project from codex-claude-init"

# 3. Customize AGENTS.md for your project (see "Customize for Your Stack")
#    Edit: tech stack, gate commands, project-specific patterns

# 4. Start Claude Code
claude

# 5. Begin working — use slash commands:
#    /feature-workflow my-first-feature
#    /fix some bug description
#    /audit-fix
```

---

## Project Structure Explained

```
.
├── AGENTS.md                          # Single source of truth for all AI agents
├── CLAUDE.md                          # Claude Code entry point (@AGENTS.md)
├── CHANGELOG.md                       # Keep a Changelog format (create on first commit)
├── PROGRESS.md                        # Experience log — lessons learned (create when needed)
│
├── .claude/
│   ├── README.md                      # Documentation for the .claude folder
│   ├── settings.json                  # Team-shared settings
│   ├── settings.local.json            # Personal settings (gitignored)
│   │
│   ├── agents/                        # 9 subagent role definitions
│   │   ├── planner.md                 # Research → modular Work Items
│   │   ├── spec-guardian.md           # Validates plan vs project rules
│   │   ├── impact-analyst.md          # Maps minimal file changes
│   │   ├── implementer.md            # TDD-driven code changes
│   │   ├── test-runner.md             # Runs tests, reports failures
│   │   ├── auditor.md                 # Reviews diffs for correctness
│   │   ├── manual-test-author.md      # Manual testing documentation
│   │   ├── verifier.md               # Final pre-release checklist
│   │   └── release-steward.md         # Commit messages, release notes
│   │
│   ├── commands/                      # Slash commands
│   │   ├── feature-workflow.md        # 9-step gated feature pipeline
│   │   ├── fix.md                     # Root-cause bug fixing
│   │   ├── audit-fix.md              # Audit → fix → verify loop (Claude-only)
│   │   ├── codex-preflight.md        # Check Codex connectivity & models
│   │   ├── codex-init.md             # Generate project-level Codex config
│   │   ├── codex-audit-mini.md       # Fast 6-dimension audit via Codex
│   │   ├── codex-audit.md            # Full 10-dimension audit via Codex
│   │   ├── codex-audit-fix.md        # Audit→fix→verify loop with Codex
│   │   ├── codex-bug-analyze.md      # Root cause analysis via Codex
│   │   ├── codex-review-plan.md      # Architectural plan review via Codex
│   │   ├── codex-implement.md        # Delegate plan to Codex for execution
│   │   ├── codex-verify.md           # Verify fixes from previous audit
│   │   ├── codex-continue.md         # Continue a previous Codex session
│   │   ├── fix-issue.md              # End-to-end GitHub issue resolver
│   │   ├── merge-prs.md              # Safe PR review and merge
│   │   ├── _model-selection.md       # Internal: model selection alias
│   │   ├── shared/
│   │   │   └── model-selection.md    # Internal: dynamic model discovery
│   │   └── speckit.*.md              # SpecKit spec-driven commands
│   │
│   ├── rules/                         # Auto-loaded every session
│   │   ├── 00-engineering-principles.md
│   │   └── 10-tdd.md
│   │
│   ├── scripts/                       # Helper scripts
│   │   └── codex-preflight.sh         # Probes Codex models & auth
│   │
│   └── skills/                        # Loaded on demand
│       ├── planning/                  # Plan creation + templates
│       ├── plan-audit/                # Audit work against a plan
│       ├── plan-verify/               # Verify completion
│       ├── release-gate/              # Quality gate checks
│       ├── save-context.md            # Save session state
│       └── load-context.md            # Restore session state
│
├── .codex/                            # Codex CLI configuration
│   └── prompts/speckit.*.md           # Codex versions of SpecKit commands
│
├── .specify/                          # SpecKit templates & scripts
│   ├── memory/constitution.md         # Project constitution
│   ├── templates/                     # Spec, plan, task, checklist templates
│   └── scripts/                       # Setup and utility scripts
│
└── .gitignore
```

### Key Concepts

| Concept | File(s) | Purpose |
|---------|---------|---------|
| **Rules** | `.claude/rules/*.md` | Auto-loaded every session. Enforce conventions. |
| **Commands** | `.claude/commands/*.md` | User-invocable workflows (slash commands). |
| **Agents** | `.claude/agents/*.md` | Role definitions used by `/feature-workflow`. |
| **Skills** | `.claude/skills/*/SKILL.md` | On-demand capabilities loaded when needed. |
| **AGENTS.md** | Root `AGENTS.md` | Single source of truth all AI tools read. |

---

## Tutorial: Building a Project From Zero to Finish

This walks through building a complete feature — from empty project to shipped code — using the toolkit's workflows.

### Phase 1: Initialize Your Project

#### Step 1.1: Clone and reinitialize

```bash
git clone https://github.com/YOUR_USERNAME/codex-claude-init.git my-awesome-app
cd my-awesome-app
rm -rf .git
git init
```

#### Step 1.2: Set up your project's code

Initialize your project however your stack requires:

```bash
# Examples:
npm init -y                                    # Node.js
npx create-next-app@latest . --typescript      # Next.js
python3 -m venv venv && source venv/bin/activate  # Python
cargo init .                                   # Rust
```

#### Step 1.3: Create initial files

```bash
touch CHANGELOG.md PROGRESS.md
```

Add to `CHANGELOG.md`:
```markdown
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/).

## [Unreleased]
```

#### Step 1.4: First commit

```bash
git add -A
git commit -m "chore: initialize project with codex-claude-init toolkit"
```

### Phase 2: Customize for Your Stack

#### Step 2.1: Edit `AGENTS.md`

Open `AGENTS.md` and update:

1. **Tech stack section** — List your actual stack:
   ```markdown
   ## Tech Stack
   - Next.js 14 (App Router), TypeScript, TailwindCSS
   - Supabase (database + auth + storage)
   - Vitest for testing, Playwright for E2E
   ```

2. **Gate command** — Replace the placeholder with your actual test command:
   ```markdown
   - Run `npm run check:all` for gates.
   ```
   Define this in your `package.json`:
   ```json
   {
     "scripts": {
       "check:all": "npm run lint && npm run test && npm run build"
     }
   }
   ```

3. **Project-specific patterns** — Add architecture patterns unique to your project.

#### Step 2.2: Update `.claude/rules/10-tdd.md`

Replace the generic test patterns with your stack's patterns:

- For **Next.js/React**: Add component testing with React Testing Library
- For **Python**: Add pytest patterns
- For **Rust**: Add `#[test]` patterns
- Update the "Running Tests" section with your actual commands

#### Step 2.3: Update agent test commands

In these files, replace test command references:
- `.claude/agents/test-runner.md` — Your test commands
- `.claude/agents/verifier.md` — Your gate command
- `.claude/commands/feature-workflow.md` — Step 5 test commands

#### Step 2.4: Commit customizations

```bash
git add -A
git commit -m "chore: customize toolkit for [your-stack]"
```

### Phase 3: Plan Your Feature

Now you're ready to build. Start Claude Code:

```bash
claude
```

#### Option A: Full agent pipeline via `/feature-workflow`

For a medium-to-large feature, use the gated workflow. The command takes a **work-name slug** and optionally a reference to an existing plan doc:

```
/feature-workflow user-authentication
```

**What to provide as input** — The Planner agent accepts different levels of detail. The more you give it, the faster and more accurate the plan. Here are 5 tiers, from minimal to comprehensive:

**Tier 1: Just a name (minimal — Planner does all the research)**

```
/feature-workflow user-authentication
```

The Planner will research your codebase, look up industry best practices, brainstorm edge cases, and produce Work Items from scratch. This works but takes longer and may need more revision.

**Tier 2: Name + brief goal (recommended minimum)**

```
/feature-workflow user-authentication

Goal: Add Google and GitHub OAuth login via Supabase.
Users can sign in, sign out, and view their profile.
Protected routes redirect to /login.
```

The Planner now has a clear scope. It will research patterns, identify edge cases, and create Work Items aligned with your stated goal.

**Tier 3: Name + bullet-point rough plan (recommended for most features)**

```
/feature-workflow user-authentication

Goal: Add OAuth authentication via Supabase.

Rough plan:
- Set up Supabase client (server + browser)
- Create /auth/callback route for OAuth redirect
- Add sign-in page with Google and GitHub buttons
- Add sign-out functionality
- Create auth middleware for protected routes
- Add user profile page showing name + avatar
- Handle edge cases: expired tokens, revoked access, network failures

Non-goals:
- No email/password auth (OAuth only)
- No admin roles (just authenticated vs not)
```

This is the sweet spot. You define *what* you want, the Planner structures *how* to build it — filling in acceptance criteria, test requirements, edge cases you missed, and file-level impact analysis.

**Tier 4: Name + existing plan doc (fastest — Planner refines rather than creates)**

First, write a rough plan document:

```bash
mkdir -p docs/plans
cat > docs/plans/user-authentication.md << 'EOF'
# User Authentication Plan

## Outcomes
- Users can sign in with Google or GitHub OAuth
- Protected routes redirect unauthenticated users to /login
- Session persists across page reloads

## Work Items (rough)
1. Supabase client setup — createServerClient, createBrowserClient, env vars
2. OAuth callback route — /auth/callback, token exchange, redirect
3. Sign-in page — Google and GitHub buttons, loading states, error display
4. Auth middleware — check session, redirect, preserve original URL
5. User profile — display name, avatar, sign-out button
6. Error handling — expired tokens, revoked access, network failures

## Constraints
- Next.js 14 App Router, server components where possible
- Supabase Auth, no custom JWT handling
- RLS policies on all database tables
EOF
```

Then reference it:

```
/feature-workflow user-authentication

Existing plan: docs/plans/user-authentication.md
Please refine this into proper Work Items with acceptance criteria and tests.
```

The Planner reads your doc, adds acceptance criteria, test specifications, edge cases, rollback strategies, and risk analysis to each Work Item. This is significantly faster than starting from scratch.

**Tier 5: Name + researched spec with decisions (maximum control)**

If you've already done research and made architectural decisions, pass a detailed spec. This is useful when you've compared approaches (e.g., session-based vs JWT, middleware vs layout-level auth) and want the Planner to *execute* your decisions rather than *evaluate* them:

```
/feature-workflow user-authentication

I've already decided:
- Using Supabase Auth with PKCE flow (not implicit)
- Server-side session via createServerClient in middleware.ts
- Client-side via createBrowserClient in a React context provider
- Protected routes checked in middleware.ts, not in individual layouts
- OAuth callback at /auth/callback using Route Handler
- Profile data from Supabase auth.users (no separate profiles table yet)

See the full spec: docs/plans/user-authentication.md
Refine into Work Items and proceed.
```

**What happens after you provide input:**

Regardless of tier, the Planner agent will:
1. **Research** — Search your codebase for existing patterns, dependencies, and conventions. For new domains, it may search for industry best practices.
2. **Brainstorm edge cases** — Empty input, auth failures, token expiry, race conditions, concurrent sessions, network interruptions, cross-browser differences.
3. **Create Work Items** — Each WI gets: goal, non-goals, acceptance criteria (measurable), tests to write first, files to touch, dependencies, risks, rollback strategy.
4. **Write a plan file** — Saved to `docs/plans/YYYYMMDD-HHMM-user-authentication.md`
5. **Present for your review** — You approve, refine, or reject before implementation begins.

**What makes a good input (summary):**

| Element | Why it helps | Example |
|---------|-------------|---------|
| **Goal** | Scopes what "done" means | "Users can sign in with OAuth" |
| **Rough Work Items** | Gives structure to refine | "1. Client setup 2. Callback route 3. Sign-in page" |
| **Non-goals** | Prevents scope creep | "No email/password auth" |
| **Constraints** | Limits the solution space | "Server components only, no client-side auth state" |
| **Decisions made** | Skips research for settled questions | "Using PKCE flow, not implicit" |
| **References** | Provides context | "Similar to how Vercel's auth template works" |

**What NOT to provide:**

- Don't write implementation details (file contents, code snippets) — that's the Implementer's job
- Don't specify test code — the Planner defines *what* to test, the Implementer writes the actual tests
- Don't over-constrain the architecture unless you've researched it — let the Planner investigate

#### Option B: Spec-driven via SpecKit

For a more structured, document-heavy approach, use SpecKit commands. This creates formal artifacts (spec → plan → tasks) that persist as files:

```
/speckit.specify Add user authentication with OAuth (Google, GitHub).
Users should be able to sign in, sign out, and see their profile.
Protected routes should redirect to login.
```

This creates a detailed specification file. Then generate a plan from it:

```
/speckit.plan
```

Then generate ordered tasks:

```
/speckit.tasks
```

Then optionally clarify ambiguities:

```
/speckit.clarify
```

**When to use SpecKit vs Feature Workflow:**

| Use SpecKit when... | Use Feature Workflow when... |
|--------------------|---------------------------|
| You want formal spec documents | You want agent-driven automation |
| Multiple stakeholders review specs | Solo developer or small team |
| You need GitHub issues from tasks | You want in-session implementation |
| Requirements are still being defined | Requirements are roughly clear |
| You want to iterate on the spec first | You want to go from plan to code in one session |

**Combining both:** You can use SpecKit to define the spec and plan, then hand off to Feature Workflow for implementation:

```
/speckit.specify [description]
/speckit.plan
/feature-workflow user-authentication
# Tell the Planner: "Use the spec at .specify/features/*/spec.md as input"
```

#### Option C: Direct conversation

For smaller features or when you want a lightweight approach:

```
I want to add user authentication to this Next.js app.
It should support Google and GitHub OAuth via Supabase.
Can you create a plan first?
```

Claude will enter Plan Mode, explore your codebase, and propose an approach. This is less structured than Options A/B but works well for features that fit in a single session.

### Phase 4: Discuss & Refine the Plan

After the Planner (or SpecKit) produces a plan, **review it before implementation begins**.

#### What to check:

1. **Scope** — Is it too broad? Can it be broken into smaller phases?
2. **Work Items** — Does each WI have clear acceptance criteria and tests?
3. **Edge cases** — Are empty inputs, auth failures, and race conditions covered?
4. **Dependencies** — Are WIs ordered correctly?

#### How to refine:

```
The plan looks good, but:
1. Let's split WI-003 into two parts — the API route and the UI component separately.
2. Add an edge case for expired OAuth tokens.
3. WI-001 should include setting up the Supabase client first.
```

The Planner will update the plan. When satisfied:

```
Plan looks good. Proceed with implementation.
```

If you used `/feature-workflow`, the **Spec Guardian** agent automatically validates the plan against `AGENTS.md` and `.claude/rules/*.md` before proceeding. If conflicts are found, you'll be asked to resolve them.

### Phase 5: Generate Tasks

#### Via `/feature-workflow` (automatic)

If you used `/feature-workflow`, tasks are already structured as Work Items in the plan. The **Impact Analyst** agent maps each WI to the minimal set of files that need to change.

#### Via SpecKit (explicit)

```
/speckit.tasks
```

This generates an ordered task list from the plan, with dependencies.

To convert tasks into GitHub issues:
```
/speckit.taskstoissues
```

### Phase 6: Implement Code (TDD)

#### Via `/feature-workflow` (automatic)

The **Implementer agent** picks up each Work Item and follows strict TDD:

1. **Preflight investigation** — Traces the call chain, identifies the smallest test seam
2. **RED** — Writes a failing test that describes expected behavior
3. **GREEN** — Writes the minimum code to pass the test
4. **REFACTOR** — Cleans up without changing behavior

You'll see the agent working through each step. It may delegate subtasks to subagents for large diffs.

#### Via manual conversation

```
Let's implement WI-001: Set up Supabase client configuration.
Start with the tests.
```

Claude will:
1. Write failing tests first
2. Implement the minimal code
3. Run tests to confirm they pass

#### Key principles during implementation:

- **Never skip the RED step** — Always write the failing test first
- **Keep diffs small** — One Work Item at a time
- **Keep files under 300 lines** — Split proactively
- **Don't bundle unrelated changes** — Each commit = one logical change

### Phase 7: Test & Fix Errors

#### Automated testing

After each Work Item is implemented, the **Test Runner agent** runs your gate command:

```bash
npm run check:all    # or your project's equivalent
```

If tests fail, the agent reports:
- Which tests failed
- File paths and line numbers
- Suggested next actions

#### Fixing test failures

If a test fails, use `/fix`:

```
/fix The auth callback test fails with "TypeError: Cannot read property 'user' of undefined"
```

The `/fix` command follows the philosophy: **no half measures**:
1. Reproduces the issue
2. Finds the root cause (not just the symptom)
3. Writes a regression test (RED)
4. Fixes properly (GREEN)
5. Runs full gate to verify zero regressions

#### Manual testing

For UI features, the **Manual Test Author** agent produces a testing guide with step-by-step instructions:

```
Run the app and follow the manual test guide at docs/testing/manual-testing-guide.md
```

### Phase 8: Audit & Harden

#### Via `/feature-workflow` (automatic)

The **Auditor agent** reviews all diffs for:
- Correctness and edge cases
- Security vulnerabilities (XSS, injection, path traversal)
- Spec/rule compliance
- Accidental scope creep

If issues are found, it loops back to the **Implementer** for fixes. This cycle repeats until the audit passes.

#### Via `/audit-fix` (Claude-only)

Run a Claude-only audit on your recent changes:

```
/audit-fix                 # Audit uncommitted changes
/audit-fix staged          # Audit staged changes
/audit-fix commit -1       # Audit last commit
/audit-fix src/auth/       # Audit specific directory
```

The audit-fix loop:
1. **Audit** — Scans across 7 dimensions (correctness, edge cases, security, duplicates, dead code, shortcuts, compliance)
2. **Fix** — Fixes every finding (Critical through Low)
3. **Verify** — Confirms all fixes are correct
4. **Loop** — Repeats up to 3 times until zero findings remain

#### Via Codex dual-model audit (recommended for important changes)

For higher confidence, use the Codex-powered commands that run audits in an **isolated second model**. This catches blind spots that a single model would miss:

```
/codex-audit-mini              # Fast 6-dimension audit (logic, duplication, dead code, debt, shortcuts, comments)
/codex-audit                   # Full 10-dimension audit (adds security, performance, compliance, deps, docs)
/codex-audit-fix               # Full audit→fix→verify loop (Claude fixes, Codex verifies — or vice versa)
```

The dual-model advantage: Claude writes the code, Codex reviews it independently. If Codex finds issues, Claude (or Codex) fixes them, and then Codex re-verifies. This cross-model verification catches hallucinations, subtle logic errors, and security issues that self-review misses.

See the [Codex Integration Guide](#codex-integration-guide) for detailed setup and usage.

#### Verification

The **Verifier agent** runs a final check:
- All tests pass
- No data-loss paths introduced
- All acceptance criteria from the plan are satisfied

### Phase 9: Commit & Ship

#### Via `/feature-workflow` (automatic)

The **Release Steward** agent proposes commits:

```
Proposed commits:

1. feat(auth): add Supabase client configuration
   - Set up createServerClient and createBrowserClient
   - Add environment variable validation
   - Tests: 4 passing

2. feat(auth): add Google and GitHub OAuth providers
   - Configure OAuth callback route
   - Add sign-in and sign-out functions
   - Tests: 7 passing

3. feat(auth): add protected route middleware
   - Redirect unauthenticated users to /login
   - Preserve original URL for post-login redirect
   - Tests: 5 passing

Accept and commit? (yes/no)
```

Say `yes` (or "accept + commit") to create the commits.

#### Manual commits

If working outside the workflow:

```bash
# Stage specific files (never git add -A blindly)
git add src/lib/supabase/client.ts src/lib/supabase/client.test.ts

# Commit with conventional format
git commit -m "feat(auth): add Supabase client configuration

- Set up createServerClient and createBrowserClient
- Add environment variable validation
- 4 tests passing"
```

#### Update changelog

Every commit needs a changelog entry:

```markdown
## [Unreleased]

### Added
- Supabase client configuration (abc1234)
- Google and GitHub OAuth providers (def5678)
- Protected route middleware (ghi9012)
```

#### Push and create PR

```bash
git push -u origin feat/user-authentication

# Create PR via GitHub CLI
gh pr create --title "feat: add user authentication" --body "..."
```

---

## Codex Integration Guide

### Why Dual-Model?

When a single AI model writes code and reviews its own output, it has blind spots — it tends to confirm its own logic rather than challenge it. The Codex integration solves this by using **two independent models**:

```
┌──────────────┐         ┌──────────────┐
│  Claude Code │         │  Codex CLI   │
│  (primary)   │────────→│  (reviewer)  │
│              │         │              │
│  Writes code │         │  Reads code  │
│  Fixes bugs  │         │  In sandbox  │
│  Full context│         │  Isolated    │
└──────────────┘         └──────────────┘
       │                        │
       │     ┌──────────┐      │
       └────→│  Result  │←─────┘
             │  Compare │
             └──────────┘
```

**Claude Code** is the primary coder — it has full project context, can edit files, run commands, and manage sessions. **Codex CLI** runs in a read-only sandbox as an independent auditor. It sees the same files but has no shared memory with Claude, so it brings genuinely fresh eyes to every review.

This pattern catches:
- **Hallucinated logic** — Claude generates plausible-looking code that Codex flags as incorrect
- **Missed edge cases** — Different models have different intuitions about boundary conditions
- **Security blind spots** — Cross-model review significantly improves vulnerability detection
- **Stale assumptions** — Codex re-reads files from disk, catching cases where Claude's cached understanding drifted from reality

### Codex Setup

#### Step 1: Install Codex CLI

```bash
npm install -g @openai/codex
```

#### Step 2: Authenticate

```bash
# Preferred: use ChatGPT Plus/Pro subscription (dramatically cheaper for sustained use)
codex login

# Fallback: API key (for light or automated usage)
codex login --with-api-key
```

#### Step 3: Add Codex as MCP server

Add to your project's `.mcp.json` (or Claude Code's MCP settings):

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

#### Step 4: Verify connectivity

Inside Claude Code:

```
/codex-preflight
```

This runs the preflight script that probes which Codex models are available, checks authentication, and reports the results. You should see a list of available models (e.g. `gpt-5.3-codex`, `gpt-5.2-codex`).

#### Step 5 (optional): Initialize project config

```
/codex-init
```

This generates a `.codex-toolkit-for-claude.md` file in your project root that customizes how all `/codex-*` commands behave. It detects your tech stack, asks about audit focus (balanced / security-first / performance-first / quality-first), and sets default model and reasoning effort. You can skip this — all commands work with sensible defaults.

### Command Map: When to Use Which

| Situation | Command | What Happens |
|-----------|---------|-------------|
| Quick sanity check after small changes | `/codex-audit-mini` | Fast 6-dimension audit: logic, duplication, dead code, refactoring debt, shortcuts, code comments |
| Thorough review before release or merge | `/codex-audit` | Full 10-dimension audit: adds security, performance, compliance, dependencies, documentation |
| Fix everything automatically | `/codex-audit-fix` | Audit → fix → verify loop (max 3 rounds). Choose Claude or Codex as fixer. |
| Investigate a stubborn bug | `/codex-bug-analyze` | Codex traces data flow, state management, error handling to find root cause |
| Review a plan before building | `/codex-review-plan` | 5-dimension plan review: consistency, completeness, feasibility, ambiguity, risk |
| Delegate implementation to Codex | `/codex-implement` | Send a plan file; Codex executes it autonomously in sandbox |
| Verify fixes from previous audit | `/codex-verify` | Checks each issue from a previous audit report — FIXED / NOT FIXED / PARTIAL |
| Continue a previous Codex conversation | `/codex-continue` | Resume a thread to iterate on findings, request fixes, or drill deeper |
| Resolve a GitHub issue end-to-end | `/fix-issue #123` | Fetch → classify → branch → TDD fix → Codex audit loop → gate → PR |
| Merge multiple PRs safely | `/merge-prs` | Review, rebase, merge sequentially with conflict handling |

### Workflow 1: Quick Audit After Changes

**When**: You've made some changes and want a quick sanity check before committing.

```
/codex-audit-mini
```

The command will:
1. Ask you to choose a Codex model and reasoning effort
2. Detect uncommitted changes via `git diff`
3. Send each changed file to Codex for 6-dimension review
4. Present a findings table with file:line, severity, and suggested fixes

If no issues: you get a CLEAN verdict. If issues found: fix them manually or run `/codex-audit-fix` to automate.

**Scope options:**
```
/codex-audit-mini                    # Uncommitted changes (default)
/codex-audit-mini staged             # Only staged changes
/codex-audit-mini commit -1          # Last commit
/codex-audit-mini commit -3          # Last 3 commits
/codex-audit-mini src/auth/          # Specific directory
```

### Workflow 2: Full Audit Before Release

**When**: Before merging to main, creating a release, or after a large feature is complete.

```
/codex-audit
```

Same scoping options as mini audit, but covers 10 dimensions:

| # | Dimension | Checks |
|---|-----------|--------|
| 1 | Redundant & Low-Value Code | Dead code, duplicates, unused imports |
| 2 | Security & Risk Management | Injection, XSS, hardcoded secrets, auth issues |
| 3 | Code Correctness & Reliability | Logic errors, race conditions, null deref, resource leaks |
| 4 | Compliance & Standards | Naming conventions, framework best practices |
| 5 | Maintainability & Readability | Complexity, magic numbers, DRY violations |
| 6 | Performance & Efficiency | N+1 queries, O(n^2) algorithms, blocking I/O |
| 7 | Testing & Validation | Coverage gaps, flaky tests |
| 8 | Dependency & Environment Safety | Known CVEs, outdated packages |
| 9 | Documentation & Knowledge Transfer | Missing docs, outdated comments |
| 10 | Code Comments & Headers | Stale comments, missing function docs, TODO rot |

The output is a structured report with severity ratings and a PASS / NEEDS WORK / BLOCKED verdict.

### Workflow 3: Audit-Fix-Verify Loop

**When**: You want issues found AND fixed automatically, with independent verification.

```
/codex-audit-fix
```

This is the most powerful command. It runs a complete cycle:

```
┌─────────┐     ┌───────┐     ┌────────┐
│  Audit  │────→│  Fix  │────→│ Verify │
│ (Codex) │     │ (your │     │ (Codex)│
│         │     │ choice)│     │        │
└─────────┘     └───────┘     └────────┘
     ▲                             │
     │         if issues remain    │
     └─────────────────────────────┘
              (max 3 rounds)
```

**Step-by-step:**

1. Codex audits your changes (mini 6-dim or full 10-dim — you choose)
2. Findings are presented with severity ratings
3. You choose: fix all / fix Critical+High only / stop here
4. You choose who fixes: **Claude** (has full project context, precise edits) or **Codex** (sandboxed, autonomous)
5. Fixes are applied
6. Codex independently verifies each fix: FIXED / NOT FIXED / PARTIAL / REGRESSED
7. If issues remain and rounds < 3: loop back to step 4
8. Final report with full audit trail

**The dual-model magic**: When Claude fixes and Codex verifies (or vice versa), you get genuine cross-validation. One model's hallucination is caught by the other's independent review.

**Scope options:**
```
/codex-audit-fix                     # Mini audit on uncommitted changes
/codex-audit-fix --full              # Full 10-dimension audit
/codex-audit-fix --full src/auth/    # Full audit on specific path
/codex-audit-fix commit -2           # Mini audit on last 2 commits
```

### Workflow 4: Bug Root Cause Analysis

**When**: You have a bug you can't figure out, or you want an independent investigation before spending time debugging.

```
/codex-bug-analyze The login page shows a blank screen after OAuth redirect on Safari
```

Codex independently investigates:
1. **Logic Flow** — Traces execution paths related to your bug description
2. **State Management** — Checks for race conditions, stale state, async issues
3. **Data Flow** — Traces data transformations, type coercion, null propagation
4. **Error Handling** — Finds swallowed exceptions, missing error cases
5. **Edge Cases** — Tests boundary conditions, platform-specific behavior

The output is a structured report with:
- **Root cause** (with confidence level)
- **Contributing factors** with file:line locations
- **Related bugs** found by searching for similar patterns elsewhere
- **Recommended fix** (immediate vs proper)
- **Test cases to add**

After the analysis, use `/codex-continue <threadId>` to drill deeper into any finding.

### Workflow 5: Plan Review Before Building

**When**: You've written a plan (via `/feature-workflow`, SpecKit, or manually) and want an independent architectural review before implementation.

```
/codex-review-plan docs/plans/user-authentication.md
```

Codex evaluates the plan across 5 buildability dimensions:

| # | Dimension | What it checks |
|---|-----------|---------------|
| 1 | Internal Consistency | Do decisions contradict each other? |
| 2 | Completeness | Are error paths, startup/shutdown, edge cases covered? |
| 3 | Feasibility | Can this actually be built as described? API misuse? |
| 4 | Ambiguity | Where would an implementer get stuck? |
| 5 | Risk & Sequencing | Is the build order correct? High-risk items addressed early? |

The output includes a **verdict** (READY TO BUILD / NEEDS REVISION / MAJOR GAPS), top 3 risks, and specific recommendations per finding.

This is useful as a pre-implementation gate: catch fundamental issues before writing any code.

### Workflow 6: Delegate Implementation

**When**: You have a well-defined plan and want Codex to implement it autonomously while you do other things.

```
/codex-implement docs/plans/user-authentication.md
```

This sends the plan to Codex running in a sandboxed environment with write access. Codex:
1. Reads the plan
2. Creates files, installs dependencies, writes code
3. Runs tests and builds
4. Reports what it did (files created/modified, commands run, issues encountered)

After Codex finishes, Claude verifies the results by running `git status`, `git diff`, and your project's test suite.

**When NOT to use this**: For complex features with many interdependencies, Claude's interactive approach (with full project context and your real-time feedback) is more reliable. Use `/codex-implement` for well-scoped, self-contained tasks where the plan is unambiguous.

### Workflow 7: End-to-End Issue Resolution

**When**: You have GitHub issues to resolve and want the full pipeline automated.

```
/fix-issue #123
```

The complete pipeline:
1. **Fetch** — Downloads issue details from GitHub
2. **Classify** — Bug / Feature / Question (by labels or content)
3. **Branch** — Creates `fix/issue-123-description` or `feat/issue-123-description`
4. **Resolve** — Bug path (TDD fix), Feature path (research + TDD), or Question path (post answer as comment)
5. **Codex audit** — Runs up to 3 rounds of audit→fix→verify on changed files
6. **Gate** — Runs your project's test command
7. **PR** — Creates a pull request with audit summary

**Multi-issue mode:**
```
/fix-issue #123 #456 #789
```
Creates parallel worktrees and resolves each issue independently using background agents.

### Thread Continuation

Every Codex command returns a **thread ID**. Codex threads preserve full conversation context, so you can iterate:

```
/codex-continue abc-123-def "Now fix the 3 Critical issues you found"
/codex-continue abc-123-def "Explain the race condition in more detail"
/codex-continue abc-123-def "Run the tests and report results"
```

Threads are **in-memory only** — they're lost when the Codex MCP server restarts (e.g. when you restart Claude Code). If a thread is gone, re-run the original command to start fresh.

### Project-Level Configuration

Run `/codex-init` to generate a `.codex-toolkit-for-claude.md` config file. This customizes all `/codex-*` commands for your specific project:

```markdown
# What you can configure:

## Defaults
- Default model: gpt-5.3-codex
- Default effort: high
- Default audit type: mini
- Default sandbox: workspace-write

## Audit Focus
- Balanced (equal weight across all dimensions)
- Security-first (auth, injection, data exposure flagged as Critical)
- Performance-first (N+1 queries, O(n^2), blocking I/O flagged as High)
- Quality-first (untested paths, high complexity flagged as High)

## Skip Patterns
node_modules/, dist/, build/, coverage/, *.min.js, *.lock, vendor/

## Project-Specific Instructions
"This is a Next.js 14 project using Supabase. Follow existing patterns in src/components/."
```

The config file is optional — all commands work with sensible defaults. Commit it to share settings with your team, or gitignore it for personal preferences.

---

## Workflow Reference

### Which workflow to use

| Situation | Workflow | Command |
|-----------|----------|---------|
| **New feature** (medium-large) | Feature Workflow | `/feature-workflow [name]` |
| **Bug fix** (any size) | Fix | `/fix [description]` |
| **Code audit** (Claude-only) | Audit-Fix Loop | `/audit-fix [scope]` |
| **Quick audit** (dual-model) | Codex Mini Audit | `/codex-audit-mini [scope]` |
| **Full audit** (dual-model) | Codex Full Audit | `/codex-audit [scope]` |
| **Audit + auto-fix** (dual-model) | Codex Audit-Fix | `/codex-audit-fix [scope]` |
| **Bug investigation** (dual-model) | Codex Bug Analysis | `/codex-bug-analyze <desc>` |
| **Plan review** (dual-model) | Codex Plan Review | `/codex-review-plan [file]` |
| **Delegate implementation** | Codex Implement | `/codex-implement <plan>` |
| **Verify audit fixes** | Codex Verify | `/codex-verify <report>` |
| **Continue Codex thread** | Codex Continue | `/codex-continue <threadId>` |
| **Resolve GitHub issues** | Fix Issue | `/fix-issue #N` |
| **Merge open PRs** | Merge PRs | `/merge-prs` |
| **Check Codex connectivity** | Codex Preflight | `/codex-preflight` |
| **Configure Codex for project** | Codex Init | `/codex-init` |
| **Create specification** | SpecKit | `/speckit.specify [description]` |
| **Generate plan from spec** | SpecKit | `/speckit.plan` |
| **Generate tasks from plan** | SpecKit | `/speckit.tasks` |
| **Implement tasks** | SpecKit | `/speckit.implement` |
| **Clarify ambiguities** | SpecKit | `/speckit.clarify` |
| **Cross-check artifacts** | SpecKit | `/speckit.analyze` |
| **Save session** | Context Management | `/save-context` |
| **Restore session** | Context Management | `/load-context` |

### Feature Workflow Pipeline (9 steps)

```
┌─────────┐   ┌──────────────┐   ┌────────┐
│ Planner │──→│ Spec Guardian │──→│ Impact │
└─────────┘   └──────────────┘   └────────┘
                                      │
                                      ▼
                               ┌──────────────┐
                               │ Implementer  │◄──┐
                               └──────┬───────┘   │
                                      │            │
                                      ▼            │
                               ┌──────────────┐   │
                               │ Test Runner  │   │
                               └──────┬───────┘   │
                                      │            │
                                      ▼            │
                               ┌──────────────┐   │
                               │   Auditor    │───┘ (loop if issues found)
                               └──────┬───────┘
                                      │
                                      ▼
                         ┌─────────────────────────┐
                         │ Manual Test Author       │
                         └────────────┬────────────┘
                                      │
                                      ▼
                               ┌──────────────┐
                               │  Verifier    │
                               └──────┬───────┘
                                      │
                                      ▼
                         ┌─────────────────────────┐
                         │  Release Steward        │
                         │  (user says "commit")   │
                         └─────────────────────────┘
```

---

## Agent Reference

| Agent | Role | Tools | When Active |
|-------|------|-------|-------------|
| **Planner** | Research, edge cases, creates modular Work Items | Read, Grep | Step 1 of feature workflow |
| **Spec Guardian** | Validates plan against project rules | Read, Grep | Step 2 — blocks if rules violated |
| **Impact Analyst** | Maps minimal correct file changes | Read, Grep | Step 3 — prevents over-scoping |
| **Implementer** | TDD code changes (RED→GREEN→REFACTOR) | Read, Edit, Bash | Step 4 — core implementation |
| **Test Runner** | Runs tests, reports failures | Read, Bash | Step 5 — quality gate |
| **Auditor** | Reviews diffs for correctness/security | Read, Grep | Step 6 — loops back if issues |
| **Manual Test Author** | Creates manual testing guides | Read, Edit, Grep | Step 7 — human testing docs |
| **Verifier** | Final pre-release checklist | Read, Bash | Step 8 — confirms all gates pass |
| **Release Steward** | Proposes atomic commits | Read, Bash | Step 9 — never commits without user approval |

---

## Command Reference

### `/feature-workflow [work-name]`

Full 9-step gated pipeline. Use for medium-to-large features.

```
/feature-workflow user-authentication
/feature-workflow api-rate-limiting
/feature-workflow dark-mode-support
```

### `/fix [description]`

Root-cause bug fixing with TDD. No patches, no shortcuts.

```
/fix Login fails with 401 when token contains special characters
/fix Memory leak in WebSocket connection handler
/fix Race condition in concurrent file uploads
```

### `/audit-fix [scope]`

Audit → fix → verify loop (Claude-only). Runs up to 3 iterations.

```
/audit-fix                    # Uncommitted changes
/audit-fix staged             # Staged changes
/audit-fix commit -1          # Last commit
/audit-fix src/auth/          # Specific path
```

### Codex Commands (Dual-Model)

Setup and connectivity:

```
/codex-preflight               # Check Codex connectivity, list available models
/codex-init                    # Generate .codex-toolkit-for-claude.md project config
```

Auditing (read-only — Codex reviews, doesn't change files):

```
/codex-audit-mini              # Fast 6-dimension audit (logic, duplication, dead code, debt, shortcuts, comments)
/codex-audit-mini staged       # Audit staged changes
/codex-audit-mini commit -3    # Audit last 3 commits
/codex-audit-mini src/auth/    # Audit specific path

/codex-audit                   # Full 10-dimension audit (adds security, performance, compliance, deps, docs)
/codex-audit --full            # Entire codebase
/codex-audit commit -1         # Last commit
```

Audit + auto-fix (Codex audits, Claude or Codex fixes, Codex verifies):

```
/codex-audit-fix               # Mini audit → fix → verify on uncommitted changes
/codex-audit-fix --full        # Full 10-dim audit → fix → verify
/codex-audit-fix --full src/   # Full audit on specific path
```

Investigation and review:

```
/codex-bug-analyze Login fails after OAuth redirect on Safari
/codex-bug-analyze Memory leak in WebSocket handler after 1000+ connections
/codex-review-plan docs/plans/user-auth.md
/codex-review-plan plan.md +AGENTS.md +docs/architecture.md    # Plan + context files
```

Implementation and verification:

```
/codex-implement docs/plans/user-auth.md    # Delegate plan to Codex
/codex-verify audit-report.md               # Verify fixes from previous audit
```

Thread continuation:

```
/codex-continue abc-123 "Fix the Critical issues you found"
/codex-continue abc-123 "Explain the race condition in detail"
/codex-continue abc-123 "Run tests and report"
```

### GitHub Integration Commands

```
/fix-issue #123                # Resolve single issue end-to-end
/fix-issue #123 #456 #789     # Resolve multiple issues in parallel
/merge-prs                     # Merge your open PRs
/merge-prs #12 #34             # Merge specific PRs
/merge-prs --pattern fix/*     # Merge PRs matching branch pattern
```

### SpecKit Commands

```
/speckit.specify [feature description]   # Create specification
/speckit.clarify                         # Find ambiguities in spec
/speckit.plan                            # Generate implementation plan
/speckit.tasks                           # Generate ordered task list
/speckit.implement                       # Execute tasks
/speckit.analyze                         # Cross-check all artifacts
/speckit.checklist                       # Generate QA checklist
/speckit.taskstoissues                   # Convert tasks to GitHub issues
```

### Context Management

```
/save-context       # Save session state before context runs low
/load-context       # Restore previous session state
```

---

## Customization Guide

### Adding new rules

Create numbered markdown files in `.claude/rules/`:

```markdown
# .claude/rules/20-api-design.md

# 20 - API Design Rules

- All API routes must validate input with Zod schemas.
- Error responses must follow RFC 7807 (Problem Details).
- Rate limiting is required on all public endpoints.
```

Rules are auto-loaded every session — no configuration needed.

### Adding new skills

Create a directory with a `SKILL.md`:

```
.claude/skills/my-skill/
├── SKILL.md              # Skill definition (required)
├── references/           # Reference docs (optional)
│   └── patterns.md
└── scripts/              # Helper scripts (optional)
    └── scan.sh
```

### Adding new agents

Create a markdown file in `.claude/agents/`:

```yaml
---
name: security-reviewer
description: Reviews code for security vulnerabilities.
tools: Read, Grep
skills: []
---

You review code for OWASP Top 10 vulnerabilities...
```

Then reference it in `.claude/commands/feature-workflow.md` if you want it in the pipeline.

### Adding new commands

Create a markdown file in `.claude/commands/`:

```markdown
---
description: Run database migrations
argument-hint: "[up|down|status]"
---

# Database Migrations

Run database migrations safely.

## Process
1. Check current migration status
2. ...
```

Usage: `/project:db-migrate up`

---

## FAQ & Troubleshooting

### Q: The feature workflow seems stuck on one step

The workflow pauses at gate boundaries for human approval. Check if it's waiting for you to:
- Approve the plan
- Resolve a spec conflict
- Accept commits

### Q: Tests keep failing in a loop

Check if:
1. Your gate command is correctly defined (`npm run check:all` or equivalent)
2. Test configuration is correct (vitest.config, jest.config, etc.)
3. The failing test is actually testing the right behavior

Use `/fix [error message]` to debug specific failures.

### Q: How do I use this with Codex CLI instead of Claude Code?

The `AGENTS.md` file is tool-agnostic. Codex CLI reads it automatically. The `.claude/` directory is Claude Code-specific, but the agents and rules concepts translate:
- `.codex/prompts/` is Codex's equivalent of `.claude/commands/`
- Codex reads `AGENTS.md` directly

### Q: How do I set up Codex as a second-opinion auditor?

1. Install Codex: `npm install -g @openai/codex`
2. Authenticate: `codex login` (use ChatGPT Plus/Pro subscription)
3. Add to `.mcp.json`:
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
4. Verify: `/codex-preflight` — should show available models
5. (Optional) Configure: `/codex-init` — generates project-specific defaults
6. Start using: `/codex-audit-mini` for quick checks, `/codex-audit-fix` for the full loop

### Q: What's the difference between `/audit-fix` and `/codex-audit-fix`?

`/audit-fix` is **Claude-only** — Claude reads the code, finds issues, fixes them, and verifies its own fixes. It's fast and has full project context, but it's a single model reviewing its own work.

`/codex-audit-fix` is **dual-model** — Codex audits independently in a sandbox, then either Claude or Codex fixes the issues, and Codex re-verifies. The cross-model validation catches blind spots that self-review misses. Use `/codex-audit-fix` when:
- Changes touch security-sensitive code (auth, payments, crypto)
- You're about to merge to main or create a release
- You've been debugging for a while and want fresh eyes
- The change is complex enough that you don't trust a single model's review

### Q: Codex thread expired — what do I do?

Codex threads are in-memory only and are lost when the MCP server restarts. If `/codex-continue <threadId>` fails:
- Re-run the original command (`/codex-audit`, `/codex-bug-analyze`, etc.) to start a fresh thread
- The new thread starts from scratch but reads current file state, so your previous fixes are preserved

### Q: Which Codex model should I pick?

When running a `/codex-*` command, you'll be asked to choose a model. General guidance:

| Model | Best for |
|-------|----------|
| `gpt-5.3-codex` | Most tasks — strongest reasoning, best for audits and bug analysis |
| `gpt-5.2-codex` | Faster, good enough for verification and simple audits |
| `gpt-5-codex-mini` | Quick checks, trivial changes — cheapest and fastest |
| `o4-mini` | When you want fast reasoning at low cost |

If unsure, accept the recommended default. You can also run `/codex-init` to set a project-level default.

### Q: Can I use Codex to implement a plan while I work on something else?

Yes. `/codex-implement plan.md` sends the plan to Codex for autonomous execution in a sandbox. Codex creates files, installs dependencies, writes code, and runs tests. After it finishes, Claude verifies the results. This works best for well-scoped, self-contained tasks where the plan is unambiguous. For complex features with many interdependencies, Claude's interactive approach is more reliable.

### Q: Context is running low during a long session

1. Run `/save-context` — this saves your current session state
2. Start a new session: `claude`
3. Run `/load-context` — this restores where you left off

### Q: How do I skip the feature workflow for a small change?

Just talk to Claude directly:

```
Add a loading spinner to the login button while authentication is in progress.
```

Or use `/fix` for bug fixes. The feature workflow is for medium-to-large features.

### Q: The plan has too many Work Items

Ask Claude to simplify:

```
This plan is too granular. Merge WI-002 and WI-003 into one item.
Keep the total under 5 Work Items.
```

### Q: How do I update the toolkit when new versions are released?

Since you removed the original git history on setup, track the upstream manually:

```bash
# Add upstream remote
git remote add toolkit https://github.com/YOUR_USERNAME/codex-claude-init.git

# Fetch and compare
git fetch toolkit
git diff HEAD toolkit/master -- .claude/
```

Cherry-pick specific improvements as needed.
