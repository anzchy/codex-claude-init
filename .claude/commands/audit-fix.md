---
description: Audit code → fix all findings → verify → repeat until clean
argument-hint: "[scope: empty=uncommitted, commit -N, path]"
---

# Audit-Fix Loop

Audit code, fix all findings, verify fixes. Loop until zero findings remain.

## Input

```text
$ARGUMENTS
```

## Phase 1: Scope

Parse `$ARGUMENTS` to determine what to audit:

| Input | Scope |
|-------|-------|
| (empty) | Uncommitted changes (`git diff HEAD --name-only`) |
| `staged` | Staged changes (`git diff --cached --name-only`) |
| `commit -1` | Last commit (`git diff HEAD~1 --name-only`) |
| `commit -N` | Last N commits (`git diff HEAD~N --name-only`) |
| `path/to/file` | Specific file or directory |

If no changed files found: report "Nothing to audit" and STOP.

## Phase 2: Audit

Read each changed file and analyze across these dimensions:

1. **Correctness & logic** — is the code logically sound? No patching around symptoms.
2. **Edge cases** — boundary conditions, null/empty, Unicode/CJK, concurrent access
3. **Security** — no vulnerabilities (injection, XSS, path traversal, SQL injection)
4. **Duplicate code** — copy-paste patterns, repeated logic that should be unified
5. **Dead code** — unused imports, unreachable branches, orphaned functions
6. **Shortcuts & patches** — workarounds, TODO markers, band-aids, bypass flags
7. **Project compliance** — adherence to rules in `.claude/rules/*.md` and `AGENTS.md`

Report EVERY issue as: `file:line | severity (Critical/High/Medium/Low) | issue | fix`

## Phase 3: Fix All

Fix **every** finding — Critical, High, Medium, and Low. No exceptions, no deferrals.

Rules:
- Fix the root cause, not the symptom.
- If fixing introduces new code, apply the same audit dimensions mentally before moving on.
- Keep diffs minimal and focused.
- Follow project conventions (`.claude/rules/*.md`).

## Phase 4: Verify

Re-read each fixed file:
1. Confirm the original issue is resolved
2. Check for regressions or new issues introduced by the fix
3. Report any remaining findings

## Phase 5: Loop or Exit

- **Zero findings** (all severities): audit passes. Report "Clean" to user.
- **Findings remain** and iteration < 3: go back to Phase 3, fix, then Phase 4 verify again.
- **3 iterations exhausted**: STOP. List all remaining findings. Do not declare clean.

## Phase 6: Gate (optional)

If the loop exited clean, offer to run the project's full test/gate command.
Report pass/fail to user.
