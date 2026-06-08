# A11y Engine — Project Context

## Overview

A11y Engine is an accessibility lint engine built with Rust and WebAssembly.

The project analyzes HTML and reports accessibility issues inspired by WCAG rules and modern accessibility practices.

The engine must be:

* fast
* framework agnostic
* reusable
* extensible
* browser compatible
* usable from JavaScript / TypeScript

Primary execution target:

* WebAssembly (browser)
* Node.js
* CLI environments

Future targets:

* VS Code extension
* Browser extension
* Accessibility playground
* Screen reader simulation

---

# Product Vision

The project is not only a lint tool.

It should become a reusable accessibility platform.

Long term goals:

1. Accessibility lint engine
2. HTML semantic analyzer
3. Accessibility tree analysis
4. Screen reader simulation
5. Developer tooling ecosystem

The project should prioritize:

* accessibility education
* developer experience
* actionable feedback
* extensibility

---

# Architecture Overview

System layers:

Consumer Layer

* Playground application
* CLI
* React apps
* Next.js apps
* VS Code extension
* Browser extension

↓

TypeScript Adapter Layer

Responsibilities:

* load wasm module
* expose public API
* normalize outputs
* provide typings

↓

WebAssembly Boundary

Responsibilities:

* export Rust functions
* serialize data
* bridge Rust ↔ JS

↓

Rust Engine

Responsibilities:

* parse HTML
* build internal AST
* run rules
* generate reports

---

# Monorepo Structure

```text
a11y-engine/

apps/
  playground/

crates/
  a11y_engine/
  a11y_wasm/

packages/
  a11y_js/
  a11y_cli/

docs/
  PROJECT_CONTEXT.md
  SPEC.md
  ARCHITECTURE.md
```

---

# Rust Engine Modules

```text
crates/a11y_engine/src/

parser/
ast/
indexer/
rules/
analyzer/
report/
utils/
```

Responsibilities:

parser/

Convert HTML into internal structures.

ast/

Contains normalized tree representation independent from parser implementation.

indexer/

Precompute:

* headings
* labels
* interactive elements
* aria references

rules/

Accessibility rules.

analyzer/

Coordinates analysis pipeline.

report/

Produces final output.

---

# Internal Pipeline

HTML Input

↓

Parse

↓

Normalize

↓

Index

↓

Analyze

↓

Report

Detailed flow:

1. Parse raw HTML
2. Normalize nodes
3. Build indexes
4. Execute rules
5. Collect issues
6. Generate result

---

# Internal AST

The engine must NOT depend directly on parser structures.

Create an internal AST.

Example:

```rust
pub struct Node {
    pub tag_name: String,
    pub attributes: HashMap<String, String>,
    pub children: Vec<Node>,
    pub text_content: Option<String>,
    pub path: String,
}
```

Rules operate only against this structure.

---

# Rule Engine Design

Rules must be:

* isolated
* deterministic
* stateless
* testable

Interface:

```rust
pub trait Rule {
    fn code(&self) -> &'static str;

    fn check(
        &self,
        node: &Node,
        ctx: &RuleContext
    ) -> Vec<Issue>;
}
```

Rules must NOT:

* mutate nodes
* modify AST
* depend on browser APIs
* perform side effects

---

# Initial Rule Set (MVP)

Images

* img without alt
* suspicious alt

Forms

* input without label
* button without accessible name

Structure

* multiple h1
* heading skipping

Interaction

* clickable div/span
* missing keyboard support

ARIA

* empty aria-label
* invalid aria usage

Future:

* contrast analysis
* landmark analysis
* accessibility tree validation

---

# Issue Model

```rust
pub struct Issue {
    pub code: String,
    pub category: Category,
    pub severity: Severity,

    pub message: String,

    pub description: Option<String>,

    pub node_path: String,

    pub snippet: Option<String>,

    pub help: Option<String>,

    pub suggestion: Option<String>,
}
```

Result:

```rust
pub struct AnalyzeResult {
    pub issues: Vec<Issue>,
    pub summary: Summary,
}
```

Summary:

```rust
pub struct Summary {
    pub errors: usize,
    pub warnings: usize,
    pub infos: usize,
    pub score: Option<u32>,
}
```

---

# Public JavaScript API

Expected API:

```ts
import { analyze } from "@blueprint/a11y-engine"

const result = analyze(html)
```

Configuration:

```ts
analyze(html, {
  enabledRules: [
    "img-alt",
    "input-label"
  ],

  severityOverrides: {
    "heading-order": "warning"
  }
})
```

---

# Distribution Strategy

Primary package:

```text
@blueprint/a11y-engine
```

Future packages:

```text
@blueprint/a11y-cli
@blueprint/a11y-react
@blueprint/a11y-vscode
```

CLI example:

```bash
npx a11y-engine page.html
```

---

# Performance Goals

Small HTML documents:

Target:

under 50ms analysis

Medium pages:

under 150ms

Large documents:

must remain responsive

The engine should avoid unnecessary allocations.

Prefer iterator patterns.

Minimize copies.

Keep parser isolated.

---

# Non Goals (Current Phase)

Do NOT implement:

* screen reader simulation
* DOM rendering
* browser APIs
* CSS rendering engine
* visual snapshots
* AI features

These belong to future versions.

---

# Development Phases

Phase 1

* parser
* AST
* traversal
* img alt rule
* input label rule

Phase 2

* rule engine
* issue model
* reporting

Phase 3

* wasm exports
* JS wrapper
* npm package

Phase 4

* CLI
* playground

Phase 5

* advanced rules

Phase 6

* VS Code extension

Phase 7

* accessibility tree simulation

---

# Engineering Principles

Prefer:

* clean architecture
* separation of concerns
* deterministic behavior
* pure functions
* small modules
* composability

Avoid:

* framework coupling
* browser dependency
* global mutable state
* large monolithic analyzers

---

# Expected Outcome

The AI should generate:

1. Rust workspace
2. Engine crate
3. Wasm crate
4. TypeScript wrapper
5. Initial rules
6. Public API
7. Tests
8. Documentation
9. Monorepo structure

The generated code must prioritize maintainability, accessibility knowledge, extensibility and performance.
