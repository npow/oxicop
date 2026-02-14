//! Oxicop - A blazing-fast Ruby linter implemented in Rust.
//!
//! This library provides the core linting infrastructure, cop implementations,
//! and output formatting for the oxicop command-line tool.

pub mod cop;
pub mod cops;
pub mod config;
pub mod formatter;
pub mod offense;
pub mod registry;
pub mod runner;
pub mod source;
