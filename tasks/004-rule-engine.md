---
status: todo
title: Rule engine, Issue model and analyzer pipeline
type: backend
complexity: medium
dependencies: [003-html-parser]
---

# Task: Rule engine + Issue model + analyzer

## Goal
Define the `Rule` trait, the `Issue`/`AnalyzeResult`/`Summary` model, a rule
registry, and the analyzer pipeline that produces a result from HTML.

## Scope
- `Rule` trait (`code()`, `check(node, ctx)`).
- `RuleContext` carrying indexes (can start minimal).
- `Issue`, `AnalyzeResult`, `Summary` types (per docs/PROJECT_CONTEXT.md).
- Registry of enabled rules.
- `analyze(html) -> AnalyzeResult` wiring parse -> normalize -> index -> rules -> report.

## Non-goals
- Implementing concrete rules (next task).
- Wasm/JS exposure.

## Files to create
- (none beyond crate)

## Files to modify
- crates/a11y_engine/src/rules/mod.rs (trait + registry)
- crates/a11y_engine/src/report/mod.rs (Issue / AnalyzeResult / Summary)
- crates/a11y_engine/src/indexer/mod.rs (minimal context)
- crates/a11y_engine/src/analyzer/mod.rs (pipeline)
- crates/a11y_engine/src/lib.rs (public `analyze`)

## Implementation details
```rust
pub trait Rule {
    fn code(&self) -> &'static str;
    fn check(&self, node: &Node, ctx: &RuleContext) -> Vec<Issue>;
}
```
- Reporter computes `Summary` counts; `score` may be `None` in MVP.
- Analysis must be deterministic.

## Acceptance criteria
- `analyze(html)` returns an `AnalyzeResult` with empty issues when no rules match.
- Reporter aggregates issues into a correct `Summary`.

## Test cases
- passing: empty/compliant HTML -> zero issues, correct summary.
- failing: a dummy test rule reports an issue that appears in the result.
- edge: deterministic output for the same input across runs.

## Review checklist
- Rules are isolated and stateless; no rule depends on another.
- No business logic outside the engine boundaries.
