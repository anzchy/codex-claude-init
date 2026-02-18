---
description: Continue a previous Codex session — iterate on findings, request fixes, or drill deeper
argument-hint: "<threadId> <follow-up prompt>"
---

## User Input

```text
$ARGUMENTS
```

## What This Does

Uses the `codex-reply` MCP tool to continue a previous Codex session. The thread preserves full context from the original command, so you can:

> **Note**: Codex threads are **in-memory only** — they are lost when the MCP server restarts (e.g. after restarting Claude Code or the Codex MCP process). If a thread is no longer available, start a fresh session with the appropriate `/codex-*` command instead.

- Iterate on audit findings: "Now fix the 3 Critical issues you found"
- Follow up on implementation: "Run the tests and fix any failures"
- Drill into bug analysis: "Show me the exact call stack for issue #2"
- Refine a review: "Explain the race condition you flagged in more detail"

## Workflow

### Step 1: Parse input

Extract the `threadId` and follow-up prompt from `$ARGUMENTS`:

| Input | Interpretation |
|-------|----------------|
| `<threadId> <prompt>` | Thread ID + follow-up message |
| `<threadId>` (no prompt) | Ask the user for the follow-up prompt |
| (empty) | Ask the user for both threadId and prompt |

If `$ARGUMENTS` is empty or missing the threadId, ask the user for the thread ID.

If the user doesn't have a threadId, suggest they run one of the main commands first and STOP.

If the follow-up prompt is missing, offer common options:
- Fix the issues found
- Explain in more detail
- Run tests

### Step 2: Send follow-up to Codex

Use ToolSearch to find and load `mcp__codex__codex-reply`, then call it:

```
mcp__codex__codex-reply with:
  threadId: {threadId}
  prompt: "{follow_up_prompt}"
```

**If `codex-reply` fails** (thread not found / expired / MCP server restarted):

Tell the user:
```
Thread `{threadId}` is no longer available — Codex threads are in-memory only and are lost when the MCP server restarts.

Options:
- Start a fresh session: /codex-audit, /codex-implement, /codex-bug-analyze, etc.
- Re-run the original command to create a new thread
```
And STOP.

### Step 3: Display response

Present the response:

```markdown
## Codex Follow-up

**Thread ID**: `{threadId}`
**Prompt**: {follow_up_prompt}

---

{codex response}

---

_Thread ID: `{threadId}` — run `/codex-continue {threadId}` to continue this conversation._
```

### Step 4: Offer to continue

Ask the user what to do next:
- Continue the conversation (another `/codex-continue`)
- Start a fresh command
- Done
