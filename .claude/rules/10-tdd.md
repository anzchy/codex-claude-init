# 10 - TDD Workflow

Test-Driven Development is structurally enforced. Writing code without tests breaks the quality gate.

## Core Discipline: RED → GREEN → REFACTOR

1. **RED** — Write a failing test that describes the expected behavior.
2. **GREEN** — Write the minimum code to make the test pass.
3. **REFACTOR** — Clean up without changing behavior. Tests must still pass.

Never skip RED. If you write code first, you don't know your test actually catches regressions.

## When Tests Are Required

| Category | Required? | Examples |
|----------|-----------|---------|
| Business logic | **ALWAYS** | State transitions, calculations, validation |
| Utils / helpers | **ALWAYS** | Pure functions, parsers, formatters |
| API endpoints | **ALWAYS** | Request/response, auth, error paths |
| Bug fixes | **ALWAYS** | Regression test proving the fix |
| Edge cases | **ALWAYS** | Empty input, null, boundary values |
| Hooks / effects | **ALWAYS** | Side effects, event handling, lifecycle |
| CSS-only changes | No | Visual QA with reference doc instead |
| Docs / config | No | Markdown, JSON, TOML changes |
| Type-only changes | No | Interface/type additions with no runtime effect |
| Components | Case-by-case | Test behavior (clicks, ARIA), not rendering |

## Pattern Catalog

Adapt these templates to your project's stack:

### 1. Unit Tests (Pure Functions)

```
describe("functionName", () => {
  it.each([
    { input: X, expected: Y },
    { input: A, expected: B },
  ])("input=$input → $expected", ({ input, expected }) => {
    expect(functionName(input)).toBe(expected);
  });
});
```

**Key patterns:**
- Table-driven tests with parameterized cases — exhaustive, readable.
- Pure functions = no mocking needed.
- Cover all branches in one `describe` block.

### 2. State / Store Tests

```
beforeEach(() => {
  // Reset state between tests — isolation is critical
  resetStore();
});

it("tracks state transition", () => {
  performAction();
  expect(getState().value).toBe(expected);
});
```

**Key patterns:**
- Reset state in `beforeEach` to isolate tests.
- Test state transitions, not implementation details.

### 3. Integration / API Tests

```
it("returns 401 for unauthorized request", async () => {
  const response = await request(app).get("/api/protected");
  expect(response.status).toBe(401);
});
```

**Key patterns:**
- Test the full request-response cycle.
- Mock external dependencies (DB, APIs), not internal logic.

## Anti-Patterns — What NOT to Do

| Anti-pattern | Why it's wrong | Do this instead |
|-------------|----------------|-----------------|
| Write code first, tests after | You can't verify your test catches regressions | RED first — always |
| `it("renders without crashing")` | Tests nothing meaningful | Test specific behavior or output |
| Testing implementation details | Breaks on refactor | Test observable behavior (state, output, DOM) |
| Mocking everything | Tests prove nothing | Mock boundaries (APIs, filesystem), not logic |
| Skipping edge cases | Bugs live at boundaries | Empty input, null, max values, concurrent access |

## Running Tests

Replace these with your project's actual commands:

```bash
# Run all tests
npm test              # or: pytest, cargo test, etc.

# Watch mode during development
npm test -- --watch

# Run with coverage
npm test -- --coverage

# Full quality gate (lint + test + build)
npm run check:all     # define this in your package.json / Makefile
```

## File Placement

- Tests go next to the source: `foo.test.ts` beside `foo.ts`
- Larger test suites use `__tests__/` subdirectory
- Shared test helpers go in a `test/` or `__helpers__/` directory
