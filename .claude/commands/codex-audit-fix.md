---
description: Audit->fix->verify loop via Codex — finds issues, fixes them, verifies fixes, repeats until clean or you stop
argument-hint: "[scope] [--full | --mini]"
---

## User Input

```text
$ARGUMENTS
```

## What This Does

Runs a complete audit->fix->verify cycle using Codex as an independent reviewer:

1. **Audit** — find issues (full 10-dimension or mini 6-dimension)
2. **Fix** — Claude or Codex fixes the issues (your choice)
3. **Verify** — check that each fix actually resolved the issue
4. **Repeat** — if issues remain, loop back to fix

Continues until all issues are resolved or the user decides to stop.

## Model & Settings Selection

Follow the instructions in `commands/shared/model-selection.md` to discover available models and present choices.

- **Recommended model**: `gpt-5.3-codex`
- **Recommended reasoning effort**: `high`
- **Recommended sandbox level**: `workspace-write`
- **Include sandbox question**: Yes (fixes require write access)

## Workflow

### Step 1: Determine audit type and scope

Parse `$ARGUMENTS`:

| Input | Interpretation |
|-------|----------------|
| (empty) | Mini audit on uncommitted changes |
| `--full` | Full 10-dimension audit on uncommitted changes |
| `--mini` | Mini 6-dimension audit (explicit, same as default) |
| `--full path/to/dir` | Full audit on specific path |
| `path/to/file` | Mini audit on specific file/directory |
| `commit -N` | Mini audit on last N commits |
| `--full commit -N` | Full audit on last N commits |

If scope is empty (no changes), tell the user and STOP.

### Step 1b: Trivial Scope Check

Before proceeding, analyze the diff to determine if the changes warrant an audit.

**Get the diff**:
- For uncommitted changes: `git diff HEAD`
- For commit ranges: `git diff HEAD~N`
- For specific paths: read the files directly

**Classify as trivial if ALL of the following are true**:
- Total code changes <= 5 lines (excluding blank lines and comments)
- Changes are purely mechanical: typo fixes, formatting, whitespace, import reordering, comment edits, version bumps in config files
- No logic, control flow, or data handling changes whatsoever

**NEVER classify as trivial if ANY of these apply**:
- Any change to logic, conditionals, loops, or data flow — even a single character (`>` vs `>=`)
- Files in security-sensitive paths (auth, crypto, permissions, payments, sessions)
- New dependencies added or removed
- Config changes that affect runtime behavior (env vars, feature flags, API endpoints)
- Changes to error handling or validation

**If trivial**: Ask user whether to skip or audit anyway.
If "Skip audit" → respond with "Scope too trivial for audit — no issues expected." and STOP.

Ask the user to confirm audit depth (Mini 6-dim or Full 10-dim).

### Step 2: Run initial audit

**Availability test** — ping Codex first:
```
mcp__codex__codex with:
  prompt: "Respond with 'ok' if you can read this."
  model: {chosen_model}
  config: {"model_reasoning_effort": "{chosen_effort}"}
```
If Codex does not respond, fall back to manual audit and STOP (no fix loop without Codex).

Run the audit using the appropriate prompt from `codex-audit.md` (full) or `codex-audit-mini.md` (mini). For each file in scope:

```
mcp__codex__codex with:
  model: {chosen_model}
  config: {"model_reasoning_effort": "{chosen_effort}"}
  sandbox: read-only
  approval-policy: never
  developer-instructions: "You are a thorough code auditor. Report every issue with exact file:line locations."
  prompt: "{audit prompt for the chosen audit type, per file}"
```

**Wait for each call to complete before the next.**

Collect all findings into a structured audit report.

**Save the `threadId`** from the audit Codex call as `{audit_threadId}`. This will be reused for fix and verify steps when Codex is the fixer.

Display the report to the user.

If **no issues found** → report CLEAN and STOP.

### Step 3: Fix loop

**IMPORTANT**: Maximum **3 iterations** of the fix->verify cycle. After 3 rounds, stop and report remaining issues regardless.

Set `iteration = 1`.

#### 3a: Ask before fixing

Show the findings summary and ask:

**Question 1 — Scope** (severity filter): Fix all / Fix Critical+High only / Stop here

**Question 2 — Who fixes**:

| Fixer | Description |
|-------|-------------|
| Claude (Recommended) | Fix directly using Read/Edit — has full project context, precise edits |
| Codex | Send to Codex for autonomous fixing — sandboxed, isolated |

Store the choice as `{chosen_fixer}` for use in 3b.

#### 3b: Fix issues

##### If `{chosen_fixer}` is **Claude**:

Fix each issue directly using Claude's tools:

1. For each issue in the filtered findings list:
   - Read the file at the reported location
   - Understand the surrounding context
   - Apply the minimal correct fix using the Edit tool
   - If a fix requires changing multiple related locations, fix all of them
2. Do NOT refactor surrounding code — only fix what was reported
3. Do NOT delete code unless the issue specifically calls for removal (dead code, unused imports)
4. After fixing all issues, run available tests if the project has them
5. Show a summary of what was fixed

##### If `{chosen_fixer}` is **Codex**:

**Reuse the audit thread** via `codex-reply` so Codex has full context:

```
mcp__codex__codex-reply with:
  threadId: {audit_threadId}
  prompt: "Fix the following issues from your audit. For each issue, make the minimal correct fix at the exact file:line location.

ISSUES TO FIX:
{filtered findings in file:line | severity | issue | fix format}

RULES:
- Fix each issue at the exact location reported
- Make minimal, targeted changes — do not refactor surrounding code
- If a fix requires changing multiple related locations, fix all of them
- Do not delete code unless the issue specifically calls for removal (dead code, unused imports)
- After fixing all issues, run any available tests to check for regressions
- Report: what you fixed, what you couldn't fix, and any test results"
```

**Fallback**: If `codex-reply` fails (e.g. thread expired), fall back to a fresh `codex` call.

#### 3c: Verify fixes

**If `{chosen_fixer}` was Codex** — continue the same thread for verification.
**If `{chosen_fixer}` was Claude** — use a fresh Codex call for independent verification.

Ask Codex to verify each issue: FIXED / NOT FIXED / PARTIAL / REGRESSED.

#### 3d: Evaluate results

- **All FIXED** → proceed to Step 4 (success)
- **Some NOT FIXED** and `iteration < 3`: ask user to continue, switch fixer, or stop
- **iteration = 3** → proceed to Step 4 with whatever remains

### Step 4: Final report

```markdown
# Audit Fix Report

**Date**: {today}
**Scope**: {what was audited}
**Audit type**: Full (10-dim) / Mini (6-dim)
**Fixer**: {Claude / Codex}
**Model**: {chosen_model} | **Effort**: {chosen_effort} | **Sandbox**: {chosen_sandbox}
**Thread ID**: `{audit_threadId}` _(use `/codex-continue {audit_threadId}` to iterate further — Codex only)_
**Rounds**: {iteration count}

## Result: {ACCEPTED / PARTIAL / UNCHANGED}

## Summary

| Status | Count |
|--------|-------|
| Fixed | {n} |
| Not Fixed | {n} |
| Partial | {n} |
| Regressed | {n} |
| Total | {n} |

## Fixed Issues

| File:Line | Severity | Issue | Status |
|-----------|----------|-------|--------|
| {file:line} | {sev} | {issue} | FIXED |

## Remaining Issues (if any)

| File:Line | Severity | Issue | Status | Notes |
|-----------|----------|-------|--------|-------|
| {file:line} | {sev} | {issue} | NOT FIXED | {why} |

## Changes Made

{git diff --stat output}

## Next Steps

- Review changes: `git diff`
- Run tests: {project-appropriate test command}
- Commit: if satisfied with the fixes
- Revert: `git checkout .` to undo all changes
- Continue: `/codex-continue {audit_threadId}` to address remaining issues
```

### Verdicts

- **ACCEPTED** — all issues fixed, verification passed
- **PARTIAL** — some issues fixed, some remain
- **UNCHANGED** — user chose to stop before fixing, or Codex couldn't fix anything
