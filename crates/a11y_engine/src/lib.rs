//! A11y Engine - pure Rust accessibility analysis core.
//!
//! Modules follow the analysis pipeline described in `docs/ARCHITECTURE.md`:
//! `parser -> ast -> indexer -> rules -> analyzer -> report`.
//! These modules are currently skeletons and are implemented task by task.

pub mod analyzer;
pub mod ast;
pub mod indexer;
pub mod parser;
pub mod report;
pub mod rules;
pub mod utils;
