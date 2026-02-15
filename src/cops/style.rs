//! Style cops for Ruby code formatting and conventions.

use regex::Regex;

use crate::cop::{Category, Cop, Severity};
use crate::offense::{Location, Offense};
use crate::source::SourceFile;

/// Checks that files start with `# frozen_string_literal: true`.
///
/// This cop enforces the Ruby best practice of declaring frozen string literals
/// at the top of each file. The comment should appear on line 1, or line 2 if
/// line 1 is a shebang (`#!`).
pub struct FrozenStringLiteralComment;

impl Cop for FrozenStringLiteralComment {
    fn name(&self) -> &str {
        "Style/FrozenStringLiteralComment"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for frozen_string_literal comment at the top of files"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        if source.is_empty() {
            return offenses;
        }

        // Determine which line should have the magic comment
        let first_line = source.line(1).unwrap_or("");
        let check_line = if first_line.starts_with("#!") {
            2
        } else {
            1
        };

        // Check if the magic comment exists on the expected line
        if let Some(line) = source.line(check_line) {
            let trimmed = line.trim();
            let is_frozen_comment = trimmed.starts_with('#')
                && trimmed[1..].trim_start().starts_with("frozen_string_literal:");
            if !is_frozen_comment {
                offenses.push(Offense::new(
                    self.name(),
                    "Missing frozen_string_literal comment",
                    self.severity(),
                    Location::new(check_line, 1, line.len()),
                ));
            } else if !trimmed.contains("true") {
                offenses.push(Offense::new(
                    self.name(),
                    "frozen_string_literal should be set to true",
                    self.severity(),
                    Location::new(check_line, 1, line.len()),
                ));
            }
        } else if check_line == 2 {
            // File only has a shebang line
            offenses.push(Offense::new(
                self.name(),
                "Missing frozen_string_literal comment",
                self.severity(),
                Location::new(1, 1, first_line.len()),
            ));
        }

        offenses
    }
}

/// Prefers single-quoted strings when no interpolation or special characters are used.
///
/// This cop detects double-quoted strings that could be single-quoted. Double quotes
/// are only necessary when the string contains interpolation (`#{}`) or escape sequences
/// like `\n`, `\t`, etc.
pub struct StringLiterals;

impl Cop for StringLiterals {
    fn name(&self) -> &str {
        "Style/StringLiterals"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Prefers single-quoted strings when no interpolation is needed"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        // Match double-quoted strings
        let string_regex = Regex::new(r#""([^"\\]*(\\.[^"\\]*)*)""#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;

            // Find all double-quoted strings on this line
            for capture in string_regex.captures_iter(line) {
                if let Some(matched) = capture.get(0) {
                    let start_col = matched.start() + 1; // 1-based
                    let content = capture.get(1).map(|m| m.as_str()).unwrap_or("");

                    // Check if this position is already inside a comment
                    if source.in_string_or_comment(line_number, start_col) {
                        continue;
                    }

                    // Check if the string needs double quotes
                    if needs_double_quotes(content) {
                        continue;
                    }

                    offenses.push(Offense::new(
                        self.name(),
                        "Prefer single-quoted strings when you don't need interpolation",
                        self.severity(),
                        Location::new(line_number, start_col, matched.len()),
                    ));
                }
            }
        }

        offenses
    }
}

/// Helper function to determine if a string needs double quotes.
fn needs_double_quotes(content: &str) -> bool {
    // Check for interpolation
    if content.contains("#{") {
        return true;
    }

    // Check for escape sequences (backslash followed by a letter or special char)
    let mut chars = content.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            if let Some(&next) = chars.peek() {
                // Common escape sequences that need double quotes
                if matches!(next, 'n' | 't' | 'r' | 'a' | 'b' | 'f' | 'v' | 'e' | '0' | 'x' | 'u' | 's' | '"') {
                    return true;
                }
            }
        }
    }

    false
}

/// Detects negated conditionals and suggests using `unless` or `if` instead.
///
/// This cop flags:
/// - `if !condition` -> suggest `unless condition`
/// - `unless !condition` -> suggest `if condition`
pub struct NegatedIf;

impl Cop for NegatedIf {
    fn name(&self) -> &str {
        "Style/NegatedIf"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Detects negated conditionals that could use unless/if instead"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        // Match "if !" or "unless !" at statement start
        let if_negated_regex = Regex::new(r#"^\s*(if\s+!)"#).unwrap();
        let unless_negated_regex = Regex::new(r#"^\s*(unless\s+!)"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;

            // Skip complex conditions with logical operators
            if line.contains("&&") || line.contains("||") {
                continue;
            }

            // Check for "if !"
            if let Some(capture) = if_negated_regex.captures(line) {
                if let Some(matched) = capture.get(1) {
                    let col = matched.start() + 1;
                    if !source.in_string_or_comment(line_number, col) {
                        offenses.push(Offense::new(
                            self.name(),
                            "Prefer `unless` over `if` with negated condition",
                            self.severity(),
                            Location::new(line_number, col, matched.len()),
                        ));
                    }
                }
            }

            // Check for "unless !"
            if let Some(capture) = unless_negated_regex.captures(line) {
                if let Some(matched) = capture.get(1) {
                    let col = matched.start() + 1;
                    if !source.in_string_or_comment(line_number, col) {
                        offenses.push(Offense::new(
                            self.name(),
                            "Prefer `if` over `unless` with negated condition",
                            self.severity(),
                            Location::new(line_number, col, matched.len()),
                        ));
                    }
                }
            }
        }

        offenses
    }
}

/// Detects redundant `return` statements at the end of method bodies.
///
/// Ruby methods implicitly return the last evaluated expression, so an explicit
/// `return` on the final line is redundant.
pub struct RedundantReturn;

impl Cop for RedundantReturn {
    fn name(&self) -> &str {
        "Style/RedundantReturn"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Detects redundant return statements at the end of method bodies"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        // Track method definitions and their end statements
        let mut method_stack: Vec<usize> = Vec::new(); // Stack of def line numbers

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            let trimmed = line.trim();

            // Detect method definition start
            if trimmed.starts_with("def ") {
                method_stack.push(line_number);
            }

            // Detect end statement
            if trimmed == "end" && !method_stack.is_empty() {
                let def_line = method_stack.pop().unwrap();
                
                // Find the last non-empty line before this "end"
                let mut last_statement_line = None;
                for i in (def_line..line_number).rev() {
                    if let Some(prev_line) = source.line(i) {
                        let prev_trimmed = prev_line.trim();
                        if !prev_trimmed.is_empty() && !prev_trimmed.starts_with("def ") {
                            last_statement_line = Some(i);
                            break;
                        }
                    }
                }

                // Check if the last statement is a return
                if let Some(stmt_line_num) = last_statement_line {
                    if let Some(stmt_line) = source.line(stmt_line_num) {
                        let stmt_trimmed = stmt_line.trim();
                        if stmt_trimmed.starts_with("return ") || stmt_trimmed == "return" {
                            // Find the position of "return" in the original line
                            if let Some(return_pos) = stmt_line.find("return") {
                                offenses.push(Offense::new(
                                    self.name(),
                                    "Redundant `return` at end of method body",
                                    self.severity(),
                                    Location::new(stmt_line_num, return_pos + 1, 6),
                                ));
                            }
                        }
                    }
                }
            }
        }

        offenses
    }
}

/// Detects empty method definitions.
///
/// This cop flags methods where `def` and `end` have nothing between them,
/// or only blank lines.
pub struct EmptyMethod;

impl Cop for EmptyMethod {
    fn name(&self) -> &str {
        "Style/EmptyMethod"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Detects empty method definitions"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        let mut method_stack: Vec<usize> = Vec::new();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            let trimmed = line.trim();

            if trimmed.starts_with("def ") {
                method_stack.push(line_number);
            }

            if trimmed == "end" && !method_stack.is_empty() {
                let def_line = method_stack.pop().unwrap();

                // Check if there are any non-empty lines between def and end
                let mut has_content = false;
                for i in (def_line + 1)..line_number {
                    if let Some(middle_line) = source.line(i) {
                        if !middle_line.trim().is_empty() {
                            has_content = true;
                            break;
                        }
                    }
                }

                if !has_content {
                    // Method is empty
                    if let Some(def_line_str) = source.line(def_line) {
                        if let Some(def_pos) = def_line_str.find("def ") {
                            offenses.push(Offense::new(
                                self.name(),
                                "Empty method definition",
                                self.severity(),
                                Location::new(def_line, def_pos + 1, 3),
                            ));
                        }
                    }
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

    // ===== FrozenStringLiteralComment Tests =====

    #[test]
    fn test_frozen_string_literal_present() {
        let source = test_source("# frozen_string_literal: true\n\nclass Foo\nend\n");
        let cop = FrozenStringLiteralComment;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_frozen_string_literal_missing() {
        let source = test_source("class Foo\nend\n");
        let cop = FrozenStringLiteralComment;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert!(offenses[0].message.contains("Missing"));
    }

    #[test]
    fn test_frozen_string_literal_after_shebang() {
        let source = test_source("#!/usr/bin/env ruby\n# frozen_string_literal: true\n\nclass Foo\nend\n");
        let cop = FrozenStringLiteralComment;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_frozen_string_literal_missing_after_shebang() {
        let source = test_source("#!/usr/bin/env ruby\nclass Foo\nend\n");
        let cop = FrozenStringLiteralComment;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_frozen_string_literal_empty_file() {
        let source = test_source("");
        let cop = FrozenStringLiteralComment;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_frozen_string_literal_false() {
        let source = test_source("# frozen_string_literal: false\n\nclass Foo\nend\n");
        let cop = FrozenStringLiteralComment;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert!(offenses[0].message.contains("true"));
    }

    #[test]
    fn test_frozen_string_literal_with_extra_spaces() {
        let source = test_source("#  frozen_string_literal:  true\n\nclass Foo\nend\n");
        let cop = FrozenStringLiteralComment;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_frozen_string_literal_only_shebang() {
        let source = test_source("#!/usr/bin/env ruby\n");
        let cop = FrozenStringLiteralComment;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    // ===== StringLiterals Tests =====

    #[test]
    fn test_string_literals_single_quotes_ok() {
        let source = test_source("x = 'hello world'\n");
        let cop = StringLiterals;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_string_literals_double_quotes_unnecessary() {
        let source = test_source("x = \"hello world\"\n");
        let cop = StringLiterals;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert!(offenses[0].message.contains("single-quoted"));
    }

    #[test]
    fn test_string_literals_interpolation_ok() {
        let source = test_source("x = \"hello #{name}\"\n");
        let cop = StringLiterals;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_string_literals_escape_sequences_ok() {
        let source = test_source("x = \"hello\\nworld\"\n");
        let cop = StringLiterals;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_string_literals_tab_escape_ok() {
        let source = test_source("x = \"hello\\tworld\"\n");
        let cop = StringLiterals;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_string_literals_escaped_quote() {
        let source = test_source("x = \"hello \\\"world\\\"\"\n");
        let cop = StringLiterals;
        let offenses = cop.check(&source);
        // Escaped quotes alone shouldn't require double quotes in our simplified model
        // But with backslash-letter detection, this might trigger
        // Let's accept 0 offenses since \" doesn't match our pattern
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_string_literals_empty_string() {
        let source = test_source("x = \"\"\n");
        let cop = StringLiterals;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_string_literals_multiple_strings() {
        let source = test_source("x = \"hello\"\ny = \"world\"\nz = 'ok'\n");
        let cop = StringLiterals;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 2);
    }

    #[test]
    fn test_string_literals_in_comment() {
        let source = test_source("# This is a \"comment\"\nx = 'hello'\n");
        let cop = StringLiterals;
        let offenses = cop.check(&source);
        // The string in the comment shouldn't be flagged
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_string_literals_unicode_escape() {
        let source = test_source("x = \"hello\\u0041\"\n");
        let cop = StringLiterals;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // ===== NegatedIf Tests =====

    #[test]
    fn test_negated_if_detected() {
        let source = test_source("if !condition\n  do_something\nend\n");
        let cop = NegatedIf;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert!(offenses[0].message.contains("unless"));
    }

    #[test]
    fn test_negated_unless_detected() {
        let source = test_source("unless !condition\n  do_something\nend\n");
        let cop = NegatedIf;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert!(offenses[0].message.contains("if"));
    }

    #[test]
    fn test_negated_if_normal_if() {
        let source = test_source("if condition\n  do_something\nend\n");
        let cop = NegatedIf;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_negated_if_normal_unless() {
        let source = test_source("unless condition\n  do_something\nend\n");
        let cop = NegatedIf;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_negated_if_with_logical_operators() {
        let source = test_source("if !a && b\n  do_something\nend\n");
        let cop = NegatedIf;
        let offenses = cop.check(&source);
        // Should be skipped due to && operator
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_negated_if_with_or_operator() {
        let source = test_source("if !a || b\n  do_something\nend\n");
        let cop = NegatedIf;
        let offenses = cop.check(&source);
        // Should be skipped due to || operator
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_negated_if_indented() {
        let source = test_source("  if !condition\n    do_something\n  end\n");
        let cop = NegatedIf;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_negated_if_in_string() {
        let source = test_source("x = \"if !condition\"\n");
        let cop = NegatedIf;
        let offenses = cop.check(&source);
        // Should not flag patterns in strings
        assert_eq!(offenses.len(), 0);
    }

    // ===== RedundantReturn Tests =====

    #[test]
    fn test_redundant_return_detected() {
        let source = test_source("def foo\n  return 42\nend\n");
        let cop = RedundantReturn;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert!(offenses[0].message.contains("Redundant"));
    }

    #[test]
    fn test_redundant_return_no_return() {
        let source = test_source("def foo\n  42\nend\n");
        let cop = RedundantReturn;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_redundant_return_early_return() {
        let source = test_source("def foo\n  return 0 if error\n  42\nend\n");
        let cop = RedundantReturn;
        let offenses = cop.check(&source);
        // Early return is fine, only last statement matters
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_redundant_return_bare_return() {
        let source = test_source("def foo\n  return\nend\n");
        let cop = RedundantReturn;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_redundant_return_empty_method() {
        let source = test_source("def foo\nend\n");
        let cop = RedundantReturn;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_redundant_return_multiple_methods() {
        let source = test_source("def foo\n  return 1\nend\n\ndef bar\n  2\nend\n");
        let cop = RedundantReturn;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_redundant_return_nested_methods() {
        let source = test_source("def outer\n  def inner\n    return 1\n  end\n  2\nend\n");
        let cop = RedundantReturn;
        let offenses = cop.check(&source);
        // Should detect the redundant return in inner method
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_redundant_return_with_blank_lines() {
        let source = test_source("def foo\n  return 42\n\nend\n");
        let cop = RedundantReturn;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    // ===== EmptyMethod Tests =====

    #[test]
    fn test_empty_method_detected() {
        let source = test_source("def foo\nend\n");
        let cop = EmptyMethod;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert!(offenses[0].message.contains("Empty"));
    }

    #[test]
    fn test_empty_method_with_blank_lines() {
        let source = test_source("def foo\n\n\nend\n");
        let cop = EmptyMethod;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_empty_method_not_empty() {
        let source = test_source("def foo\n  42\nend\n");
        let cop = EmptyMethod;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_empty_method_multiple() {
        let source = test_source("def foo\nend\n\ndef bar\nend\n");
        let cop = EmptyMethod;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 2);
    }

    #[test]
    fn test_empty_method_nested() {
        let source = test_source("def outer\n  def inner\n  end\n  42\nend\n");
        let cop = EmptyMethod;
        let offenses = cop.check(&source);
        // Only inner is empty
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_empty_method_with_comment() {
        let source = test_source("def foo\n  # TODO: implement\nend\n");
        let cop = EmptyMethod;
        let offenses = cop.check(&source);
        // Comment counts as content
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_empty_method_one_liner_not_supported() {
        // This syntax "def foo; end" would need special handling
        // For now, our implementation handles multi-line only
        let source = test_source("def foo; end\n");
        let cop = EmptyMethod;
        let offenses = cop.check(&source);
        // This might not be caught depending on how we parse
        // Our current implementation looks for "end" on its own line
        assert_eq!(offenses.len(), 0);
    }

    // ===== Integration Tests =====

    #[test]
    fn test_all_cops_on_clean_file() {
        let source = test_source(
            "# frozen_string_literal: true\n\
             \n\
             def foo\n\
               'hello'\n\
             end\n"
        );
        
        let frozen = FrozenStringLiteralComment;
        let strings = StringLiterals;
        let negated = NegatedIf;
        let redundant = RedundantReturn;
        let empty = EmptyMethod;

        assert_eq!(frozen.check(&source).len(), 0);
        assert_eq!(strings.check(&source).len(), 0);
        assert_eq!(negated.check(&source).len(), 0);
        assert_eq!(redundant.check(&source).len(), 0);
        assert_eq!(empty.check(&source).len(), 0);
    }

    #[test]
    fn test_all_cops_on_problematic_file() {
        let source = test_source(
            "class Foo\n\
             def bar\n\
               x = \"hello\"\n\
               if !x.empty?\n\
                 return x\n\
               end\n\
             end\n\
             \n\
             def baz\n\
             end\n\
             end\n"
        );
        
        let frozen = FrozenStringLiteralComment;
        let strings = StringLiterals;
        let negated = NegatedIf;
        let redundant = RedundantReturn;
        let empty = EmptyMethod;

        assert_eq!(frozen.check(&source).len(), 1); // Missing frozen_string_literal
        assert_eq!(strings.check(&source).len(), 1); // "hello" should be 'hello'
        assert_eq!(negated.check(&source).len(), 1); // if !x.empty? -> unless x.empty?
        assert_eq!(redundant.check(&source).len(), 1); // return x is redundant
        assert_eq!(empty.check(&source).len(), 1); // baz is empty
    }
}
