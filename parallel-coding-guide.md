# Parallel Coding Guide

How to use this starter toolkit to run multiple features in parallel, split tasks safely, and resolve conflicts — boosting throughput while maintaining code quality.

---

## Table of Contents

1. [Why Parallel Coding?](#why-parallel-coding)
2. [Core Mechanism: Git Worktrees](#core-mechanism-git-worktrees)
3. [Setup: One-Time Preparation](#setup-one-time-preparation)
4. [Step-by-Step: Running 2+ Features in Parallel](#step-by-step-running-2-features-in-parallel)
5. [Task Decomposition: How to Split Work Safely](#task-decomposition-how-to-split-work-safely)
6. [Conflict Prevention: Architecture for Parallelism](#conflict-prevention-architecture-for-parallelism)
7. [Conflict Resolution: When Two Workers Touch the Same File](#conflict-resolution-when-two-workers-touch-the-same-file)
8. [Orchestration Patterns](#orchestration-patterns)
9. [Practical Limits & When to Stop Adding Workers](#practical-limits)
10. [Real-World Example: VMark's 6-Agent Documentation Sprint](#real-world-example)
11. [Quick Reference: Commands Cheat Sheet](#quick-reference)

---

## Why Parallel Coding?

A single Claude Code session implements features sequentially — one at a time. With git worktrees, you can run **N sessions simultaneously**, each on its own branch, isolated from each other.

```
Without parallelism:        With parallelism (3 workers):

Feature A ████████           Feature A ████████
Feature B         ████████   Feature B ████████
Feature C                 ████████  Feature C ████████

Total: 3x time              Total: 1x time (+ merge overhead)
```

**Real-world impact:** The incident.io team went from sequential AI development to running multiple agents in parallel — each working on isolated features with complete environments — and significantly accelerated feature development.

---

## Core Mechanism: Git Worktrees

### What are worktrees?

`git worktree` lets you check out multiple branches into **separate directories** simultaneously. Each directory is a full working copy with its own staged files, but they all share the same git history and object database.

```
my-project/                   # Main worktree (orchestrator)
├── .git/
├── AGENTS.md
├── .claude/
├── src/
└── ...

my-project-wt/                # Sibling directory for worktrees
├── feat-auth/                # Worktree 1: branch feat/auth
│   ├── AGENTS.md             # Full copy of project
│   ├── src/
│   └── ...
├── feat-dashboard/           # Worktree 2: branch feat/dashboard
│   ├── AGENTS.md
│   ├── src/
│   └── ...
└── fix-billing/              # Worktree 3: branch fix/billing
    ├── AGENTS.md
    ├── src/
    └── ...
```

### Why not just `git clone` multiple copies?

| | `git worktree` | Multiple clones |
|-|---------------|----------------|
| Disk space | Shared `.git` objects | Full copy each (including `node_modules`) |
| Git history | Shared, always in sync | Independent, can drift |
| Branch protection | Prevents checking out same branch twice | No protection |
| Cleanup | `git worktree remove` | `rm -rf` |

---

## Setup: One-Time Preparation

### 1. Add worktree directory to `.gitignore`

```bash
echo "*.worktrees/" >> .gitignore
```

### 2. Create a worktree launcher script

Save this as `scripts/worktree-new.sh` in your project:

```bash
#!/usr/bin/env bash
set -euo pipefail

# Usage: ./scripts/worktree-new.sh <branch-name> [base-branch]
# Example: ./scripts/worktree-new.sh feat/user-auth main

BRANCH="${1:?Usage: worktree-new.sh <branch-name> [base-branch]}"
BASE="${2:-main}"
SLUG=$(echo "$BRANCH" | sed 's|/|-|g')
WT_DIR="../$(basename "$PWD")-wt/$SLUG"

# Create branch from base
git fetch origin "$BASE" 2>/dev/null || true
git branch "$BRANCH" "origin/$BASE" 2>/dev/null || git branch "$BRANCH" "$BASE" 2>/dev/null || true

# Create worktree
git worktree add "$WT_DIR" "$BRANCH"

# Install dependencies if needed
if [ -f "$WT_DIR/package.json" ]; then
  echo "Installing dependencies in worktree..."
  (cd "$WT_DIR" && npm install --silent)
fi

if [ -f "$WT_DIR/requirements.txt" ]; then
  echo "Installing Python dependencies in worktree..."
  (cd "$WT_DIR" && python3 -m pip install -r requirements.txt -q)
fi

echo ""
echo "Worktree ready at: $WT_DIR"
echo "Branch: $BRANCH"
echo ""
echo "To start Claude Code in this worktree:"
echo "  cd $WT_DIR && claude"
echo ""
echo "To remove when done:"
echo "  git worktree remove $WT_DIR"
```

```bash
chmod +x scripts/worktree-new.sh
```

### 3. Create a worktree cleanup script

Save as `scripts/worktree-cleanup.sh`:

```bash
#!/usr/bin/env bash
set -euo pipefail

# Remove all worktrees and prune stale entries
WT_DIR="../$(basename "$PWD")-wt"

if [ -d "$WT_DIR" ]; then
  echo "Removing worktrees in $WT_DIR..."
  for dir in "$WT_DIR"/*/; do
    [ -d "$dir" ] || continue
    echo "  Removing: $dir"
    git worktree remove "$dir" --force 2>/dev/null || rm -rf "$dir"
  done
  rmdir "$WT_DIR" 2>/dev/null || true
fi

git worktree prune
echo "Cleanup complete."
```

---

## Step-by-Step: Running 2+ Features in Parallel

### Step 1: Plan all features first (sequential)

Before parallelizing, create plans for all features in the main worktree. This is the **Architect step** — it must happen sequentially to identify dependencies.

```bash
# In your main project directory
claude

# Plan all features
> /feature-workflow user-authentication
> # Review plan, approve it, then STOP before implementation

> /feature-workflow dashboard-analytics
> # Review plan, approve it, then STOP before implementation
```

Or, in a single conversation:

```
I need to build 3 features:
1. User authentication (Google/GitHub OAuth)
2. Dashboard with analytics charts
3. Email notification system

Please create plans for all 3 features. For each, identify which files
will be touched. I need to know which can be built in parallel.
```

### Step 2: Analyze file overlap (critical)

After planning, check for file overlaps between features:

```
Based on the 3 plans, show me a file overlap matrix:
- Which files does each feature touch?
- Which features can be built in parallel (no shared files)?
- Which must be sequential?
```

The Planner/Impact Analyst will produce something like:

```
File Impact Matrix:

                          Auth    Dashboard   Notifications
src/lib/supabase.ts       ✦
src/app/login/page.tsx     ✦
src/app/dashboard/         	       ✦
src/components/charts/             ✦
src/lib/email.ts                                ✦
src/app/api/notify/                             ✦
src/middleware.ts          ✦                     ✦         ⚠ OVERLAP
src/app/layout.tsx         ✦       ✦                      ⚠ OVERLAP

Parallel-safe pairs:
  ✅ Auth + Dashboard (1 shared file: layout.tsx — low risk)
  ✅ Dashboard + Notifications (zero overlap)
  ⚠️ Auth + Notifications (share middleware.ts — needs coordination)
```

### Step 3: Create worktrees and launch agents

```bash
# From your main project directory:

# Create worktrees
./scripts/worktree-new.sh feat/user-auth main
./scripts/worktree-new.sh feat/dashboard main
./scripts/worktree-new.sh feat/notifications main
```

### Step 4: Launch Claude Code in each worktree

Open 3 terminal windows/tabs:

```bash
# Terminal 1
cd ../my-project-wt/feat-user-auth
claude
> /feature-workflow user-authentication
> # Paste the approved plan, or say: "Use plan at docs/plans/user-auth.md"
> Proceed with implementation.

# Terminal 2
cd ../my-project-wt/feat-dashboard
claude
> /feature-workflow dashboard-analytics
> Proceed with implementation.

# Terminal 3
cd ../my-project-wt/feat-notifications
claude
> /feature-workflow email-notifications
> Proceed with implementation.
```

### Step 5: Monitor progress

Check worktree status from your main project:

```bash
# See all active worktrees
git worktree list

# Check commit progress in each branch
git log --oneline feat/user-auth..origin/main   # How far ahead?
git log --oneline feat/dashboard..origin/main
git log --oneline feat/notifications..origin/main
```

### Step 6: Merge sequentially (one at a time)

When agents finish, merge branches **one at a time** back to main:

```bash
# In main project directory
git checkout main

# Merge the feature with fewest file overlaps first
git merge feat/dashboard
# Run tests
npm run check:all

# Then merge auth (may have layout.tsx conflict — easy to resolve)
git merge feat/user-auth
# Resolve any conflicts, run tests
npm run check:all

# Finally merge notifications (may conflict with middleware.ts)
git merge feat/notifications
# Resolve conflicts, run tests
npm run check:all
```

### Step 7: Cleanup

```bash
./scripts/worktree-cleanup.sh
# Optionally delete merged branches
git branch -d feat/user-auth feat/dashboard feat/notifications
```

---

## Task Decomposition: How to Split Work Safely

### The Parallelism Spectrum

Not all tasks can be parallelized. Here's a framework for classification:

#### Embarrassingly Parallel (safe — zero coordination needed)

| Task Type | Why It's Safe | Example |
|-----------|--------------|---------|
| **New isolated features** | Touch only new files | New API endpoint + tests |
| **Documentation per module** | Read-only on code, write-only on docs | Add JSDoc to `src/utils/` |
| **Test generation** | Read code, write new test files | Generate tests for `src/auth/` |
| **Independent bug fixes** | Each bug in a different file | Fix A in `parser.ts`, fix B in `renderer.ts` |
| **Migration tasks** | File-by-file mechanical changes | Convert CSS to Tailwind per component |
| **Prototype competition** | Same spec, different approaches | "Build auth two ways, we'll pick the better one" |

#### Partially Parallel (safe with contract-first approach)

| Task Type | Risk | Mitigation |
|-----------|------|------------|
| **Frontend + backend** | API shape conflicts | Define interface contract first, then parallelize |
| **Features sharing a config file** | Config merge conflicts | One agent adds entries, merge is mechanical |
| **Features sharing a layout** | Layout slot conflicts | Define layout slots first, each agent fills one |

#### Sequential Only (cannot parallelize)

| Task Type | Why Sequential |
|-----------|---------------|
| **Core utility refactor** | Everything depends on it |
| **Database schema migration** | Subsequent features depend on new schema |
| **Shared state management refactor** | All features read/write the same store |
| **Build system changes** | Affects all worktrees |

### Decomposition Process

```
1. List all planned features/tasks
2. For each, ask the Impact Analyst: "Which files will this touch?"
3. Build a file overlap matrix
4. Group into:
   a. Independent sets (parallelize freely)
   b. Contract-linked sets (define interface first, then parallelize)
   c. Sequential dependencies (do these first or last)
5. Order: sequential foundations → parallel features → sequential integration
```

### Vertical vs Horizontal Slicing

**Prefer vertical slicing** (each worker owns a full feature end-to-end) over horizontal slicing (one worker does all DB, another all API, another all UI):

```
❌ Horizontal (high conflict risk):
  Worker A: All database changes for features 1, 2, 3
  Worker B: All API routes for features 1, 2, 3
  Worker C: All UI components for features 1, 2, 3
  → Workers constantly touch related files, API shape conflicts

✅ Vertical (low conflict risk):
  Worker A: Feature 1 (DB + API + UI)
  Worker B: Feature 2 (DB + API + UI)
  Worker C: Feature 3 (DB + API + UI)
  → Workers stay in their own feature directories
```

---

## Conflict Prevention: Architecture for Parallelism

### 1. Contract-First Development

When two features share a boundary (e.g., frontend/backend), define the contract **before** parallelizing:

```bash
# Step 1 (Sequential): Architect defines the API contract
claude
> Define a TypeScript interface for the user auth API:
> - POST /api/auth/login → { user, session }
> - POST /api/auth/logout → { success }
> - GET /api/auth/me → { user }
> Write it to src/types/auth-api.ts and commit.

# Step 2 (Parallel): Both workers import the contract
# Worker A: Implements the backend matching auth-api.ts
# Worker B: Implements the frontend consuming auth-api.ts
```

### 2. Module Boundaries

Structure your project so parallel workers operate in **disjoint directories**:

```
src/
├── features/              # Each feature is isolated
│   ├── auth/              # Worker A: only touches this
│   │   ├── components/
│   │   ├── hooks/
│   │   ├── api/
│   │   └── auth.test.ts
│   ├── dashboard/         # Worker B: only touches this
│   │   ├── components/
│   │   ├── hooks/
│   │   ├── api/
│   │   └── dashboard.test.ts
│   └── notifications/     # Worker C: only touches this
├── shared/                # ⚠ Danger zone — touched by multiple workers
│   ├── ui/                #   → Assign changes here to ONE worker
│   ├── utils/
│   └── types/
└── app/                   # ⚠ Routing — coordinate carefully
    ├── layout.tsx
    └── middleware.ts
```

### 3. Barrel File Avoidance

Barrel files (`index.ts` re-exports) are a merge conflict magnet. When Worker A adds `export { Login }` and Worker B adds `export { Dashboard }` to the same `index.ts`, you get a conflict.

**Solution:** During parallel work, import directly from the file path:

```typescript
// During parallel development:
import { Login } from '@/features/auth/components/Login';     // Direct
import { Dashboard } from '@/features/dashboard/Dashboard';   // Direct

// After merging, a cleanup agent can add barrel files:
// import { Login } from '@/features/auth';
```

### 4. Feature Flags

Instruct parallel agents to wrap new code in feature flags when touching shared files:

```typescript
// middleware.ts — two workers can add independent flags
if (featureFlags.AUTH_ENABLED) {
  // Worker A's auth middleware logic
}

if (featureFlags.NOTIFICATIONS_ENABLED) {
  // Worker C's notification middleware logic
}
```

### 5. Auto-Formatting Before Commit

Run a formatter (Prettier, Black, rustfmt) before every commit. This eliminates "phantom conflicts" caused by whitespace/formatting differences between workers:

```bash
# In each agent's commit flow:
npx prettier --write . && git add -A && git commit -m "..."
```

Or add a pre-commit hook to enforce this automatically.

---

## Conflict Resolution: When Two Workers Touch the Same File

Despite best planning, conflicts happen. Here's the resolution strategy:

### The Merge Order Protocol

Always merge branches **one at a time**, in a specific order:

```
Priority order:
1. Foundation/infrastructure branches (shared utils, types, config)
2. Largest/most complex feature branch (hardest to rebase)
3. Smaller feature branches (easier to adapt)
```

### Scenario: Simple Addition Conflict

Two workers both added entries to the same file (e.g., route config, exports):

```
<<<<<<< main (after Worker A merged)
  '/auth': AuthPage,
=======
  '/dashboard': DashboardPage,
>>>>>>> feat/dashboard
```

**Resolution:** Keep both additions. This is mechanical — git just can't auto-resolve because they're at the same location:

```typescript
  '/auth': AuthPage,
  '/dashboard': DashboardPage,
```

### Scenario: Logic Conflict in Shared File

Two workers modified the same function differently:

```
<<<<<<< main
export function middleware(req) {
  if (!req.auth) return redirect('/login');  // Worker A added auth check
  return next(req);
}
=======
export function middleware(req) {
  await logRequest(req);                     // Worker C added logging
  return next(req);
}
>>>>>>> feat/notifications
```

**Resolution:** Use a **Merge Agent** — spin up a Claude Code session specifically to resolve:

```
I have a merge conflict in src/middleware.ts.

Branch A (auth): Added authentication check — redirects to /login if unauthenticated.
Branch B (notifications): Added request logging.

Both changes are needed. Please combine them so both features work correctly.
Then run tests to verify.
```

### Scenario: Complex Structural Conflict

Two workers restructured the same file differently (renamed functions, moved code blocks):

**Resolution:** Pick one branch as the base, then manually apply the other's changes:

```bash
# Accept Worker A's version as base
git checkout --ours src/shared/utils.ts

# Then read Worker B's version and apply relevant changes
git show feat/notifications:src/shared/utils.ts > /tmp/worker-b-utils.ts

# Open Claude to merge:
claude
> I need to merge changes from two branches into src/shared/utils.ts.
> The current file has Worker A's changes.
> /tmp/worker-b-utils.ts has Worker B's changes.
> Please integrate Worker B's additions while keeping Worker A's structure.
```

### Conflict Resolution Flowchart

```
Conflict detected during merge
           │
           ▼
   Is it additive-only?  ──YES──→  Keep both additions, done ✅
   (both add new lines)
           │
          NO
           │
           ▼
   Is it in a shared     ──YES──→  Spin up Merge Agent to combine ✅
   function/component?
           │
          NO
           │
           ▼
   Is it a structural    ──YES──→  Pick one as base, cherry-pick
   refactor conflict?               the other's intent ✅
           │
          NO
           │
           ▼
   Accept one, discard   ──→  Log what was lost, create follow-up task ✅
   the other (rare)
```

---

## Orchestration Patterns

### Pattern 1: Manual Multi-Terminal (simplest, 2-4 workers)

Open multiple terminal tabs, one per worktree. Monitor by switching tabs.

```bash
# Tab 1: Main (orchestrator)
git worktree list

# Tab 2: Worker A
cd ../my-project-wt/feat-auth && claude

# Tab 3: Worker B
cd ../my-project-wt/feat-dashboard && claude

# Tab 4: Worker C
cd ../my-project-wt/feat-notifications && claude
```

**Best for:** Solo developers running 2-4 parallel agents.

### Pattern 2: Background Agents with Claude Code's `--print` Mode

Use `claude --print` for non-interactive batch execution:

```bash
# Launch 3 agents in background
cd ../my-project-wt/feat-auth && \
  claude --print "Implement user authentication following the plan at docs/plans/auth.md. Use TDD. Commit when done." \
  > ../logs/auth.log 2>&1 &

cd ../my-project-wt/feat-dashboard && \
  claude --print "Implement dashboard analytics following the plan at docs/plans/dashboard.md. Use TDD. Commit when done." \
  > ../logs/dashboard.log 2>&1 &

cd ../my-project-wt/feat-notifications && \
  claude --print "Implement email notifications following the plan at docs/plans/notify.md. Use TDD. Commit when done." \
  > ../logs/notify.log 2>&1 &

# Monitor
tail -f ../logs/*.log
```

**Best for:** Batch processing with known plans. Less interactive control.

### Pattern 3: Scoped Agents with Explicit File Lists (the VMark pattern)

Give each agent an **explicit list of files** it's allowed to touch. This prevents accidental overlap:

```
You are ONLY allowed to edit files in this list:
- src/features/auth/components/*.tsx
- src/features/auth/hooks/*.ts
- src/features/auth/api/*.ts
- src/features/auth/__tests__/*.test.ts

Do NOT edit any files outside this list.
Do NOT edit shared config, layout, or middleware files.
Commit when all tests pass.
```

**Best for:** Large mechanical tasks (documentation, migration, test generation).

### Pattern 4: External Orchestration Tools

Several open-source tools automate the worktree + Claude Code workflow:

| Tool | What It Does |
|------|-------------|
| **[ccswarm](https://github.com/nwiizo/ccswarm)** | Multi-agent orchestration with specialized agent pools (Frontend, Backend, QA) |
| **[parallel-cc](https://github.com/frankbria/parallel-cc)** | Autonomous parallel Claude Code coordination with cloud sandboxes |
| **[crystal](https://github.com/stravu/crystal)** | Desktop app for running multiple Claude/Codex sessions in parallel worktrees |
| **[ccmanager](https://github.com/kbwo/ccmanager)** | Session manager for Claude Code, Gemini CLI, Codex CLI across worktrees |
| **[worktree-workflow](https://github.com/forrestchang/worktree-workflow)** | Bash toolkit for creating worktrees and launching Claude Code |

### Metadata Tracking

For any orchestration beyond 2 workers, track state:

```json
{
  "run_id": "run_20260217_1430",
  "workers": [
    {
      "id": "worker-1",
      "branch": "feat/user-auth",
      "worktree": "../my-project-wt/feat-user-auth",
      "status": "running",
      "assigned_files": ["src/features/auth/**"],
      "started_at": "2026-02-17T14:30:00Z",
      "commits": 0
    },
    {
      "id": "worker-2",
      "branch": "feat/dashboard",
      "worktree": "../my-project-wt/feat-dashboard",
      "status": "completed",
      "assigned_files": ["src/features/dashboard/**"],
      "started_at": "2026-02-17T14:30:00Z",
      "commits": 3
    }
  ]
}
```

---

## Practical Limits

### How Many Workers Should You Run?

| Factor | Constraint | Recommendation |
|--------|-----------|----------------|
| **API rate limits** | Claude Max subscription: generous but finite | Start with **3-4**, increase if no 429 errors |
| **Review capacity** | You must review before merging | Don't launch more PRs than you can review in a day |
| **Conflict probability** | Rises quadratically with worker count | >5 workers on same codebase = significant merge overhead |
| **Machine resources** | Each worktree needs disk + possibly `node_modules` | ~500MB-2GB per worktree depending on project |
| **Context quality** | Agents with overly broad scope produce worse code | Better to have 3 focused agents than 6 unfocused ones |

### The Sweet Spot

```
Solo developer:     2-3 parallel agents
Small team (2-4):   3-5 parallel agents (coordinate via PR reviews)
Larger team:        1-2 agents per developer, each in their own worktree
```

### When Parallelism Hurts

- **Tightly coupled codebase** — Every feature touches the same 5 files → more merge time than saved
- **Unfamiliar domain** — You need to understand Feature A's approach before designing Feature B
- **No test suite** — Without tests, you can't verify merges are clean → high regression risk
- **Short features** — If a feature takes 10 minutes sequentially, the worktree setup overhead isn't worth it

---

## Real-World Example

### VMark's 6-Agent Documentation Sprint

The VMark project (a Tauri-based markdown editor) needed to add AI-maintenance documentation comments to **~400 source files**. They parallelized this across 6 agents.

#### Setup

```
vmark/            # Main repo (orchestrator)
vmark-p3a/        # Worktree: plugins first half (34 dirs)
vmark-p3b/        # Worktree: plugins second half (34 dirs)
vmark-p4a/        # Worktree: utils first half (~50 files)
vmark-p4b/        # Worktree: utils second half (~65 files) + lib
vmark-p5/         # Worktree: components (~67 files) + contexts
vmark-p678/       # Worktree: Rust + MCP + export (~70 files)
```

#### What Worked

| Practice | Why It Worked |
|----------|--------------|
| **Explicit file lists** | Each agent had a pre-computed list of exactly which files to process |
| **Template inlined in prompt** | Agents didn't waste context reading the plan file |
| **Skip rules** | Files <30 lines, already-documented files, test/CSS files were excluded |
| **`max_turns: 150`** | Prevented runaway agents from consuming infinite context |
| **No builds/tests** | Comment-only changes don't need test verification → faster |
| **Disjoint file sets** | Intersection was intentionally empty |

#### What Failed (and how they fixed it)

- **First attempt (Phase 3 only):** Tried to process all plugins in a single agent. Agent ran out of context window before completing. **Fix:** Split into two halves (p3a, p3b).
- **One conflict:** `WindowContext.tsx` was assigned to both p5 (components) and p678 (Rust/MCP). Both agents documented it. **Fix:** During cherry-pick, they merged both versions manually — took 2 minutes.

#### Results

All 6 agents completed successfully. ~400 files documented. The work that would have taken one agent many hours across multiple sessions was done in a single parallel run.

#### Key Takeaway

> "The worktree approach provides true filesystem isolation without merge conflicts." — VMark dev-docs

---

## Quick Reference

### Commands Cheat Sheet

```bash
# Create a new worktree for a feature
./scripts/worktree-new.sh feat/my-feature main

# List all active worktrees
git worktree list

# Launch Claude Code in a worktree
cd ../my-project-wt/feat-my-feature && claude

# Check what branches exist
git branch -a

# Merge a completed feature back to main
git checkout main
git merge feat/my-feature
npm run check:all    # Run tests after merge

# Remove a worktree after merging
git worktree remove ../my-project-wt/feat-my-feature

# Clean up all worktrees at once
./scripts/worktree-cleanup.sh

# Prune stale worktree references
git worktree prune

# Delete merged branches
git branch -d feat/my-feature
```

### Decision Framework

```
"Can I parallelize these two tasks?"

1. Do they touch the same files?
   └─ NO  → ✅ Safe to parallelize
   └─ YES → Continue...

2. Are the shared files additive-only? (config arrays, route lists, exports)
   └─ YES → ⚠️ Parallelize with merge-order plan
   └─ NO  → Continue...

3. Can you define an interface contract for the shared boundary?
   └─ YES → ⚠️ Define contract first (sequential), then parallelize
   └─ NO  → ❌ Do these sequentially
```

### The Parallel Coding Workflow (Summary)

```
┌─────────────────────────────────────────────────┐
│ 1. PLAN (sequential — in main worktree)         │
│    Create plans for all features                │
│    Analyze file overlap matrix                  │
│    Identify parallel-safe groups                │
└──────────────────────┬──────────────────────────┘
                       │
          ┌────────────┼────────────┐
          ▼            ▼            ▼
┌──────────────┐ ┌──────────────┐ ┌──────────────┐
│ 2. IMPLEMENT │ │ 2. IMPLEMENT │ │ 2. IMPLEMENT │
│  Worktree A  │ │  Worktree B  │ │  Worktree C  │
│  feat/auth   │ │  feat/dash   │ │  feat/notify │
│  (parallel)  │ │  (parallel)  │ │  (parallel)  │
└──────┬───────┘ └──────┬───────┘ └──────┬───────┘
       │                │                │
       └────────────────┼────────────────┘
                        │
┌───────────────────────┴─────────────────────────┐
│ 3. MERGE (sequential — one branch at a time)    │
│    Merge least-overlap first                    │
│    Resolve conflicts as they arise              │
│    Run full test gate after each merge          │
└──────────────────────┬──────────────────────────┘
                       │
┌──────────────────────┴──────────────────────────┐
│ 4. CLEANUP                                      │
│    Remove worktrees                             │
│    Delete merged branches                       │
│    Run final full test gate                     │
└─────────────────────────────────────────────────┘
```

---

## Sources

- [Claude Code Official Docs — Common Workflows](https://code.claude.com/docs/en/common-workflows)
- [Parallel AI Coding with Git Worktrees and Custom Claude Code Commands](https://docs.agentinterviews.com/blog/parallel-ai-coding-with-gitworktrees/)
- [Mastering Git Worktrees with Claude Code](https://medium.com/@dtunai/mastering-git-worktrees-with-claude-code-for-parallel-development-workflow-41dc91e645fe)
- [How we're shipping faster with Claude Code and Git Worktrees — incident.io](https://incident.io/blog/shipping-faster-with-claude-code-and-git-worktrees)
- [How Git Worktrees Changed My AI Agent Workflow — Nx Blog](https://nx.dev/blog/git-worktrees-ai-agents)
- [ccswarm — Multi-agent orchestration](https://github.com/nwiizo/ccswarm)
- [parallel-cc — Parallel Claude Code management](https://github.com/frankbria/parallel-cc)
- [crystal — Multi-session AI development](https://github.com/stravu/crystal)
- [ccmanager — Coding Agent Session Manager](https://github.com/kbwo/ccmanager)
- [worktree-workflow — Toolkit for parallel development](https://github.com/forrestchang/worktree-workflow)
