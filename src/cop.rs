use std::fmt;

use crate::offense::Offense;
use crate::source::SourceFile;

/// Categories of cops, matching RuboCop's categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Category {
    Layout,
    Style,
    Lint,
    Naming,
    Metrics,
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Category::Layout => write!(f, "Layout"),
            Category::Style => write!(f, "Style"),
            Category::Lint => write!(f, "Lint"),
            Category::Naming => write!(f, "Naming"),
            Category::Metrics => write!(f, "Metrics"),
        }
    }
}

/// Severity levels matching RuboCop's severity levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Severity {
    Info,
    Refactor,
    Convention,
    Warning,
    Error,
    Fatal,
}

impl Severity {
    /// Single-character code used in RuboCop-compatible output.
    pub fn code(&self) -> char {
        match self {
            Severity::Info => 'I',
            Severity::Refactor => 'R',
            Severity::Convention => 'C',
            Severity::Warning => 'W',
            Severity::Error => 'E',
            Severity::Fatal => 'F',
        }
    }
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code())
    }
}

/// The core trait that all cops must implement.
pub trait Cop: Send + Sync {
    /// Fully qualified name, e.g. "Layout/TrailingWhitespace".
    fn name(&self) -> &str;

    /// Category of the cop.
    fn category(&self) -> Category;

    /// Default severity.
    fn severity(&self) -> Severity;

    /// Human-readable description.
    fn description(&self) -> &str;

    /// Check a source file and return all offenses found.
    fn check(&self, source: &SourceFile) -> Vec<Offense>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_severity_ordering() {
        assert!(Severity::Info < Severity::Convention);
        assert!(Severity::Convention < Severity::Warning);
        assert!(Severity::Warning < Severity::Error);
        assert!(Severity::Error < Severity::Fatal);
    }

    #[test]
    fn test_severity_codes() {
        assert_eq!(Severity::Convention.code(), 'C');
        assert_eq!(Severity::Warning.code(), 'W');
        assert_eq!(Severity::Error.code(), 'E');
    }

    #[test]
    fn test_category_display() {
        assert_eq!(Category::Layout.to_string(), "Layout");
        assert_eq!(Category::Style.to_string(), "Style");
        assert_eq!(Category::Lint.to_string(), "Lint");
        assert_eq!(Category::Naming.to_string(), "Naming");
    }
}
