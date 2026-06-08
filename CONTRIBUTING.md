# Contributing to A11y Engine

Thanks for contributing. This project follows a spec-driven, AI-assisted workflow.
Please read [AGENTS.md](AGENTS.md) and the docs before making changes.

## Required reading

- [docs/PROJECT_CONTEXT.md](docs/PROJECT_CONTEXT.md)
- [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)
- [docs/RULES.md](docs/RULES.md)
- [docs/ROADMAP.md](docs/ROADMAP.md)
- [docs/AI_DEVELOPMENT_SPEC.md](docs/AI_DEVELOPMENT_SPEC.md)

## Workflow: spec-driven development

No code is written before a spec exists. The pipeline is:

```text
PRD  ->  Tech Spec  ->  Tasks  ->  Execution loop  ->  Review
```

1. A task is specified in `tasks/` (see existing `001..` for the template).
2. Implement only the current task - no future roadmap phases, no unrelated refactors.
3. Run QA and accessibility review (see `prompts/04` and `prompts/05`).
4. Mark the task `status: done`.

Full details: [docs/SPEC_DRIVEN_DEVELOPMENT.md](docs/SPEC_DRIVEN_DEVELOPMENT.md).

## Architecture boundaries

- Keep the Rust core (`crates/a11y_engine`) pure: no browser/DOM/Node/framework APIs.
- Rules must be isolated, stateless, deterministic and side-effect free.
- Do not couple rules to parser internals; rules read the normalized AST and indexes.
- Serialization / JS bridging belongs to the Wasm layer, never the core.

## Testing standard (every rule)

Each new accessibility rule must ship with tests covering:

- a passing case (compliant HTML, no issue),
- a failing case (violation reported with correct code/severity),
- an edge case (boundary/ambiguous input).

## Local checks

Before opening a PR, make sure these pass (CI runs the same):

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo build
cargo test
```

## Pull requests

- Keep PRs small and scoped to a single task when possible.
- Include: summary, changed files, tests, validation steps, known limitations.
- Reference the related task file in `tasks/`.

## New rules

To add an accessibility rule, follow the step-by-step in
`.cursor/skills/new-a11y-rule/SKILL.md` and add the rule to
[docs/RULES.md](docs/RULES.md) with its code, severity and WCAG reference.
