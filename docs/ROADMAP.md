# A11y Engine - Roadmap

## Vision

Build a complete accessibility platform powered by Rust and WebAssembly.

---

# Phase 1 - Core Engine

Goal:

Create the first working lint engine.

Deliverables:

* HTML parser
* AST
* Tree traversal
* Rule engine
* Issue reporting

Rules:

* img alt
* input label
* button name
* heading order

Status:

MVP

---

# Phase 2 - Distribution

Goal:

Allow developers to use the engine.

Deliverables:

* npm package
* wasm package
* TypeScript wrapper
* documentation

Example:

```ts
analyze(html)
```

---

# Phase 3 - CLI

Goal:

Accessibility analysis from terminal.

Deliverables:

```bash
npx a11y-engine page.html
```

Features:

* summary
* exit codes
* json output

---

# Phase 4 - Playground

Goal:

Public demonstration platform.

Features:

* paste HTML
* visualize issues
* highlight elements

Purpose:

* documentation
* marketing
* adoption

---

# Phase 5 - Advanced Rules

New categories:

* landmarks
* contrast
* focus management
* tables
* media

Goal:

Increase WCAG coverage.

---

# Phase 6 - VS Code Extension

Goal:

Developer feedback during coding.

Features:

* diagnostics
* quick fixes
* rule explanations

---

# Phase 7 - Browser Extension

Goal:

Analyze any website.

Features:

* DOM analysis
* issue overlay
* accessibility summary

---

# Phase 8 - Accessibility Tree

Goal:

Build an internal accessibility tree.

Features:

* semantic nodes
* accessible names
* landmarks
* roles

This phase becomes the foundation for simulation.

---

# Phase 9 - Screen Reader Simulator

Goal:

Simulate screen reader behavior.

Features:

* reading order
* focus order
* landmark navigation
* heading navigation

Supported simulations:

* NVDA-inspired
* VoiceOver-inspired

---

# Phase 10 - Accessibility Platform

Goal:

Transform the project into a complete ecosystem.

Products:

* Engine
* Playground
* CLI
* VS Code Extension
* Browser Extension
* Simulator

Long-Term Objective:

Become a reference open-source accessibility tooling platform.
