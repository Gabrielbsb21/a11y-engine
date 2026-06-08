---
status: todo
title: Implement IMG_MISSING_ALT rule
type: backend
complexity: low
dependencies: [004-rule-engine]
---

# Task: IMG_MISSING_ALT rule

## Goal

Implement the first concrete accessibility rule: images must provide alternative
text. See docs/RULES.md (IMG_MISSING_ALT, Error, WCAG 1.1.1).

## Scope

- A `Rule` that reports an issue for `<img>` elements missing a valid `alt`.
- Register it in the rule registry.

## Non-goals

- Other rules (INPUT_MISSING_LABEL, BUTTON_MISSING_NAME, HEADING_ORDER_INVALID).
- Suspicious-alt heuristics (IMG_SUSPICIOUS_ALT is a future rule).

## Files to create

- crates/a11y_engine/src/rules/img_missing_alt.rs

## Files to modify

- crates/a11y_engine/src/rules/mod.rs (register rule)

## Implementation details

- Trigger when an `<img>` has no `alt` attribute at all.
- Do NOT trigger when `alt=""` is present (valid for decorative images).
- Consider `role="presentation"` / `aria-hidden="true"` as compliant.
- Issue: code `IMG_MISSING_ALT`, severity Error, helpful message + suggestion,
  correct `node_path`.

## Acceptance criteria

- Matches docs/RULES.md code/severity/WCAG.
- No false positive on `alt=""` or decorative images.

## Test cases

- passing: `<img src="x" alt="A cat">` -> no issue; `<img src="x" alt="">` -> no issue.
- failing: `<img src="x">` -> one IMG_MISSING_ALT issue (Error).
- edge: `<img src="x" role="presentation">` -> no issue; nested img reports correct path.

## Review checklist

- Rule is isolated, stateless, deterministic.
- Tests cover passing / failing / edge.
- Message includes actionable remediation.
