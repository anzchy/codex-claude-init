---
name: release-gate
description: Run project release gates and summarize results. Use when asked to run full quality gates (lint/test/build), verify readiness, or produce a gate report.
---

# Release Gate

## Overview
Run the full project quality gate and summarize outcomes.

## Workflow
1) Confirm current branch and dirty state (`git status -sb`).
2) Run the full gate command (e.g. `npm run check:all`, `make test`, etc.).
3) If failures occur, capture the first error block and stop.
4) Report:
   - Which steps ran (lint/test/build)
   - Pass/fail status
   - Key errors and next actions

## Notes
- Prefer the full gate over partial commands unless asked.
- Do not run interactive dev servers.
