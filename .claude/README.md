# .claude/ — AI Development Configuration

This directory contains configuration for AI coding tools — primarily [Claude Code](https://docs.anthropic.com/en/docs/claude-code), with cross-tool support via `AGENTS.md`.

## Directory Structure

```
.claude/
├── README.md              # This file
├── settings.json          # Team-shared settings (checked in)
├── settings.local.json    # Personal settings (gitignored)
├── rules/                 # Auto-loaded project rules
├── commands/              # Slash commands (/fix, /feature-workflow, etc.)
├── skills/                # Extended capabilities (planning, auditing, etc.)
└── agents/                # Subagent definitions for /feature-workflow
```

### Settings Files

| File | Shared? | Purpose |
|------|---------|---------|
| `settings.json` | Yes | Team-wide plugin config |
| `settings.local.json` | **No** (gitignored) | Personal permissions, tool approvals |

### Rules (`rules/`)

Auto-loaded into every Claude Code session. These enforce project conventions:

| File | Scope |
|------|-------|
| `00-engineering-principles.md` | Core working agreement |
| `10-tdd.md` | TDD workflow, test patterns, coverage |

### Slash Commands (`commands/`)

| Command | Purpose |
|---------|---------|
| `/feature-workflow` | Gated agent-driven workflow with specialized subagents |
| `/fix` | Root-cause bug fixing with TDD |
| `/audit-fix` | Audit, fix all findings, verify — repeat until clean |

### Skills (`skills/`)

Extended capabilities that Claude Code loads on demand:

| Skill | When used |
|-------|-----------|
| `planning` | Implementation planning with templates |
| `plan-audit` | Audit work against a plan |
| `plan-verify` | Verify completed work against a plan |
| `release-gate` | Quality gate checks |

### Agents (`agents/`)

Subagent definitions used by `/feature-workflow` for complex tasks:

| Agent | Role |
|-------|------|
| `planner` | Research, edge cases, modular work items |
| `implementer` | TDD-driven code changes |
| `auditor` | Diff review for correctness and rule violations |
| `test-runner` | Test execution coordination |
| `verifier` | Final pre-release checklist |
| `spec-guardian` | Validates work against specifications |
| `impact-analyst` | Finds minimal correct change set |
| `release-steward` | Commit messages and release notes |
| `manual-test-author` | Manual testing guide maintenance |

## Related Files (Project Root)

| File | Purpose |
|------|---------|
| `AGENTS.md` | Single source of truth for all AI tool instructions |
| `CLAUDE.md` | Claude Code entry point — `@AGENTS.md` directive |
