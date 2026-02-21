# Plan: How to Use Speckit + Feature Workflow for MiniClaw

## Context

You have `MINICLAW_BLUEPRINT-V2.md` ready and want to know: can you run the speckit pipeline (specify → plan → tasks) to generate `spec.md`, `plan.md`, `tasks.md`, then use `/feature-workflow` to implement one task at a time?

**Short answer: Not directly.** They are two separate systems with different planning formats and no built-in handoff. But there IS a clear path to get the best of both worlds.

---

## The Problem: Two Systems, No Bridge

### Speckit pipeline produces:
- `specs/<feature>/spec.md` — user stories (US1, US2, US3...)
- `specs/<feature>/plan.md` — technical plan (data model, contracts, research)
- `specs/<feature>/tasks.md` — ordered tasks (T001, T002... organized by user story)
- `speckit.implement` — executes tasks.md phase by phase (simple executor, no quality agents)

### Feature-workflow produces:
- `docs/plans/YYYYMMDD-HHMM-<name>.md` — its OWN plan with Work Items (WI-001, WI-002...)
- 9-agent gated pipeline: Planner → Spec Guardian → Impact Analyst → Implementer (TDD) → Test Runner → Auditor → Manual Test Author → Verifier → Release Steward

### The mismatch:
- Feature-workflow's **Planner agent creates its own plan** — it doesn't consume speckit's `tasks.md`
- Speckit's `tasks.md` uses `T001 [US1]` format; feature-workflow uses `WI-001` format
- `speckit.implement` is a flat executor; feature-workflow has quality gates (audit, spec-check, impact analysis)
- There is no "feed speckit tasks into feature-workflow" mechanism

---

## The Three Options

### Option A: Feature Workflow Only (Recommended for MiniClaw)

**The Blueprint IS your spec.** MINICLAW_BLUEPRINT-V2.md already contains:
- Protocol contracts (the "what")
- Implementation order (the "when")
- Module breakdown with line budgets (the "how much")
- Dependency chain (the "sequence")

This is exactly what the dev-workflow doc says (Section 5):
> **Implementing a Protocol from the blueprint → `/feature-workflow` directly — the blueprint IS your spec**

---

### Option A: Practical Execution Guide

#### Prerequisite: Update Blueprint Reference

Before running any feature-workflow, update `CLAUDE.md` and `AGENTS.md` to point to V2. The Planner agent reads these files to find architecture context.

**Files to edit:**
- `CLAUDE.md` line 15: change `MINICLAW_BLUEPRINT.md` → `references/MINICLAW_BLUEPRINT-V2.md`
- `AGENTS.md` line 19: change `MINICLAW_BLUEPRINT.md` → `references/MINICLAW_BLUEPRINT-V2.md`
- `CLAUDE.md` line 16: update "Implementation Order" to match V2's 5-phase order
- `.specify/memory/constitution.md` line 51: update "8 Protocol definitions" → "6 core abstractions" (or however many V2 defines)

#### How Feature Workflow Works

When you type `/feature-workflow event-bus`, here's what happens:

1. The **Planner agent** receives `event-bus` as the work name
2. It searches the codebase — reads `AGENTS.md`, `CLAUDE.md`, `MINICLAW_BLUEPRINT-V2.md`, existing `src/` code
3. It creates `docs/plans/YYYYMMDD-HHMM-event-bus.md` with Work Items (WI-001, WI-002, ...)
4. **You review and approve** the plan before implementation starts
5. The remaining 8 agents execute (implement, test, audit, verify, commit)

**Is just the slug enough?** Usually yes — if the Blueprint clearly describes the module. The Planner is smart and will search for context. But for complex modules, adding a one-line hint after the slug helps it focus.

#### The Exact Commands

##### Phase 1: Foundation

```
/feature-workflow types-and-config
```
> Planner context: Blueprint V2 §4.1 (Event/TriggerType/SessionKey) + §9 (config.yaml). This is the smallest module — types.py already has stubs, config.py loads YAML.

```
/feature-workflow memory-store
```
> Planner context: Blueprint V2 §4.5 + V1 §4.1 (full Memory Protocol). Largest Phase 1 module — FTS5, file I/O, search, context loading, pre-compaction flush.

```
/feature-workflow skill-registry
```
> Planner context: Blueprint V2 §4.8 + V1 §4.4 (Skill Protocol). SKILL.md parsing, directory discovery, gate checks, token budget.

```
/feature-workflow conversation-history
```
> Planner context: Blueprint V2 §4.4. NEW module — bounded deque per session, optional SQLite persistence, snapshot save.

##### Phase 2: Runtime (the new layer from V2)

```
/feature-workflow event-bus
```
> Planner context: Blueprint V2 §4.1 + §4.2. NEW module — SimpleEventBus with per-session queues, semaphore-bounded concurrency, backpressure.

```
/feature-workflow agent-runner
```
> Planner context: Blueprint V2 §4.3. REVISED — the agentic loop (prompt builder → LLM → tool_use → execute → loop), ToolCall/ToolResult, max_tool_rounds, HEARTBEAT_OK suppression.

```
/feature-workflow lifecycle-hooks
```
> Planner context: Blueprint V2 §4.9. NEW module — HookRegistry with ON_STARTUP, ON_SHUTDOWN, ON_COMPACTION, ON_CHANNEL_DISCONNECT callbacks.

##### Phase 3: Triggers

```
/feature-workflow telegram-channel
```
> Planner context: Blueprint V2 §4.6 + V1 §4.3. Channel protocol + Telegram adapter. Key change from V1: channels emit Events to the bus instead of calling agent directly.

```
/feature-workflow heartbeat-cron-scheduler
```
> Planner context: Blueprint V2 §4.7 + §7.6. REVISED — heartbeat and cron are separate event emitters. Heartbeat reads HEARTBEAT.md. Cron uses croniter. Webhook HTTP server (aiohttp, HMAC validation).

##### Phase 4: Integration

```
/feature-workflow cli-entrypoint
```
> Planner context: Blueprint V2 §8 Phase 4. Wire event bus, start all sources, click CLI with `miniclaw run` and `miniclaw memory search`.

##### Phase 5: Extension

```
/feature-workflow whatsapp-channel
```
> Planner context: V1 §4.3. WhatsApp adapter via Baileys HTTP sidecar + aiohttp bridge.

#### Tips for Each Run

1. **After typing the command**, the Planner shows you its plan — **read it carefully** before approving. This is your chance to steer.

2. **If the Planner misses V2 changes**, tell it: "Read `references/MINICLAW_BLUEPRINT-V2.md` section X for the protocol definition."

3. **After each feature-workflow completes**, run:
   ```
   /audit-fix commit -1
   ```

4. **Between modules**, update `PROGRESS.md` with lessons learned.

5. **Each feature-workflow creates one branch per module.** The Release Steward proposes commits — you approve each one explicitly.

#### Concrete Sequence (copy-paste ready)

```
Phase 1: /feature-workflow types-and-config
Phase 1: /feature-workflow memory-store
Phase 1: /feature-workflow skill-registry
Phase 1: /feature-workflow conversation-history     [NEW in V2]
Phase 2: /feature-workflow event-bus                [NEW in V2]
Phase 2: /feature-workflow agent-runner             [REVISED in V2]
Phase 2: /feature-workflow lifecycle-hooks          [NEW in V2]
Phase 3: /feature-workflow telegram-channel
Phase 3: /feature-workflow heartbeat-cron-scheduler [REVISED in V2]
Phase 4: /feature-workflow cli-entrypoint
Phase 5: /feature-workflow whatsapp-channel
```

**Pros:** Simplest, uses the most powerful pipeline (9 agents), Blueprint already provides everything speckit.specify would generate.
**Cons:** No formal user-story tracking, no cross-module dependency graph in one file.

---

### Option B: Speckit as Roadmap + Feature Workflow for Execution (Hybrid)

Use speckit ONCE to create a project-level spec/plan/tasks as your **master roadmap**, then use feature-workflow for each module's actual implementation.

**Workflow:**
```
Step 1: /speckit.specify "MiniClaw V2: event-driven personal AI assistant
        with unified event bus, agentic loop, 5 trigger types, conversation
        history, and lifecycle hooks — per MINICLAW_BLUEPRINT-V2.md"

Step 2: /speckit.clarify   (resolve ambiguities)
Step 3: /speckit.plan      (technical plan with data model, contracts)
Step 4: /speckit.tasks     (ordered task list across ALL modules)
Step 5: /speckit.analyze   (consistency check)

  → Now you have specs/<feature>/tasks.md as your MASTER CHECKLIST

Step 6: For each phase/group in tasks.md:
        /feature-workflow <module-name>
        (feature-workflow creates its own Work Items from the Blueprint)

Step 7: After each feature-workflow completes, manually mark tasks
        as [X] in speckit's tasks.md to track overall progress
```

**Pros:** Formal spec with user stories, big-picture task ordering, progress tracking across modules.
**Cons:** Speckit artifacts become a tracking document only — feature-workflow doesn't read them. Manual sync needed. Speckit was designed for user-facing features, not internal module architecture.

---

### Option C: Speckit Full Pipeline (No Feature Workflow)

Use the speckit pipeline for everything, including implementation via `speckit.implement`.

**Workflow:**
```
/speckit.specify → /speckit.clarify → /speckit.plan → /speckit.tasks → /speckit.analyze → /speckit.implement
```

**Pros:** Single unified pipeline, tasks.md is both plan and execution tracker.
**Cons:** `speckit.implement` is a simple phase-by-phase executor — it does NOT have the 9-agent quality pipeline (no Spec Guardian, no Impact Analyst, no Auditor, no Verifier). You lose TDD gating, audit loops, and the quality enforcement that feature-workflow provides.

---

## Recommendation

For MiniClaw core modules: **Option A** — the Blueprint V2 is already more precise than what speckit.specify would generate (Protocol contracts > user stories for internal architecture).

For future user-facing features (web dashboard, voice channel, multi-user support): **Option B or C** — speckit shines when requirements are ambiguous and user-facing.

---

## When WOULD You Use Speckit on This Project?

Speckit makes sense for **future user-facing features** beyond core architecture:

- `/speckit.specify "Add web dashboard for memory browsing and skill management"`
- `/speckit.specify "Add voice channel via Whisper API integration"`
- `/speckit.specify "Add multi-user support with per-user workspaces"`

---

## Verification

After all 11 modules are implemented, run the full gate:
```bash
python3 -m pytest tests/ && python3 -m mypy src/ && python3 -m black --check .
```

Then optionally: `/codex-audit --full` for an independent second-opinion review of the entire codebase.
