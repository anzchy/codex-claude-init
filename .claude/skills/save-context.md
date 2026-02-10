# Save Context Skill

## Description
Save the current session context to a markdown file before context compaction. This allows continuing work seamlessly after the context is compacted.

## Trigger
Invoke with `/save-context` when you see "context left until auto-compact" approaching low levels (< 10%).

## Instructions

When this skill is invoked, you MUST:

1. **Generate a comprehensive session summary** including:

   ### Session Metadata
   - Current timestamp (ISO 8601 format)
   - Current git branch
   - Main files being worked on

   ### Completed Tasks
   - List all tasks that were completed in this session
   - Include brief description of what was done
   - Note any important decisions made

   ### In-Progress Tasks
   - Current task being worked on
   - Progress percentage or status
   - Next immediate steps needed
   - Any blockers or pending questions

   ### Pending Tasks
   - Tasks identified but not yet started
   - Priority order if known
   - Dependencies between tasks

   ### Key Context
   - Important technical decisions made
   - Architecture patterns being followed
   - Debugging findings or insights
   - Any temporary workarounds in place

   ### Code Changes Summary
   - Files modified in this session
   - Key functions/classes added or changed
   - Any breaking changes

   ### Resume Instructions
   - Clear steps for the next session to continue
   - What to read/check first
   - Any commands to run

2. **Save to file**:
   - Path: `.claude/contexts/YYYY-MM-DD_HH-MM-SS.md`
   - Use current timestamp for filename

3. **Confirm to user**:
   - Show the file path
   - Show a brief summary of what was saved

## Output Format

```markdown
# Session Context: [Brief Description]

**Saved:** YYYY-MM-DD HH:MM:SS
**Branch:** [current branch]
**Context Usage:** [X%] (approaching compact)

---

## Completed Tasks
- [x] Task 1: Brief description
- [x] Task 2: Brief description

## In-Progress Tasks
- [ ] **Current:** Task description
  - Progress: [X%] or [status]
  - Next steps:
    1. Step 1
    2. Step 2
  - Blockers: None / [description]

## Pending Tasks
- [ ] Task 1
- [ ] Task 2

## Key Decisions
1. Decision 1: Rationale
2. Decision 2: Rationale

## Files Modified
- `path/to/file1.swift` - Description of changes
- `path/to/file2.swift` - Description of changes

## Resume Instructions
1. First, [do this]
2. Then, [do that]
3. Continue with [task]

---
*Auto-generated context save for session continuity*
```
