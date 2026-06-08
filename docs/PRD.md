# PRD - A11y Engine MVP (Core Lint Engine)

Status: Draft
Phase: Roadmap Phase 1 - Core Engine
Owner: Project lead
Related docs: PROJECT_CONTEXT.md, ARCHITECTURE.md, RULES.md, ROADMAP.md

---

## 1. Problem

Accessibility issues in HTML are common, expensive to find late, and most tooling
is either browser-bound, slow, or hard to embed in other environments (CLI, CI,
editors). Developers need a fast, portable, framework-agnostic engine that
analyzes HTML and reports actionable accessibility issues.

## 2. Goal

Deliver the first working lint engine: a pure Rust core that parses HTML, builds
a normalized AST, runs a small set of isolated rules, and produces a stable
result model. This is the foundation every future product (Wasm package, CLI,
playground, extensions) consumes.

## 3. Target users (MVP)

- Engine integrators (the team building the Wasm/JS/CLI layers on top).
- Indirectly: web developers who will use those layers later.

The MVP is consumed programmatically in Rust; end-user UX (CLI/playground) is out
of scope here.

## 4. Scope (in)

Pure Rust core in `crates/a11y_engine` implementing the pipeline from
`docs/ARCHITECTURE.md`:

- `parser`: HTML -> raw node structures (attributes + hierarchy preserved).
- `ast`: normalized internal `Node` independent from the parser implementation.
- `indexer`: precomputed indexes (headings, labels, interactive elements, aria refs).
- `rule engine`: `Rule` trait + registry; isolated, stateless rules.
- `reporter`: builds `AnalyzeResult` (issues + summary).
- `analyzer`: coordinates parse -> normalize -> index -> run rules -> report.

### MVP rules (4)

| Code | Severity | WCAG |
| --- | --- | --- |
| `IMG_MISSING_ALT` | Error | 1.1.1 |
| `INPUT_MISSING_LABEL` | Error | 1.3.1, 3.3.2 |
| `BUTTON_MISSING_NAME` | Error | 4.1.2 |
| `HEADING_ORDER_INVALID` | Warning | 1.3.1 |

Each rule is defined in `docs/RULES.md` and shipped with tests.

## 5. Out of scope (non-goals)

- Wasm crate, TypeScript wrapper, CLI, playground (later phases).
- Additional rules beyond the 4 above (contrast, landmarks, ARIA, etc.).
- Browser/DOM APIs, CSS rendering, visual snapshots, AI features.
- Screen reader simulation and accessibility tree.

## 6. Functional requirements

- Given HTML input, the analyzer returns an `AnalyzeResult` with an `issues` list
  and a `summary` (errors/warnings/infos counts, optional score).
- Each `Issue` carries `code`, `category`, `severity`, `message`, `node_path`,
  and optionally `snippet`, `help`, `suggestion`.
- Rules run independently; disabling/removing one rule does not affect others.
- Analysis is deterministic: same input -> same output.

## 7. Non-functional requirements

- Performance: small documents analyzed in under 50ms; medium pages under 150ms.
- Portability: no browser/Node/framework dependencies in the core.
- Maintainability: small modules, explicit types, isolated rules.

## 8. Success metrics / acceptance criteria

The MVP is done when:

- `cargo test -p a11y_engine` passes with passing/failing/edge tests for all 4 rules.
- The analyzer produces a stable `AnalyzeResult` for a sample HTML document.
- Architecture boundaries hold: core is pure Rust, rules are isolated and stateless.
- The 4 MVP rules detect their target violations and stay silent on compliant HTML.
- Performance target met for a representative small document.

## 9. Risks / open questions

- Choice of HTML parsing crate (must stay isolated behind the parser layer).
- `node_path` format needs to be stable enough for future consumers (CLI/playground).
- Scoring formula for `summary.score` (can stay `None` in MVP).

## 10. Rollout

This PRD feeds the Tech Spec (architecture decisions) and the task breakdown in
`tasks/`. Implementation proceeds task by task per `docs/SPEC_DRIVEN_DEVELOPMENT.md`.
