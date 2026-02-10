

## Session Context Management

**When context is running low (< 10% before auto-compact):**

1. Run `/save-context` to save current session state

2. Context saved to `.claude/contexts/YYYY-MM-DD_HH-MM-SS.md`

3. Includes: completed tasks, in-progress work, pending tasks, key decisions

**When starting a new session after compaction:**

1. Run `/load-context` to restore previous session state

2. Review the loaded context summary

3. Continue with in-progress or pending tasks

**Context files location:** `.claude/contexts/`
