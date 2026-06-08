# AGENTS.md

## Project

A11y Engine is a Rust + WebAssembly accessibility lint engine.

## Required Reading

Before making changes, always read:

* docs/PROJECT_CONTEXT.md
* docs/ARCHITECTURE.md
* docs/RULES.md
* docs/ROADMAP.md
* docs/AI_DEVELOPMENT_SPEC.md

## Core Rules

* Keep the Rust engine pure.
* Do not use browser APIs inside `crates/a11y_engine`.
* Do not couple rules to parser internals.
* Keep rules stateless and testable.
* Prefer small modules.
* Prefer explicit types.
* Write tests for every rule.
* Do not implement future roadmap phases unless explicitly requested.

## Architecture Boundaries

Allowed:

* Rust core handles parsing, AST, indexing, rules and reports.
* Wasm crate exposes Rust functionality.
* TypeScript package handles developer experience.
* CLI handles filesystem and terminal output.
* Playground handles UI only.

Not allowed:

* React inside Rust core.
* DOM APIs inside Rust core.
* Rule logic inside TypeScript adapter.
* Business logic inside Wasm boundary.
* Large unrelated refactors.

## Implementation Style

Every task must include:

* summary
* changed files
* tests
* validation steps
* known limitations

## Testing

For each new rule, include:

* passing case
* failing case
* edge case

## Agent Behavior

When uncertain:

1. Stop.
2. Explain the ambiguity.
3. Suggest the safest option.

Do not invent requirements.

Do not implement everything at once.

Favor incremental delivery.
