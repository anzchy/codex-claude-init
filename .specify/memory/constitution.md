# AI Code Connect Constitution

## Core Principles

### I. Atomic Commits (MUST)
- Every commit MUST represent exactly one logical change.
- Commit messages MUST follow `type(scope): description`.
- Allowed types: `feat`, `fix`, `refactor`, `test`, `docs`, `chore`, `build`.
- Unrelated changes MUST NOT be bundled in one commit.

### II. Test-Driven Development (MUST)
- Tests MUST be written before implementation for new logic (Red -> Green -> Refactor).
- New behavior MUST have at least one automated test.
- Unit tests use Vitest in `src/**/*.test.ts`.
- Integration tests, when needed, live in `tests/integration/`.
- Relevant tests and `npx tsc --noEmit` MUST pass before commit.

### III. Changelog Tracking (MUST)
- Every feature, fix, and commit MUST be reflected in root `CHANGELOG.md`.
- Entries MUST be added under `## [Unreleased]` during development.
- Keep a Changelog categories MUST be used: `Added`, `Changed`, `Deprecated`, `Removed`, `Fixed`, `Security`.
- Entries SHOULD include the related commit hash after commit creation.

### IV. Privacy & Local-First (MUST)
- AIC² itself MUST remain a local CLI bridge with no telemetry by default.
- Project content MUST NOT be transmitted by AIC² to external services directly.
- Networked inference is delegated to user-installed upstream CLIs (Claude, Gemini, Codex).
- Local configuration MUST remain in user-local paths (for example `~/.aic/`).

### V. Documentation Consistency (MUST)
- Behavior/config/default changes MUST update docs in the same commit.
- Required review targets before commit: `README.md`, `CONTRIBUTING.md`, `CLAUDE.md`, and affected files in `docs/`.
- Setup instructions, command examples, parameter tables, and defaults MUST match implementation.

## Development Workflow

### Code Quality
- TypeScript strict compatibility is required.
- Runtime ESM imports in source use `.js` extensions.
- Keep adapter behavior behind the `ToolAdapter` contract in `src/adapters/base.ts`.

### Testing Strategy
- Unit tests: Vitest (`src/**/*.test.ts`).
- Integration tests: Vitest (`tests/integration/**/*.test.ts`) when cross-component behavior needs validation.
- Build/type gates: `npm run build` and `npx tsc --noEmit`.

## Constraints

- Preserve backward compatibility for default non-settings workflows.
- Fail closed to read-safe behavior on invalid/unsupported permission mappings.
- Avoid introducing new external services or telemetry in core flows.

## Governance

- This constitution supersedes ad-hoc process decisions in planning/tasks artifacts.
- Any exception requires explicit rationale documented in the relevant spec/plan.
- Amendments must be made in this file first, then reflected in downstream artifacts.

**Version**: 1.0.0 | **Ratified**: 2026-02-14 | **Last Amended**: 2026-02-14
