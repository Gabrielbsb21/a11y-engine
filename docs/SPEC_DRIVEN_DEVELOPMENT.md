# Spec-Driven Development - Process Guide

This is the short, operational guide for how we run the AI-assisted workflow on
A11y Engine. The "why" and the full theory live in `docs/AI_DEVELOPMENT_SPEC.md`.
This file is the "how to run it".

## The pipeline

```
Idea  ->  PRD  ->  Tech Spec  ->  Tasks  ->  Execution loop  ->  Review
```

| Stage | Artifact | Where |
| --- | --- | --- |
| PRD (business) | what + why | `docs/PRD.md` |
| Tech Spec (design/arch) | how + decisions + risks | `docs/ARCHITECTURE.md` (+ ADRs) |
| Tasks | small implementable units | `tasks/00x-*.md` |
| Execution | code + tests | the crate + `prompts/03` |
| Review | QA + a11y check | `prompts/04`, `prompts/05` |

No code is written before the relevant PRD/Tech Spec/Task exists.

## Agent roles (prompts)

Each prompt in `prompts/` is a reusable agent role. Run them in order:

1. `01-context-review.md` - read all docs, return architecture/risks/order, no code.
2. `02-next-task-planning.md` - Planning Agent: produce the next small task spec.
3. `03-implementation.md` - Implementation Agent: implement only the current task.
4. `04-qa-review.md` - QA Agent: pass/fail against acceptance criteria.
5. `05-accessibility-review.md` - a11y review: WCAG, severity, messages, suggestions.

## Manual loop (Cursor only)

For one task:

1. Open the task file in `tasks/`.
2. Use `prompts/03-implementation.md` with that task as the current task.
3. Run `cargo test -p a11y_engine`. On failure, feed the error back and fix.
4. Run `prompts/04-qa-review.md`, then `prompts/05-accessibility-review.md`.
5. Mark the task `status: done` and move to the next.

## Orchestrated loop (Compozy)

Compozy automates the loop above (the "Looper"). It runs each task in its own
flow with memory between runs, and can fetch/fix review comments.

### Bootstrap (one time)

```bash
# 1. Install the Compozy CLI (see https://github.com/compozy/compozy)
# 2. Install core workflow skills into Cursor:
compozy setup            # interactive: pick agents/skills (choose cursor)
# or:
compozy setup --all
```

### Run the loop

```bash
# Validate task metadata against the v2 schema
compozy tasks validate --tasks-dir .compozy/tasks/a11y-mvp

# Run the tasks through the Looper using Cursor as runtime
compozy tasks run --ide cursor --tasks-dir .compozy/tasks/a11y-mvp

# After a PR exists, pull review comments and auto-fix
compozy reviews fetch
compozy reviews fix --ide cursor
```

Notes:

- Reusable agents live in `.compozy/agents/<name>/AGENT.md` (committed).
- Workflow artifacts that Compozy generates live under `.compozy/tasks/<workflow>/`
  (PRDs, tech specs, generated tasks, `memory/`). We do not hand-author those.
- Our committed `tasks/` and `docs/PRD.md` are the human-authored source of truth
  that seed the Compozy workflow.

## Definition of done (every change)

- Task spec existed before implementation.
- Tests added (passing / failing / edge for rules).
- Architecture boundaries respected (pure Rust core, isolated rules).
- Implementation report: summary, changed files, tests, validation, limitations.
