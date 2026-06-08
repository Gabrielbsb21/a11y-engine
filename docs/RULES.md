# A11y Engine - Rules Catalog

## Purpose

This document defines all accessibility rules supported by the engine.

Each rule has:

* Code
* Severity
* Description
* WCAG Reference

---

# Image Rules

## IMG_MISSING_ALT

Severity: Error

Description:

Image elements must provide alternative text.

WCAG:

1.1.1 Non-text Content

---

## IMG_SUSPICIOUS_ALT

Severity: Warning

Description:

Alternative text appears generic or meaningless.

Examples:

* image
* photo
* img

WCAG:

1.1.1 Non-text Content

---

# Form Rules

## INPUT_MISSING_LABEL

Severity: Error

Description:

Form controls must have an associated label.

WCAG:

1.3.1
3.3.2

---

## BUTTON_MISSING_NAME

Severity: Error

Description:

Buttons must expose an accessible name.

WCAG:

4.1.2

---

# Structure Rules

## MULTIPLE_H1

Severity: Warning

Description:

More than one H1 detected.

---

## HEADING_ORDER_INVALID

Severity: Warning

Description:

Heading levels should not skip hierarchy.

Example:

H1 → H3

WCAG:

1.3.1

---

# Interaction Rules

## CLICKABLE_NON_SEMANTIC

Severity: Error

Description:

Interactive behavior detected on non-semantic elements.

Examples:

* div onclick
* span onclick

WCAG:

2.1.1

---

## MISSING_KEYBOARD_SUPPORT

Severity: Error

Description:

Interactive elements must be operable using keyboard.

WCAG:

2.1.1

---

# ARIA Rules

## EMPTY_ARIA_LABEL

Severity: Error

Description:

aria-label cannot be empty.

WCAG:

4.1.2

---

## INVALID_ARIA_ATTRIBUTE

Severity: Error

Description:

Invalid ARIA attribute detected.

WCAG:

4.1.2

---

# Landmark Rules

## MISSING_MAIN

Severity: Warning

Description:

Document does not contain a main landmark.

WCAG:

1.3.1

---

## MULTIPLE_MAIN

Severity: Warning

Description:

More than one main landmark found.

WCAG:

1.3.1

---

# Future Rules

Contrast

* TEXT_CONTRAST_FAIL

Focus

* MISSING_VISIBLE_FOCUS

Tables

* TABLE_MISSING_HEADERS

Media

* VIDEO_MISSING_CAPTIONS

Navigation

* SKIP_LINK_MISSING

Accessibility Tree

* INVALID_READING_ORDER
* DUPLICATE_ACCESSIBLE_NAME
