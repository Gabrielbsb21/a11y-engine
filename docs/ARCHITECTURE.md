# A11y Engine - Architecture

## Purpose

This document describes the architectural decisions, system boundaries, execution flow and module responsibilities of A11y Engine.

---

# Architectural Principles

The engine must be:

* Deterministic
* Framework agnostic
* Extensible
* Testable
* Portable
* Fast

The engine should not depend on:

* Browser APIs
* React
* Vue
* Angular
* Node-specific APIs

The core must remain pure Rust.

---

# High Level Architecture

```text
Consumers
│
├── Playground
├── CLI
├── VS Code Extension
├── Browser Extension
├── React Apps
└── Next.js Apps
│
▼

TypeScript Adapter
│
▼

WebAssembly Boundary
│
▼

Rust Engine
│
├── Parser
├── AST
├── Indexer
├── Rule Engine
├── Reporter
└── Analyzer
```

---

# Execution Pipeline

```text
HTML
 │
 ▼

Parser

 │
 ▼

Normalized AST

 │
 ▼

Indexer

 │
 ▼

Rule Engine

 │
 ▼

Issues

 │
 ▼

Reporter

 │
 ▼

AnalyzeResult
```

---

# Parser Layer

Responsibilities:

* Parse HTML
* Create raw node structures
* Preserve attributes
* Preserve hierarchy

Must not:

* Execute rules
* Generate reports

---

# AST Layer

Purpose:

Provide a stable internal representation independent of parser implementation.

Example:

```rust
pub struct Node {
    tag_name: String,
    attributes: HashMap<String, String>,
    children: Vec<Node>,
    text_content: Option<String>,
    path: String,
}
```

---

# Indexer Layer

Purpose:

Create optimized indexes used by rules.

Examples:

* headings
* labels
* landmarks
* interactive elements
* aria references

Indexes should be generated once.

Rules consume indexes.

---

# Rule Engine

Every rule must be isolated.

Rules cannot depend on other rules.

Interface:

```rust
trait Rule {
    fn code(&self) -> &'static str;

    fn check(
        &self,
        node: &Node,
        ctx: &RuleContext
    ) -> Vec<Issue>;
}
```

---

# Reporter

Transforms collected issues into:

```rust
AnalyzeResult
```

Responsibilities:

* summary generation
* score calculation
* grouping

---

# Wasm Layer

Purpose:

Expose Rust APIs to JavaScript.

Responsibilities:

* serialization
* deserialization
* bridge

Business logic must never exist here.

---

# TypeScript Adapter

Purpose:

Developer experience.

Responsibilities:

* lazy wasm loading
* typings
* API ergonomics

---

# Future Architecture

Future modules:

```text
Accessibility Tree Builder

Screen Reader Simulator

Accessibility Scoring Engine

VS Code Diagnostics

Browser Overlay
```

These modules should consume the existing AST.
