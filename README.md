# a11y-engine

A fast, developer-friendly accessibility analyzer for HTML, built with Rust and WebAssembly.

[![CI](https://github.com/Gabrielbsb21/a11y-engine/actions/workflows/ci.yml/badge.svg)](https://github.com/Gabrielbsb21/a11y-engine/actions/workflows/ci.yml)

## Status

Early development - Roadmap Phase 1 (Core Engine). The Rust workspace skeleton is
in place; rules and the analysis pipeline are being implemented task by task.

## What it is

A11y Engine parses HTML, builds a normalized internal AST, runs isolated
accessibility rules (inspired by WCAG) and produces a structured report. The core
is pure Rust so it can run anywhere: browser (via WebAssembly), Node.js, CLI, and
future editor/browser extensions.

Design goals: deterministic, framework-agnostic, extensible, testable, fast.

## Repository structure

```text
a11y-engine/
  crates/
    a11y_engine/        # pure Rust core (parser, ast, indexer, rules, analyzer, report)
  docs/                 # project context, architecture, rules catalog, roadmap, PRD
  prompts/              # reusable AI agent role prompts (01..05)
  tasks/                # spec-driven task specs (001..)
  .cursor/              # Cursor rules, skills and MCP config
  .compozy/             # Compozy reusable agents + workspace config
```

## Build and test

```bash
cargo build
cargo test
```

## Development workflow

This project is developed with a professional AI-assisted, spec-driven workflow:

```text
PRD  ->  Tech Spec  ->  Tasks  ->  Execution loop  ->  Review
```

See [docs/SPEC_DRIVEN_DEVELOPMENT.md](docs/SPEC_DRIVEN_DEVELOPMENT.md) for how to
run it (manually in Cursor or orchestrated with Compozy) and
[CONTRIBUTING.md](CONTRIBUTING.md) for contribution standards.

## Documentation

- [docs/PROJECT_CONTEXT.md](docs/PROJECT_CONTEXT.md) - overview and domain model
- [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) - boundaries and pipeline
- [docs/RULES.md](docs/RULES.md) - accessibility rules catalog
- [docs/ROADMAP.md](docs/ROADMAP.md) - phased roadmap
- [docs/PRD.md](docs/PRD.md) - MVP product requirements
- [AGENTS.md](AGENTS.md) - rules for AI agents working in this repo

## License

MIT - see [LICENSE](LICENSE).
