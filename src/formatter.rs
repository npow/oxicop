//! Output formatters for linting results.

use colored::*;
use serde::Serialize;

use crate::cop::Severity;
use crate::runner::RunResult;

/// Available output formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Simple,
    Compact,
    Json,
}

impl Format {
    /// Parses a format string into a Format enum.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "simple" => Some(Format::Simple),
            "compact" => Some(Format::Compact),
            "json" => Some(Format::Json),
            _ => None,
        }
    }
}

/// Trait for formatting linting results.
pub trait Formatter {
    fn format(&self, result: &RunResult) -> String;
}

/// RuboCop's default multi-line format with colors.
pub struct SimpleFormatter {
    use_colors: bool,
}

impl SimpleFormatter {
    pub fn new(use_colors: bool) -> Self {
        Self { use_colors }
    }

    fn colorize_severity(&self, severity: Severity, text: &str) -> String {
        if !self.use_colors {
            return text.to_string();
        }
        match severity {
            Severity::Error | Severity::Fatal => text.red().to_string(),
            Severity::Warning => text.yellow().to_string(),
            Severity::Convention | Severity::Refactor => text.cyan().to_string(),
            Severity::Info => text.white().to_string(),
        }
    }
}

impl Formatter for SimpleFormatter {
    fn format(&self, result: &RunResult) -> String {
        let mut output = String::new();

        for file_result in &result.file_results {
            if file_result.offenses.is_empty() {
                continue;
            }

            // File header
            output.push_str(&format!("{}:\n", file_result.path.display()));

            // Each offense on its own line
            for offense in &file_result.offenses {
                let severity_code = offense.severity.code().to_string();
                let colored_code = self.colorize_severity(offense.severity, &severity_code);

                output.push_str(&format!(
                    "{}: {}: {} ({})\n",
                    offense.location, colored_code, offense.message, offense.cop_name
                ));
            }

            output.push('\n');
        }

        // Summary
        output.push_str(&format!(
            "{} file{} inspected, {} offense{} detected\n",
            result.total_files,
            if result.total_files == 1 { "" } else { "s" },
            result.total_offenses,
            if result.total_offenses == 1 { "" } else { "s" },
        ));

        output
    }
}

/// Compact one-line-per-offense format.
pub struct CompactFormatter;

impl Formatter for CompactFormatter {
    fn format(&self, result: &RunResult) -> String {
        let mut output = String::new();

        for file_result in &result.file_results {
            for offense in &file_result.offenses {
                output.push_str(&format!(
                    "{}:{}:{}: {}: {} ({})\n",
                    file_result.path.display(),
                    offense.location.line,
                    offense.location.column,
                    offense.severity.code(),
                    offense.message,
                    offense.cop_name
                ));
            }
        }

        output
    }
}

/// JSON output format.
pub struct JsonFormatter;

#[derive(Serialize)]
struct JsonOffense {
    path: String,
    line: usize,
    column: usize,
    length: usize,
    severity: String,
    message: String,
    cop_name: String,
}

#[derive(Serialize)]
struct JsonOutput {
    file_count: usize,
    offense_count: usize,
    offenses: Vec<JsonOffense>,
}

impl Formatter for JsonFormatter {
    fn format(&self, result: &RunResult) -> String {
        let offenses: Vec<JsonOffense> = result
            .file_results
            .iter()
            .flat_map(|file_result| {
                file_result.offenses.iter().map(|offense| JsonOffense {
                    path: file_result.path.display().to_string(),
                    line: offense.location.line,
                    column: offense.location.column,
                    length: offense.location.length,
                    severity: format!("{}", offense.severity.code()),
                    message: offense.message.clone(),
                    cop_name: offense.cop_name.clone(),
                })
            })
            .collect();

        let output = JsonOutput {
            file_count: result.total_files,
            offense_count: result.total_offenses,
            offenses,
        };

        serde_json::to_string_pretty(&output).unwrap()
    }
}

/// Creates a formatter based on the format type.
pub fn create_formatter(format: Format) -> Box<dyn Formatter> {
    match format {
        Format::Simple => Box::new(SimpleFormatter::new(true)),
        Format::Compact => Box::new(CompactFormatter),
        Format::Json => Box::new(JsonFormatter),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::offense::{Location, Offense};
    use crate::runner::FileResult;
    use std::path::PathBuf;

    fn create_test_result() -> RunResult {
        let offense1 = Offense::new(
            "Layout/TrailingWhitespace",
            "Trailing whitespace detected.",
            Severity::Convention,
            Location::new(5, 10, 2),
        );

        let offense2 = Offense::new(
            "Lint/Debugger",
            "Remove debugger statement.",
            Severity::Warning,
            Location::new(10, 3, 8),
        );

        RunResult {
            file_results: vec![FileResult {
                path: PathBuf::from("test.rb"),
                offenses: vec![offense1, offense2],
            }],
            total_files: 1,
            total_offenses: 2,
        }
    }

    #[test]
    fn test_format_from_str() {
        assert_eq!(Format::from_str("simple"), Some(Format::Simple));
        assert_eq!(Format::from_str("compact"), Some(Format::Compact));
        assert_eq!(Format::from_str("json"), Some(Format::Json));
        assert_eq!(Format::from_str("SIMPLE"), Some(Format::Simple));
        assert_eq!(Format::from_str("invalid"), None);
    }

    #[test]
    fn test_simple_formatter_no_colors() {
        let formatter = SimpleFormatter::new(false);
        let result = create_test_result();
        let output = formatter.format(&result);

        assert!(output.contains("test.rb:"));
        assert!(output.contains("5:10: C:"));
        assert!(output.contains("10:3: W:"));
        assert!(output.contains("Trailing whitespace detected."));
        assert!(output.contains("Remove debugger statement."));
        assert!(output.contains("1 file inspected, 2 offenses detected"));
    }

    #[test]
    fn test_compact_formatter() {
        let formatter = CompactFormatter;
        let result = create_test_result();
        let output = formatter.format(&result);

        assert!(output.contains("test.rb:5:10: C:"));
        assert!(output.contains("test.rb:10:3: W:"));
        assert!(output.contains("Trailing whitespace detected."));
        assert!(output.contains("(Layout/TrailingWhitespace)"));
    }

    #[test]
    fn test_json_formatter() {
        let formatter = JsonFormatter;
        let result = create_test_result();
        let output = formatter.format(&result);

        // Should be valid JSON
        let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();

        assert_eq!(parsed["file_count"], 1);
        assert_eq!(parsed["offense_count"], 2);
        assert_eq!(parsed["offenses"].as_array().unwrap().len(), 2);

        let offense = &parsed["offenses"][0];
        assert_eq!(offense["path"], "test.rb");
        assert_eq!(offense["line"], 5);
        assert_eq!(offense["column"], 10);
        assert_eq!(offense["severity"], "C");
    }

    #[test]
    fn test_empty_result() {
        let result = RunResult {
            file_results: vec![],
            total_files: 0,
            total_offenses: 0,
        };

        let formatter = SimpleFormatter::new(false);
        let output = formatter.format(&result);
        assert!(output.contains("0 files inspected, 0 offenses detected"));
    }

    #[test]
    fn test_plural_handling() {
        let result = RunResult {
            file_results: vec![],
            total_files: 1,
            total_offenses: 1,
        };

        let formatter = SimpleFormatter::new(false);
        let output = formatter.format(&result);
        assert!(output.contains("1 file inspected, 1 offense detected"));
    }

    #[test]
    fn test_create_formatter() {
        let simple = create_formatter(Format::Simple);
        let compact = create_formatter(Format::Compact);
        let json = create_formatter(Format::Json);

        let result = create_test_result();

        // Just verify they all produce some output
        assert!(!simple.format(&result).is_empty());
        assert!(!compact.format(&result).is_empty());
        assert!(!json.format(&result).is_empty());
    }
}
