---
name: new-a11y-rule
description: Add a new accessibility lint rule to the Rust engine following the project's architecture, rules catalog and testing standards. Use when the user wants to create, scaffold or implement a new a11y rule (e.g. IMG_MISSING_ALT, INPUT_MISSING_LABEL).
---

# Skill: Create a new accessibility rule

Use this skill to add one accessibility rule to `crates/a11y_engine`, following
`docs/ARCHITECTURE.md`, `docs/RULES.md` and the `.cursor/rules/rust-engine.mdc`
boundaries. Implement only one rule at a time.

## Step 0 - Read context

Before writing code, read:

- `docs/PROJECT_CONTEXT.md`
- `docs/ARCHITECTURE.md`
- `docs/RULES.md`
- `AGENTS.md`

## Step 1 - Define the rule in the catalog

Confirm the rule exists in `docs/RULES.md`. It must declare:

- `Code` (SCREAMING_SNAKE_CASE, e.g. `IMG_MISSING_ALT`)
- `Severity` (Error | Warning | Info)
- `Description`
- `WCAG` reference

If the rule is not in the catalog, add it there first, then continue.

## Step 2 - Scaffold the rule module

Create one file per rule under `crates/a11y_engine/src/rules/`, named after the
rule in `snake_case` (e.g. `img_missing_alt.rs`). Implement the `Rule` trait:

```rust
pub struct ImgMissingAlt;

impl Rule for ImgMissingAlt {
    fn code(&self) -> &'static str {
        "IMG_MISSING_ALT"
    }

    fn check(&self, node: &Node, ctx: &RuleContext) -> Vec<Issue> {
        // Read the normalized AST / indexes only.
        // Return one Issue per violation; empty Vec when compliant.
    }
}
```

Constraints (enforced by `.cursor/rules/rust-engine.mdc`):

- stateless, deterministic, no side effects, no AST mutation.
- depends only on the AST node + `RuleContext` indexes, never on other rules.
- no browser/DOM/Node APIs.

## Step 3 - Register the rule

Wire the rule into the rule engine registry so the analyzer runs it. Keep the
registry the single place that knows about concrete rules.

## Step 4 - Build the Issue

Each violation returns an `Issue` with at least: `code`, `category`,
`severity`, `message`, `node_path`. Prefer also filling `help` and
`suggestion` to give actionable feedback.

## Step 5 - Tests (required)

Add unit tests in the same module (or `tests/`) covering:

- passing case: compliant HTML -> no issue
- failing case: violating HTML -> exactly one issue with the right code/severity
- edge case: ambiguous/boundary input (e.g. empty `alt=""`, `role="presentation"`)

## Step 6 - Validate

- `cargo test -p a11y_engine`
- `cargo clippy -p a11y_engine`

## Step 7 - Report

Return: summary, changed files, tests added, how to validate, known limitations.
