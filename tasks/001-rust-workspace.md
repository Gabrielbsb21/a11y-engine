---
status: done
title: Create Rust workspace and engine crate skeleton
type: infra
complexity: low
dependencies: []
---

# Task: Rust workspace + a11y_engine crate skeleton

## Goal
Create the Cargo workspace and the `crates/a11y_engine` crate skeleton with the
module layout from docs/ARCHITECTURE.md, so later tasks have a place to build.

## Scope
- Workspace `Cargo.toml` with `crates/a11y_engine` as a member.
- `crates/a11y_engine` library crate.
- Empty module files: parser, ast, indexer, rules, analyzer, report, utils.

## Non-goals
- Any rule logic, parsing logic or types beyond empty module stubs.
- Wasm crate, JS package, CLI, playground.

## Files to create
- Cargo.toml (workspace)
- crates/a11y_engine/Cargo.toml
- crates/a11y_engine/src/lib.rs
- crates/a11y_engine/src/{parser,ast,indexer,rules,analyzer,report,utils}/mod.rs

## Files to modify
- (none)

## Implementation details
- `lib.rs` declares the modules; modules can be empty for now.
- No external crates yet beyond what's needed to compile an empty lib.

## Acceptance criteria
- `cargo build` succeeds at the workspace root.
- Module layout matches docs/ARCHITECTURE.md.

## Test cases
- N/A (skeleton). `cargo build` and `cargo test -p a11y_engine` succeed (no tests yet).

## Review checklist
- Pure Rust, no browser/Node/framework deps.
- Small modules, explicit layout.
- No future-phase code introduced.
