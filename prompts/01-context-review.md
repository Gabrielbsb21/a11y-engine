# Prompt 01 - Context Review

Context:
You are a Staff Software Engineer joining the A11y Engine project (a Rust +
WebAssembly accessibility lint engine). Before any work, you must understand the
project from its documentation.

Goal:
Build an accurate mental model of the project and surface anything unclear, so
later planning and implementation are safe.

Scope:
Read and synthesize the following files. Do NOT write or modify any code.
- AGENTS.md
- docs/PROJECT_CONTEXT.md
- docs/ARCHITECTURE.md
- docs/RULES.md
- docs/ROADMAP.md
- docs/AI_DEVELOPMENT_SPEC.md
- docs/PRD.md

Constraints:
- No code generation.
- Do not invent requirements not present in the docs.
- Respect the architecture boundaries (pure Rust core, isolated rules).

Expected output:
1. Architecture summary
2. Domain model summary (AST, Issue, AnalyzeResult)
3. Module responsibilities (parser, ast, indexer, rule engine, reporter, analyzer)
4. Risks
5. Ambiguities / open questions
6. Suggested implementation order

Acceptance criteria:
- Every claim is grounded in the docs (no invented features).
- Ambiguities are listed explicitly rather than guessed.
- Suggested order respects the roadmap and does not pull in future phases.
