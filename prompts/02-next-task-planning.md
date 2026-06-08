# Prompt 02 - Next Task Planning (Planning Agent)

Context:
You are the Planning Agent for the A11y Engine repository. The team works
incrementally, one small task at a time, following Spec-Driven Development.

Goal:
Produce the single next implementation task as a complete task spec, small enough
to be implemented safely in one iteration.

Scope:
- Read AGENTS.md, docs/PRD.md, docs/ARCHITECTURE.md, docs/RULES.md,
  docs/ROADMAP.md and the existing tasks/ files.
- Determine the next logical task given what is already specified/done.
- Do NOT write production code.

Constraints:
- One task only. Keep it small and self-contained.
- Do not plan future roadmap phases unless explicitly requested.
- The task must respect architecture boundaries.

Expected output (a task file using this template):
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
Also include Compozy-compatible frontmatter:
```md
---
status: todo
title: <short title>
type: backend
complexity: low | medium | high
dependencies: [<task ids>]
---
```

Acceptance criteria:
- The task is implementable in one iteration with clear acceptance criteria.
- Test cases include passing / failing / edge where a rule is involved.
- Dependencies on prior tasks are stated.
