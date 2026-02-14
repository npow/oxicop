use std::fmt;

use crate::cop::Severity;

/// A source location pointing to a specific range in a file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Location {
    /// 1-based line number.
    pub line: usize,
    /// 1-based column number.
    pub column: usize,
    /// Length of the offending span in characters.
    pub length: usize,
}

impl Location {
    pub fn new(line: usize, column: usize, length: usize) -> Self {
        Self {
            line,
            column,
            length,
        }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

/// A single offense (diagnostic) reported by a cop.
#[derive(Debug, Clone)]
pub struct Offense {
    /// Fully qualified cop name, e.g. "Layout/TrailingWhitespace".
    pub cop_name: String,
    /// Human-readable message describing the offense.
    pub message: String,
    /// Severity of the offense.
    pub severity: Severity,
    /// Location in the source file.
    pub location: Location,
}

impl Offense {
    pub fn new(
        cop_name: impl Into<String>,
        message: impl Into<String>,
        severity: Severity,
        location: Location,
    ) -> Self {
        Self {
            cop_name: cop_name.into(),
            message: message.into(),
            severity,
            location,
        }
    }
}

impl fmt::Display for Offense {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}: {} ({})",
            self.location, self.severity, self.message, self.cop_name
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_location_display() {
        let loc = Location::new(10, 5, 3);
        assert_eq!(loc.to_string(), "10:5");
    }

    #[test]
    fn test_offense_display() {
        let offense = Offense::new(
            "Layout/TrailingWhitespace",
            "Trailing whitespace detected.",
            Severity::Convention,
            Location::new(1, 10, 2),
        );
        assert_eq!(
            offense.to_string(),
            "1:10: C: Trailing whitespace detected. (Layout/TrailingWhitespace)"
        );
    }
}
