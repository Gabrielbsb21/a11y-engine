---
status: todo
title: Define internal normalized AST
type: backend
complexity: low
dependencies: [001-rust-workspace]
---

# Task: Internal AST

## Goal
Define the normalized internal `Node` representation that rules operate on,
independent of the parser implementation.

## Scope
- `Node` struct as in docs/PROJECT_CONTEXT.md / docs/ARCHITECTURE.md.
- Basic constructors / helpers (children iteration, attribute lookup).

## Non-goals
- Parsing HTML (separate task).
- Indexing or rules.

## Files to create
- (none beyond crate)

## Files to modify
- crates/a11y_engine/src/ast/mod.rs

## Implementation details
```rust
pub struct Node {
    pub tag_name: String,
    pub attributes: HashMap<String, String>,
    pub children: Vec<Node>,
    pub text_content: Option<String>,
    pub path: String,
}
```
- Provide read-only helpers (e.g. `attr(&self, name) -> Option<&str>`).
- No mutation APIs required by rules.

## Acceptance criteria
- `Node` compiles and is documented.
- Helpers have unit tests.

## Test cases
- passing: attribute lookup returns the value when present.
- failing/edge: lookup returns None when absent; node with no children iterates empty.

## Review checklist
- AST is independent from any parser type.
- No browser/DOM dependency.
