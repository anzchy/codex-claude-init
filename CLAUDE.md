# CLAUDE.md

@AGENTS.md

## Claude-Specific Notes

- When using Gemini MCP tools, prefer gemini-3-pro-preview unless a different model is specifically needed.
- Use Plan Mode for any non-trivial task before writing code.
- Use `/feature-workflow [name]` for medium-to-large features.
- Use `/fix [description]` for focused bug fixes.
- Use `/audit-fix [scope]` for audit → fix → verify loops (Claude-only).

## Codex Integration (Dual-Model Workflow)

Claude Code is the primary coder; Codex CLI serves as an independent auditor/reviewer
in an isolated sandbox. This dual-model approach catches hallucinations and blind spots
that a single model would miss.

### Prerequisites

- Install Codex CLI: `npm install -g @openai/codex`
- Authenticate: `codex login` (prefer ChatGPT Plus/Pro subscription over API key)
- Run `/codex-preflight` to verify connectivity and discover available models

### Codex Commands

| Command | Purpose |
|---------|---------|
| `/codex-preflight` | Check Codex connectivity and discover available models |
| `/codex-init` | Generate `.codex-toolkit-for-claude.md` project config |
| `/codex-audit-mini [scope]` | Fast 6-dimension audit (logic, duplication, dead code, debt, shortcuts, comments) |
| `/codex-audit [scope]` | Full 10-dimension audit (adds security, performance, compliance, deps, docs) |
| `/codex-audit-fix [scope]` | Audit → fix → verify loop with Codex as independent reviewer |
| `/codex-bug-analyze <desc>` | Root cause analysis using Codex for independent investigation |
| `/codex-review-plan` | Send a plan to Codex for architectural review (5 dimensions) |
| `/codex-implement <plan>` | Delegate implementation plan to Codex for autonomous execution |
| `/codex-verify <report>` | Verify fixes from a previous audit report |
| `/codex-continue <threadId>` | Continue a previous Codex session (iterate on findings) |
| `/fix-issue #N` | End-to-end GitHub issue resolver with Codex audit loop |
| `/merge-prs` | Review and merge open PRs safely with rebase handling |

### When to Use Which

- **Quick check after small changes**: `/codex-audit-mini`
- **Thorough review before release**: `/codex-audit --full`
- **Fix everything automatically**: `/codex-audit-fix`
- **Investigate a bug**: `/codex-bug-analyze "description"`
- **Review a plan before building**: `/codex-review-plan plan.md`
- **Delegate implementation**: `/codex-implement plan.md`
- **Resolve GitHub issues**: `/fix-issue #123`
