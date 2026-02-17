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
6. [Workflow Reference](#workflow-reference)
7. [Agent Reference](#agent-reference)
8. [Command Reference](#command-reference)
9. [Customization Guide](#customization-guide)
10. [FAQ & Troubleshooting](#faq--troubleshooting)

---

## What Is This Toolkit?

`codex-claude-init` is a **starter template** for AI-assisted software development. It provides:

- **`AGENTS.md`** — A shared source-of-truth file that instructs all AI coding tools (Claude Code, Codex CLI, Gemini CLI) how to work on your project
- **9 specialized agents** — Markdown role definitions for planning, implementing, testing, auditing, and releasing code
- **3 slash commands** — `/feature-workflow` (full feature pipeline), `/fix` (bug fixing), `/audit-fix` (code auditing)
- **2 project rules** — Auto-loaded engineering principles and TDD enforcement
- **4 skills** — Planning, plan-audit, plan-verify, and release-gate
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
│   │   ├── audit-fix.md              # Audit → fix → verify loop
│   │   └── speckit.*.md              # SpecKit spec-driven commands
│   │
│   ├── rules/                         # Auto-loaded every session
│   │   ├── 00-engineering-principles.md
│   │   └── 10-tdd.md
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

#### Via `/audit-fix` (manual)

Run an audit on your recent changes:

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

## Workflow Reference

### Which workflow to use

| Situation | Workflow | Command |
|-----------|----------|---------|
| **New feature** (medium-large) | Feature Workflow | `/feature-workflow [name]` |
| **Bug fix** (any size) | Fix | `/fix [description]` |
| **Code audit** | Audit-Fix Loop | `/audit-fix [scope]` |
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

Audit → fix → verify loop. Runs up to 3 iterations.

```
/audit-fix                    # Uncommitted changes
/audit-fix staged             # Staged changes
/audit-fix commit -1          # Last commit
/audit-fix src/auth/          # Specific path
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

### Q: How do I add a second-opinion audit with Codex?

1. Install Codex: `npm install -g @openai/codex && codex login`
2. Add to `.mcp.json`:
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
3. Use `/audit-fix` — it can leverage Codex as an MCP tool for cross-model verification.

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
