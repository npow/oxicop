//! Lint cops for detecting code quality issues.

use regex::Regex;
use std::collections::HashMap;

use crate::cop::{Category, Cop, Severity};
use crate::offense::{Location, Offense};
use crate::source::SourceFile;

/// Detects leftover debugging calls like `binding.pry`, `byebug`, etc.
pub struct Debugger {
    patterns: Vec<(&'static str, Regex)>,
}

impl Debugger {
    pub fn new() -> Self {
        let debug_methods = vec![
            "binding.pry",
            "binding.irb",
            "byebug",
            "debugger",
            "binding.break",
            "pry",
            "save_and_open_page",
            "save_and_open_screenshot",
        ];

        let patterns = debug_methods
            .into_iter()
            .map(|method| {
                // Use word boundaries to match whole words/expressions
                // \b works for word characters, but for dots we need special handling
                let pattern = if method.contains('.') {
                    // For dotted expressions, match them exactly with word boundary at start and end
                    format!(r"\b{}\b", regex::escape(method))
                } else {
                    // For single words, use simple word boundaries
                    format!(r"\b{}\b", regex::escape(method))
                };
                (method, Regex::new(&pattern).unwrap())
            })
            .collect();

        Self { patterns }
    }
}

impl Default for Debugger {
    fn default() -> Self {
        Self::new()
    }
}

impl Cop for Debugger {
    fn name(&self) -> &str {
        "Lint/Debugger"
    }

    fn category(&self) -> Category {
        Category::Lint
    }

    fn severity(&self) -> Severity {
        Severity::Warning
    }

    fn description(&self) -> &str {
        "Checks for leftover debugging code like `binding.pry` or `byebug`."
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        for (line_num, line_content) in source.lines.iter().enumerate() {
            let line_number = line_num + 1; // Convert to 1-based

            // Quick check: if column 1 is in a comment, skip the entire line
            if source.in_string_or_comment(line_number, 1) {
                // Check if it's a comment by looking for '#' at the start or after whitespace
                if line_content.trim_start().starts_with('#') {
                    continue;
                }
            }

            // Collect all matches with their positions to avoid overlapping detections
            let mut matches: Vec<(usize, usize, &str)> = Vec::new();

            // Check each debug pattern
            for (method_name, pattern) in &self.patterns {
                for match_obj in pattern.find_iter(line_content) {
                    let start = match_obj.start();
                    let end = match_obj.end();
                    let column = start + 1; // Convert to 1-based

                    // Skip if this match is inside a string or comment
                    if source.in_string_or_comment(line_number, column) {
                        continue;
                    }

                    matches.push((start, end, method_name));
                }
            }

            // Sort by start position and filter out overlapping matches
            matches.sort_by_key(|m| m.0);
            let mut last_end = 0;
            for (start, end, method_name) in matches {
                // Skip if this match overlaps with a previous match
                if start < last_end {
                    continue;
                }

                let column = start + 1; // Convert to 1-based
                let length = end - start;

                offenses.push(Offense::new(
                    self.name(),
                    format!("Remove debugger entry point `{}`.", method_name),
                    self.severity(),
                    Location::new(line_number, column, length),
                ));

                last_end = end;
            }
        }

        offenses
    }
}

/// Detects literal values used as conditions (e.g., `if true`, `if false`, `if nil`).
pub struct LiteralInCondition {
    pattern: Regex,
}

impl LiteralInCondition {
    pub fn new() -> Self {
        // Match if/unless followed by whitespace and then true/false/nil
        // We use \b for word boundaries to avoid matching `if true_value`
        let pattern = Regex::new(r"\b(if|unless)\s+(true|false|nil)\b").unwrap();
        Self { pattern }
    }
}

impl Default for LiteralInCondition {
    fn default() -> Self {
        Self::new()
    }
}

impl Cop for LiteralInCondition {
    fn name(&self) -> &str {
        "Lint/LiteralInCondition"
    }

    fn category(&self) -> Category {
        Category::Lint
    }

    fn severity(&self) -> Severity {
        Severity::Warning
    }

    fn description(&self) -> &str {
        "Checks for literals used in conditions."
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        for (line_num, line_content) in source.lines.iter().enumerate() {
            let line_number = line_num + 1; // Convert to 1-based

            for captures in self.pattern.captures_iter(line_content) {
                let full_match = captures.get(0).unwrap();
                let keyword = captures.get(1).unwrap().as_str();
                let literal = captures.get(2).unwrap().as_str();
                let column = full_match.start() + 1; // Convert to 1-based

                // Skip if inside a string or comment
                if source.in_string_or_comment(line_number, column) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    format!(
                        "Literal `{}` used in `{}` condition.",
                        literal, keyword
                    ),
                    self.severity(),
                    Location::new(line_number, column, full_match.len()),
                ));
            }
        }

        offenses
    }
}

/// Detects duplicate method definitions in the same scope.
pub struct DuplicateMethods {
    pattern: Regex,
}

impl DuplicateMethods {
    pub fn new() -> Self {
        // Match method definitions: `def method_name`
        // Exclude class methods: `def self.method_name`
        // We capture the method name for grouping
        let pattern = Regex::new(r"^\s*def\s+([a-zA-Z_][a-zA-Z0-9_]*[?!]?)").unwrap();
        Self { pattern }
    }
}

impl Default for DuplicateMethods {
    fn default() -> Self {
        Self::new()
    }
}

impl Cop for DuplicateMethods {
    fn name(&self) -> &str {
        "Lint/DuplicateMethods"
    }

    fn category(&self) -> Category {
        Category::Lint
    }

    fn severity(&self) -> Severity {
        Severity::Warning
    }

    fn description(&self) -> &str {
        "Checks for duplicate method definitions."
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let mut method_definitions: HashMap<String, Vec<(usize, usize)>> = HashMap::new();

        // First pass: collect all method definitions
        for (line_num, line_content) in source.lines.iter().enumerate() {
            let line_number = line_num + 1; // Convert to 1-based

            if let Some(captures) = self.pattern.captures(line_content) {
                let method_name = captures.get(1).unwrap().as_str();
                let column = captures.get(0).unwrap().start() + 1; // Convert to 1-based

                // Skip if inside a string or comment
                if source.in_string_or_comment(line_number, column) {
                    continue;
                }

                method_definitions
                    .entry(method_name.to_string())
                    .or_insert_with(Vec::new)
                    .push((line_number, column));
            }
        }

        // Second pass: flag duplicates
        for (method_name, locations) in method_definitions {
            if locations.len() > 1 {
                // Report all occurrences after the first as duplicates
                for &(line_number, column) in &locations[1..] {
                    offenses.push(Offense::new(
                        self.name(),
                        format!("Method `{}` is defined multiple times.", method_name),
                        self.severity(),
                        Location::new(line_number, column, 3), // length of "def"
                    ));
                }
            }
        }

        offenses
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn test_source(content: &str) -> SourceFile {
        SourceFile::from_string(PathBuf::from("test.rb"), content.to_string())
    }

    // ===== Debugger Tests =====

    #[test]
    fn test_debugger_no_offense() {
        let cop = Debugger::new();
        let source = test_source("puts 'hello world'\nx = 42\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_debugger_detects_binding_pry() {
        let cop = Debugger::new();
        let source = test_source("def foo\n  binding.pry\n  x = 1\nend\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert_eq!(offenses[0].location.line, 2);
        assert!(offenses[0].message.contains("binding.pry"));
    }

    #[test]
    fn test_debugger_detects_byebug() {
        let cop = Debugger::new();
        let source = test_source("x = 1\nbyebug\ny = 2\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert_eq!(offenses[0].location.line, 2);
        assert!(offenses[0].message.contains("byebug"));
    }

    #[test]
    fn test_debugger_detects_debugger() {
        let cop = Debugger::new();
        let source = test_source("debugger\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert_eq!(offenses[0].location.line, 1);
        assert!(offenses[0].message.contains("debugger"));
    }

    #[test]
    fn test_debugger_detects_pry() {
        let cop = Debugger::new();
        let source = test_source("pry\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert_eq!(offenses[0].location.line, 1);
        assert!(offenses[0].message.contains("pry"));
    }

    #[test]
    fn test_debugger_skips_in_string() {
        let cop = Debugger::new();
        let source = test_source("puts 'binding.pry'\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_debugger_skips_in_double_quoted_string() {
        let cop = Debugger::new();
        let source = test_source("puts \"binding.pry\"\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_debugger_skips_in_comment() {
        let cop = Debugger::new();
        let source = test_source("# binding.pry\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_debugger_skips_in_inline_comment() {
        let cop = Debugger::new();
        let source = test_source("x = 1 # binding.pry\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_debugger_multiple_on_same_line() {
        let cop = Debugger::new();
        let source = test_source("binding.pry; byebug\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 2);
    }

    #[test]
    fn test_debugger_empty_file() {
        let cop = Debugger::new();
        let source = test_source("");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_debugger_does_not_match_partial_words() {
        let cop = Debugger::new();
        let source = test_source("my_pry_method\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_debugger_all_patterns() {
        let cop = Debugger::new();
        let source = test_source(
            "binding.pry\n\
             binding.irb\n\
             byebug\n\
             debugger\n\
             binding.break\n\
             pry\n\
             save_and_open_page\n\
             save_and_open_screenshot\n",
        );
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 8);
    }

    // ===== LiteralInCondition Tests =====

    #[test]
    fn test_literal_in_condition_no_offense() {
        let cop = LiteralInCondition::new();
        let source = test_source("if x\n  puts 'hello'\nend\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_literal_in_condition_if_true() {
        let cop = LiteralInCondition::new();
        let source = test_source("if true\n  puts 'hello'\nend\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert_eq!(offenses[0].location.line, 1);
        assert!(offenses[0].message.contains("true"));
        assert!(offenses[0].message.contains("if"));
    }

    #[test]
    fn test_literal_in_condition_if_false() {
        let cop = LiteralInCondition::new();
        let source = test_source("if false\n  puts 'hello'\nend\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert_eq!(offenses[0].location.line, 1);
        assert!(offenses[0].message.contains("false"));
    }

    #[test]
    fn test_literal_in_condition_if_nil() {
        let cop = LiteralInCondition::new();
        let source = test_source("if nil\n  puts 'hello'\nend\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert_eq!(offenses[0].location.line, 1);
        assert!(offenses[0].message.contains("nil"));
    }

    #[test]
    fn test_literal_in_condition_unless_true() {
        let cop = LiteralInCondition::new();
        let source = test_source("unless true\n  puts 'hello'\nend\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert!(offenses[0].message.contains("unless"));
    }

    #[test]
    fn test_literal_in_condition_unless_false() {
        let cop = LiteralInCondition::new();
        let source = test_source("unless false\n  puts 'hello'\nend\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert!(offenses[0].message.contains("unless"));
    }

    #[test]
    fn test_literal_in_condition_skips_while_true() {
        let cop = LiteralInCondition::new();
        let source = test_source("while true\n  break if done\nend\n");
        let offenses = cop.check(&source);
        // while true is a common idiom and should not be flagged
        // Our regex only matches if/unless, so this should pass
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_literal_in_condition_skips_in_string() {
        let cop = LiteralInCondition::new();
        let source = test_source("puts 'if true'\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_literal_in_condition_skips_in_comment() {
        let cop = LiteralInCondition::new();
        let source = test_source("# if true\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_literal_in_condition_empty_file() {
        let cop = LiteralInCondition::new();
        let source = test_source("");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_literal_in_condition_does_not_match_variables() {
        let cop = LiteralInCondition::new();
        let source = test_source("if true_value\n  puts 'hello'\nend\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_literal_in_condition_inline() {
        let cop = LiteralInCondition::new();
        let source = test_source("puts 'hello' if true\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    // ===== DuplicateMethods Tests =====

    #[test]
    fn test_duplicate_methods_no_offense() {
        let cop = DuplicateMethods::new();
        let source = test_source("def foo\nend\n\ndef bar\nend\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_duplicate_methods_detects_duplicate() {
        let cop = DuplicateMethods::new();
        let source = test_source("def foo\nend\n\ndef foo\nend\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert_eq!(offenses[0].location.line, 4);
        assert!(offenses[0].message.contains("foo"));
    }

    #[test]
    fn test_duplicate_methods_detects_multiple_duplicates() {
        let cop = DuplicateMethods::new();
        let source = test_source("def foo\nend\n\ndef foo\nend\n\ndef foo\nend\n");
        let offenses = cop.check(&source);
        // Should report 2 offenses (2nd and 3rd occurrences)
        assert_eq!(offenses.len(), 2);
    }

    #[test]
    fn test_duplicate_methods_with_question_mark() {
        let cop = DuplicateMethods::new();
        let source = test_source("def valid?\nend\n\ndef valid?\nend\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert!(offenses[0].message.contains("valid?"));
    }

    #[test]
    fn test_duplicate_methods_with_exclamation_mark() {
        let cop = DuplicateMethods::new();
        let source = test_source("def save!\nend\n\ndef save!\nend\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert!(offenses[0].message.contains("save!"));
    }

    #[test]
    fn test_duplicate_methods_skips_class_methods() {
        let cop = DuplicateMethods::new();
        let source = test_source("def self.foo\nend\n\ndef foo\nend\n");
        let offenses = cop.check(&source);
        // self.foo and foo are different, and we don't match self.foo anyway
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_duplicate_methods_skips_in_string() {
        let cop = DuplicateMethods::new();
        let source = test_source("def foo\nend\n\nputs 'def foo'\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_duplicate_methods_skips_in_comment() {
        let cop = DuplicateMethods::new();
        let source = test_source("def foo\nend\n\n# def foo\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_duplicate_methods_empty_file() {
        let cop = DuplicateMethods::new();
        let source = test_source("");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_duplicate_methods_with_indentation() {
        let cop = DuplicateMethods::new();
        let source = test_source("  def foo\n  end\n\n  def foo\n  end\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_duplicate_methods_different_methods_same_prefix() {
        let cop = DuplicateMethods::new();
        let source = test_source("def foo\nend\n\ndef foobar\nend\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }
}
