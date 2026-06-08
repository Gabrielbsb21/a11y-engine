# AI Development Spec — A11y Engine

## 1. Project Overview

A11y Engine is a Rust + WebAssembly accessibility lint engine for analyzing HTML and reporting accessibility issues.

This project must be developed using an AI-assisted engineering workflow based on:

* LLM fundamentals
* Prompt engineering
* Context injection
* Rules and system prompts
* Tool use
* Spec-driven development
* Agentic planning
* Autonomous execution loops
* QA and review agents

The goal is not only to build the product, but also to create a professional AI-assisted software development process.

---

## 2. Product Goal

Build a high-performance accessibility lint engine that can run in:

* Browser
* Node.js
* CLI
* Playground web app
* Future VS Code extension
* Future browser extension

Initial product description:

> High-performance accessibility lint engine powered by Rust and WebAssembly.

---

## 3. Development Philosophy

The project must follow Spec-Driven Development.

The AI must not generate code before reading and understanding:

* PROJECT_CONTEXT.md
* ARCHITECTURE.md
* RULES.md
* ROADMAP.md
* AI_DEVELOPMENT_SPEC.md

The AI must work incrementally.

Never implement the entire project at once.

Each implementation step must include:

* goal
* scope
* files changed
* acceptance criteria
* tests
* review checklist

---

# Phase 1 — LLM Fundamentals and Prompt Engineering

## Objective

Use LLMs in a controlled and predictable way.

The AI should be treated as an engineering assistant, not as an unrestricted code generator.

## What to Study

* LLM architecture basics
* tokens
* context windows
* model differences
* prompt engineering
* zero-shot prompting
* few-shot prompting
* structured prompting
* evaluation harnesses

## Project Application

Create a `/prompts` folder.

```text
prompts/
  01-architecture-review.md
  02-repository-setup.md
  03-core-engine.md
  04-rule-engine.md
  05-first-rule.md
  06-code-review.md
```

## Required Prompt Pattern

Every prompt should include:

```text
Context:
Goal:
Scope:
Constraints:
Expected output:
Acceptance criteria:
```

## First AI Prompt

```text
Read the following files:

- PROJECT_CONTEXT.md
- ARCHITECTURE.md
- RULES.md
- ROADMAP.md
- AI_DEVELOPMENT_SPEC.md

Do not generate code yet.

Act as a Staff Software Engineer.

Return:

1. Architecture summary
2. Domain model summary
3. Module responsibilities
4. Risks
5. Ambiguities
6. Suggested implementation order
```

---

# Phase 2 — Behavior Control: Context, Rules and Skills

## Objective

Make the AI behave like a specialized engineering agent.

## What to Study

* system prompts
* rules
* tool use
* function calling
* structured outputs
* context injection
* RAG basics

## Project Application

Create an `AGENTS.md` file.

The AI must follow repository rules.

## Agent Rules

The AI must:

* read documentation before coding
* preserve Clean Architecture
* keep Rust core independent from browser APIs
* keep rules stateless
* write tests
* avoid overengineering
* avoid implementing future phases too early
* explain architectural decisions before major changes

The AI must not:

* add React code inside Rust core
* couple rules to parser internals
* generate large unrelated changes
* skip tests
* invent requirements not present in docs

## Context Strategy

The project context must be injected through:

* PROJECT_CONTEXT.md
* ARCHITECTURE.md
* RULES.md
* ROADMAP.md
* AI_DEVELOPMENT_SPEC.md
* AGENTS.md

---

# Phase 3 — Planning, MCP and Spec-Driven Development

## Objective

Use AI primarily to plan, design and break down work before coding.

## What to Study

* Model Context Protocol
* AI coding agents
* planning agents
* Spec-Driven Development
* technical specs
* architecture documents
* task decomposition

## Project Application

Before each implementation phase, the AI must generate a task spec.

Example:

```text
tasks/
  001-rust-workspace.md
  002-internal-ast.md
  003-html-parser.md
  004-rule-engine.md
  005-img-missing-alt-rule.md
```

## Task Spec Template

Each task must include:

```md
# Task: <name>

## Goal

## Scope

## Non-goals

## Files to create

## Files to modify

## Implementation details

## Acceptance criteria

## Test cases

## Review checklist
```

## Planning Agent Prompt

```text
You are the Planning Agent for this repository.

Read all documentation.

Create the next implementation task.

Do not write code.

The task must be small enough to be implemented safely in one iteration.

Return a complete task spec with acceptance criteria and test cases.
```

---

# Phase 4 — Execution Agents and Autonomous Loops

## Objective

Use AI agents to implement, test, review and fix tasks iteratively.

## What to Study

* implementation agents
* QA agents
* review agents
* ReAct pattern
* autonomous loops
* compile-test-fix cycles

## Project Application

Use separate agent roles:

1. Planning Agent
2. Implementation Agent
3. QA Agent
4. Review Agent
5. Refactor Agent

## Implementation Loop

```text
Task Spec
  ↓
Implementation Agent
  ↓
Run tests
  ↓
If failure:
  send error back to agent
  ↓
Fix
  ↓
Run tests again
  ↓
Review Agent
  ↓
Merge when accepted
```

## Implementation Agent Prompt

```text
You are the Implementation Agent.

Read:

- AGENTS.md
- AI_DEVELOPMENT_SPEC.md
- current task file

Implement only the current task.

Do not implement future roadmap items.

After coding, return:

1. Summary of changes
2. Files changed
3. Tests added
4. How to validate
5. Known limitations
```

## QA Agent Prompt

```text
You are the QA Agent.

Review the implementation against the task spec.

Check:

- correctness
- test coverage
- architecture boundaries
- edge cases
- naming
- maintainability

Return:

1. Pass/fail
2. Issues found
3. Required fixes
4. Suggested improvements
```

---

# Professional Repository Structure

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
  ARCHITECTURE.md
  RULES.md
  ROADMAP.md
  AI_DEVELOPMENT_SPEC.md

prompts/
  01-architecture-review.md
  02-repository-setup.md
  03-core-engine.md
  04-rule-engine.md
  05-first-rule.md
  06-code-review.md

tasks/
  001-rust-workspace.md
  002-internal-ast.md
  003-html-parser.md
  004-rule-engine.md
  005-img-missing-alt-rule.md

AGENTS.md
README.md
```

---

# Recommended Implementation Order

## Step 1

Create repository structure and documentation.

## Step 2

Create Rust workspace.

## Step 3

Create `a11y_engine` crate.

## Step 4

Implement internal AST.

## Step 5

Implement HTML parser abstraction.

## Step 6

Implement analyzer pipeline.

## Step 7

Implement rule engine.

## Step 8

Implement first rule:

* IMG_MISSING_ALT

## Step 9

Implement more MVP rules:

* INPUT_MISSING_LABEL
* BUTTON_MISSING_NAME
* HEADING_ORDER_INVALID

## Step 10

Create Wasm package.

## Step 11

Create TypeScript wrapper.

## Step 12

Create CLI.

## Step 13

Create Playground.

---

# Acceptance Criteria for AI Workflow

The project workflow is successful when:

* every task is documented before implementation
* every implementation has tests
* AI does not generate unrelated code
* architecture boundaries remain clear
* Rust core remains independent
* rules are isolated
* output model is stable
* npm package can expose the engine
* CLI can analyze an HTML file
* playground can run analysis in browser

---

# Long-Term Goal

This project should evolve from:

```text
Accessibility Lint Engine
```

to:

```text
Accessibility Developer Platform
```

Future modules:

* Accessibility Tree Builder
* Screen Reader Simulator
* VS Code Extension
* Browser Extension
* CI integration
* AI-assisted accessibility suggestions

```
```
