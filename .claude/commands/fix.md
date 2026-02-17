---
description: Fix issues properly - no patches, no shortcuts, no regressions
argument-hint: "[issue description or error message]"
---

# Fix

## Context

```text
$ARGUMENTS
```

## Fixing Philosophy

**No half measures.** Every fix must be complete and correct.

### Principles

1. **Understand before fixing** — Read the code, trace the flow, identify root cause
2. **Fix the cause, not the symptom** — No band-aids, no workarounds, no "good enough"
3. **Rewrite if necessary** — Bad code deserves replacement, not patching
4. **Test-first** — Write a failing test that captures the bug, then fix, then verify green (see `.claude/rules/10-tdd.md`)
5. **Zero regressions** — Run all tests before declaring done
6. **Clean as you go** — If you touch it, leave it better than you found it

### Anti-patterns to Avoid

- Adding flags to bypass broken logic
- Wrapping bad code in try-catch to silence errors
- Commenting out problematic code
- Adding TODO for "later"
- Special-casing edge cases without fixing core issue
- Copy-pasting fixes across similar code

## Process

### 1. Reproduce

- Read the relevant source files. Trace the call chain from symptom to root cause.
- If the issue involves UI behavior, ask the user to reproduce it.

### 2. Diagnose

- Find the **root cause**, not just where it crashes.
- Check if similar patterns exist elsewhere — the same bug may lurk in related code.

### 3. Test First (RED)

- Write a failing test that captures the bug.
- Follow the pattern catalog in `.claude/rules/10-tdd.md`.
- Exception: CSS-only or visual bugs don't need unit tests — use visual QA instead.

### 4. Fix Properly (GREEN)

- Address the root cause. Rewrite if the existing code is fundamentally flawed.
- Keep the diff minimal and focused — don't refactor unrelated code.

### 5. Refactor

- Clean up without changing behavior. Tests must still pass.
- Remove dead code. Update comments if they're now stale.

### 6. Verify

- Run the project's full test/gate command — lint, tests, and build must all pass.
- If user-facing behavior changed, update relevant documentation.

### When to Rewrite vs Patch

**Rewrite when:**
- The existing code is fundamentally flawed
- Patching would add complexity
- The fix requires understanding fragile logic
- Similar bugs have occurred in this code before

**Patch only when:**
- The code is sound but has a small oversight
- The fix is isolated and obvious
- Rewriting would introduce unnecessary risk
