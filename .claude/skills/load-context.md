# Load Context Skill

## Description
Load the most recent session context from a previously saved markdown file. Use this at the start of a new session after context compaction.

## Trigger
Invoke with `/load-context` at the beginning of a new session, or when resuming work.

## Instructions

When this skill is invoked, you MUST:

1. **Find the latest context file**:
   - Look in `.claude/contexts/` directory
   - Find the most recent file by timestamp in filename
   - If no files exist, inform the user

2. **Read and parse the context file**:
   - Extract all sections from the saved context
   - Understand the state of work

3. **Present a summary to the user**:
   - Show when the context was saved
   - List completed tasks (for reference)
   - Highlight in-progress tasks and their status
   - Show pending tasks
   - Display resume instructions

4. **Offer to continue**:
   - Ask if the user wants to continue with the in-progress task
   - Or pick from pending tasks

## Output Format

```
📂 Loaded context from: .claude/contexts/[filename]
📅 Saved: [timestamp]
🌿 Branch: [branch name]

━━━ SESSION RESUME ━━━

✅ Previously Completed:
   • Task 1
   • Task 2

🔄 In Progress:
   • [Task name] - [progress/status]
     Next steps:
     1. [step 1]
     2. [step 2]

📋 Pending:
   • Task 1
   • Task 2

📌 Key Context:
   • [Important decision 1]
   • [Important decision 2]

━━━━━━━━━━━━━━━━━━━━━━

🚀 Resume Instructions:
1. [First step]
2. [Second step]

Continue with the in-progress task, or specify what you'd like to work on.
```

## Notes
- If multiple context files exist, always load the most recent one
- Optionally allow user to specify a specific context file to load
- After loading, the session should have full context to continue work seamlessly
