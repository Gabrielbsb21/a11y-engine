---
status: todo
title: HTML parser abstraction to normalized AST
type: backend
complexity: medium
dependencies: [002-internal-ast]
---

# Task: HTML parser abstraction

## Goal
Parse HTML into the internal normalized `Node` AST, keeping the concrete parser
crate isolated behind the parser layer.

## Scope
- A `parse(html: &str) -> Node` (or document root) function.
- Preserve attributes, hierarchy and text content.
- Assign a stable `path` to each node.

## Non-goals
- Running rules or building reports.
- Indexing.

## Files to create
- (none beyond crate)

## Files to modify
- crates/a11y_engine/src/parser/mod.rs
- crates/a11y_engine/Cargo.toml (add the chosen HTML parsing crate)

## Implementation details
- Choose an HTML parsing crate and wrap it; do NOT leak its types past the parser.
- Map parser nodes -> internal `Node`.
- Define `path` format (document order based) and document it.

## Acceptance criteria
- Given sample HTML, returns a `Node` tree with correct tags/attributes/text.
- Concrete parser types do not appear in the public API.

## Test cases
- passing: simple nested HTML parses into the expected tree shape.
- failing/edge: malformed/empty HTML handled gracefully (no panic).
- edge: attributes preserved including boolean attributes.

## Review checklist
- Parser layer does not execute rules or reports.
- No parser type leaks into AST/rules.
