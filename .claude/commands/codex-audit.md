---
description: Autonomous code auditor running in isolated context - verifies implementation against specs, principles, and quality standards
argument-hint: "[file-or-dir] [commit -N]"
---

## User Input

```text
$ARGUMENTS
```

## Audit Checklist

Use TodoWrite to track progress through these phases:

```
* Reconnaissance: identify scope and structure
* Dimension 1: Redundant & Low-Value Code
* Dimension 2: Security & Risk Management
* Dimension 3: Code Correctness & Reliability
* Dimension 4: Compliance & Standards
* Dimension 5: Maintainability & Readability
* Dimension 6: Performance & Efficiency
* Dimension 7: Testing & Validation
* Dimension 8: Dependency & Environment Safety
* Dimension 9: Documentation & Knowledge Transfer
* Dimension 10: Code Comments & Documentation Headers
* Generate comprehensive audit report
```

## Model & Settings Selection

Follow the instructions in `commands/shared/model-selection.md` to discover available models and present choices.

- **Recommended model**: `gpt-5.3-codex`
- **Recommended reasoning effort**: `high`
- **Include sandbox question**: No (audits always use `read-only`)

## Audit Strategy

**IMPORTANT**: Run Codex calls SEQUENTIALLY (one at a time) to avoid timeouts.

### Phase 1: Reconnaissance

Parse `$ARGUMENTS` to determine scope:
| Input | Scope |
|-------|-------|
| (empty) | Uncommitted changes (`git diff HEAD --name-only`) |
| `commit -1` | Last commit (`git diff HEAD~1 --name-only`) |
| `commit -N` | Last N commits (`git diff HEAD~N --name-only`) |
| `--full` | Entire codebase (scan src/, lib/, app/) |
| `path/to/dir` | Specific directory/file |

Identify:
- Technology stack and languages
- Project structure and organization
- Entry points (main, routes, controllers)
- High-risk areas (auth, payments, data processing)

### Phase 2: Audit All 10 Dimensions

**Availability test** — before the real audit, send a short ping to Codex:
```
mcp__codex__codex with:
  prompt: "Respond with 'ok' if you can read this."
  model: {chosen_model}
  config: {"model_reasoning_effort": "{chosen_effort}"}
```
If Codex does not respond or errors out, skip to **Phase 4: Fallback** immediately. Do not retry.

For each code file, run Codex with comprehensive prompt:

```
mcp__codex__codex with:
  model: {chosen_model}
  config: {"model_reasoning_effort": "{chosen_effort}"}
  sandbox: read-only
  approval-policy: never
  developer-instructions: "You are a thorough security and code quality auditor."
  prompt: "Audit {filename} across all 10 dimensions:

    **Dimension 1: Redundant & Low-Value Code**
    - Dead code: unreachable paths, unused functions/imports, commented-out code
    - Duplicate code: copy-paste patterns, repeated logic
    - Useless code: unused variables, no-op operations, empty catch blocks

    **Dimension 2: Security & Risk Management**
    - Input validation: SQL injection, XSS, command injection, path traversal
    - Sensitive data: hard-coded secrets, logged credentials, unencrypted data
    - Auth/authz: weak passwords, broken access control, session issues
    - Cryptography: weak algorithms, improper key management

    **Dimension 3: Code Correctness & Reliability**
    - Logic errors: edge cases, boundary conditions, race conditions
    - Runtime risks: null dereference, array bounds, division by zero
    - Error handling: missing try-catch, swallowed exceptions, silent failures
    - Resource leaks: unclosed files, connections, memory

    **Dimension 4: Compliance & Standards**
    - Coding standards: naming conventions, code structure
    - Framework conventions: proper API usage, deprecated features
    - License compliance: GPL, MIT, Apache compatibility

    **Dimension 5: Maintainability & Readability**
    - Complexity: cyclomatic complexity >15, nested conditionals
    - Size: functions >50 lines, classes >500 lines
    - Magic numbers: hard-coded values not in constants
    - DRY violations: repeated logic that should be extracted

    **Dimension 6: Performance & Efficiency**
    - Algorithm efficiency: O(n^2) that could be O(n log n)
    - Database: N+1 queries, missing indexes, no pagination
    - Memory: excessive allocations, large data not streamed
    - I/O: blocking operations, unbatched requests

    **Dimension 7: Testing & Validation**
    - Coverage gaps: critical paths without tests
    - Test quality: flaky tests, missing edge cases
    - Missing integration tests

    **Dimension 8: Dependency & Environment Safety**
    - Vulnerabilities: known CVEs in dependencies
    - Outdated packages: abandoned or EOL libraries
    - Config security: secrets in configs, missing .gitignore

    **Dimension 9: Documentation & Knowledge Transfer**
    - Missing docs: undocumented public APIs
    - Outdated comments: comments that don't match code
    - Setup instructions: incomplete or missing

    **Dimension 10: Code Comments & Documentation Headers**
    - Stale comments: doc comments that don't match current code behavior
    - Misleading comments: inline or doc comments that describe different behavior than implementation
    - Missing function docs: exported functions/classes without doc comments
    - Comment rot: TODO/FIXME/HACK markers without concrete, actionable descriptions

    Report each issue as: file:line | severity(Critical/High/Medium/Low) | dimension | issue | fix"
```

**Wait for each Codex call to complete before starting the next one.**

Skip non-code files (*.md, *.json, *.css, etc.) unless specifically requested.

### Phase 3: Compile Report

After all audits complete, compile findings into:

```markdown
# Audit Report

**Date**: {today}
**Scope**: {what was audited}
**Files**: {count}
**Model**: {chosen_model} | **Effort**: {chosen_effort}
**Thread ID**: `{threadId}` _(use `/codex-continue {threadId}` to iterate on findings)_

## Executive Summary

**Overall Risk Score**: Critical / High / Medium / Low

| Dimension | Critical | High | Medium | Low |
|-----------|----------|------|--------|-----|
| 1. Redundant Code | X | X | X | X |
| 2. Security | X | X | X | X |
| 3. Correctness | X | X | X | X |
| 4. Compliance | X | X | X | X |
| 5. Maintainability | X | X | X | X |
| 6. Performance | X | X | X | X |
| 7. Testing | X | X | X | X |
| 8. Dependencies | X | X | X | X |
| 9. Documentation | X | X | X | X |
| 10. Code Comments | X | X | X | X |

**Verdict**: PASS / NEEDS WORK / BLOCKED

## Findings by Dimension

### Dimension 1: Redundant & Low-Value Code
| File:Line | Severity | Issue | Fix |
|-----------|----------|-------|-----|
| ... | ... | ... | ... |

[Continue for all 10 dimensions]

## Top Priority Actions

1. **[Critical]** {action} - {file:line}
2. **[Critical]** {action} - {file:line}
3. **[High]** {action} - {file:line}

## Positive Observations
- {good practice found}
- {good practice found}
```

### Phase 4: Fallback - Manual Audit

**CRITICAL**: If Codex returns empty/no findings, you MUST perform the audit manually.

When Codex returns nothing or incomplete results:

1. **Read each file** using the Read tool
2. **Analyze all 10 dimensions** as described above
3. **Use Grep** to search for common issues:
   - Dead code markers: `TODO`, `FIXME`, `HACK`, `XXX`, `DEPRECATED`
   - Security patterns: `password`, `api_key`, `secret`, `token`, `eval`, `exec`, `innerHTML`
   - Error handling: bare `except:`, empty `catch {}`, `.catch()`
   - Dependency check: `npm audit` or `pip-audit` if applicable
4. **Report findings** in the same format as Phase 3

**Do NOT say "Codex didn't return findings" and stop. Always complete the audit manually if Codex fails.**
