//! Layout cops - checks for whitespace, indentation, and formatting issues.

use regex::Regex;
use std::sync::OnceLock;

use crate::cop::{Category, Cop, Severity};
use crate::offense::{Location, Offense};
use crate::source::SourceFile;

// ============================================================================
// 1. TrailingWhitespace
// ============================================================================

/// Detects trailing whitespace at end of lines.
pub struct TrailingWhitespace;

impl Cop for TrailingWhitespace {
    fn name(&self) -> &str {
        "Layout/TrailingWhitespace"
    }

    fn category(&self) -> Category {
        Category::Layout
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for trailing whitespace at the end of lines"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            // Skip blank lines (lines that are all whitespace)
            if line.trim().is_empty() {
                continue;
            }

            // Check if line ends with whitespace
            if let Some(stripped) = line.strip_suffix(|c: char| c.is_whitespace()) {
                let trailing_start = stripped.len() + 1; // 1-based column
                let trailing_len = line.len() - stripped.len();
                
                offenses.push(Offense::new(
                    self.name(),
                    "Trailing whitespace detected.",
                    self.severity(),
                    Location::new(line_number, trailing_start, trailing_len),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// 2. TrailingEmptyLines
// ============================================================================

/// Detects trailing blank lines at end of file.
pub struct TrailingEmptyLines;

impl Cop for TrailingEmptyLines {
    fn name(&self) -> &str {
        "Layout/TrailingEmptyLines"
    }

    fn category(&self) -> Category {
        Category::Layout
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for trailing blank lines at the end of the file"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        if source.is_empty() {
            return offenses;
        }

        // File should end with exactly one newline
        // Check if content ends with \n
        let has_final_newline = source.content.ends_with('\n');
        
        if !has_final_newline {
            // Missing final newline
            let last_line = source.line_count();
            if let Some(line) = source.line(last_line) {
                offenses.push(Offense::new(
                    self.name(),
                    "File should end with a newline.",
                    self.severity(),
                    Location::new(last_line, line.len() + 1, 0),
                ));
            }
        } else {
            // Check for multiple trailing newlines
            let trimmed = source.content.trim_end_matches('\n');
            let newline_count = source.content.len() - trimmed.len();
            
            if newline_count > 1 {
                // Too many trailing newlines
                let last_content_line = source.lines.iter().rposition(|l| !l.is_empty()).map(|i| i + 1).unwrap_or(0);
                
                if last_content_line < source.line_count() {
                    offenses.push(Offense::new(
                        self.name(),
                        format!("{} trailing blank lines detected.", newline_count - 1),
                        self.severity(),
                        Location::new(last_content_line + 1, 1, 0),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 3. LeadingEmptyLines
// ============================================================================

/// Detects empty lines at the start of a file.
pub struct LeadingEmptyLines;

impl Cop for LeadingEmptyLines {
    fn name(&self) -> &str {
        "Layout/LeadingEmptyLines"
    }

    fn category(&self) -> Category {
        Category::Layout
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for empty lines at the start of a file"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        if source.is_empty() {
            return offenses;
        }

        // Count leading empty lines
        let mut leading_empty = 0;
        for line in &source.lines {
            if line.is_empty() {
                leading_empty += 1;
            } else {
                break;
            }
        }

        if leading_empty > 0 {
            offenses.push(Offense::new(
                self.name(),
                format!("{} leading empty lines detected.", leading_empty),
                self.severity(),
                Location::new(1, 1, 0),
            ));
        }

        offenses
    }
}

// ============================================================================
// 4. EndOfLine
// ============================================================================

/// Detects CRLF line endings (should be LF only).
pub struct EndOfLine;

impl Cop for EndOfLine {
    fn name(&self) -> &str {
        "Layout/EndOfLine"
    }

    fn category(&self) -> Category {
        Category::Layout
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for CRLF line endings (should be LF only)"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        // Check the raw content for \r\n
        let mut line_number = 1;
        let mut last_pos = 0;
        
        for (pos, _) in source.content.match_indices("\r\n") {
            // Count how many newlines we've passed to get the line number
            line_number += source.content[last_pos..pos].matches('\n').count();
            
            offenses.push(Offense::new(
                self.name(),
                "Use LF (\\n) instead of CRLF (\\r\\n).",
                self.severity(),
                Location::new(line_number, 1, 0),
            ));
            
            last_pos = pos + 2;
            line_number += 1;
        }

        offenses
    }
}

// ============================================================================
// 5. IndentationStyle
// ============================================================================

/// Detects tabs used for indentation (should be spaces).
pub struct IndentationStyle;

impl Cop for IndentationStyle {
    fn name(&self) -> &str {
        "Layout/IndentationStyle"
    }

    fn category(&self) -> Category {
        Category::Layout
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for tabs used for indentation (should be spaces)"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            // Check for leading tabs
            if let Some(pos) = line.find('\t') {
                // Only report if tab is in leading whitespace
                let leading_ws = line.chars().take_while(|c| c.is_whitespace()).collect::<String>();
                if leading_ws.contains('\t') {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use spaces for indentation, not tabs.",
                        self.severity(),
                        Location::new(line_number, pos + 1, 1),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 6. IndentationWidth
// ============================================================================

/// Detects wrong indentation width (default 2 spaces).
pub struct IndentationWidth;

impl IndentationWidth {
    /// Get the expected indentation level for a line based on keywords.
    fn expected_indent_change(line: &str) -> i32 {
        let trimmed = line.trim();
        
        // Check for end keyword (decreases indentation)
        if trimmed == "end" || trimmed.starts_with("end ") {
            return -1;
        }
        
        // Check for keywords that increase indentation
        // Patterns: def/class/module/if/unless/while/until/for/case/begin followed by space or EOL
        // Also: do blocks
        let increases = [
            "def ", "class ", "module ", "if ", "unless ", "while ", 
            "until ", "for ", "case ", "begin ", "do ",
        ];
        
        for keyword in &increases {
            if trimmed.starts_with(keyword) {
                return 1;
            }
        }
        
        // Check for line ending with do
        if trimmed.ends_with(" do") {
            return 1;
        }
        
        // when/rescue/ensure/else/elsif are special: they dedent then indent
        if trimmed.starts_with("when ") || trimmed.starts_with("else") || 
           trimmed.starts_with("elsif ") || trimmed.starts_with("rescue") || 
           trimmed.starts_with("ensure") {
            return 0; // Same level as the opening keyword
        }
        
        0
    }
    
    fn get_indentation_level(line: &str) -> usize {
        line.chars().take_while(|c| *c == ' ').count()
    }
}

impl Cop for IndentationWidth {
    fn name(&self) -> &str {
        "Layout/IndentationWidth"
    }

    fn category(&self) -> Category {
        Category::Layout
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for incorrect indentation width (default 2 spaces)"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        const INDENT_WIDTH: usize = 2;
        
        if source.is_empty() {
            return offenses;
        }

        let mut expected_indent = 0;
        
        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            // Skip empty lines
            if line.trim().is_empty() {
                continue;
            }
            
            let trimmed = line.trim();
            
            // Check for end/when/else/elsif/rescue/ensure - these dedent first
            if (trimmed.starts_with("end") || trimmed.starts_with("when ") ||
               trimmed.starts_with("else") || trimmed.starts_with("elsif ") ||
               trimmed.starts_with("rescue") || trimmed.starts_with("ensure"))
               && expected_indent > 0
            {
                expected_indent -= INDENT_WIDTH;
            }
            
            let actual_indent = Self::get_indentation_level(line);
            
            // Check if indentation matches expected
            if actual_indent != expected_indent {
                offenses.push(Offense::new(
                    self.name(),
                    format!(
                        "Incorrect indentation: expected {} spaces, found {}.",
                        expected_indent, actual_indent
                    ),
                    self.severity(),
                    Location::new(line_number, 1, actual_indent.max(1)),
                ));
            }
            
            // Update expected indentation for next line
            let change = Self::expected_indent_change(line);
            if change > 0 {
                expected_indent += INDENT_WIDTH;
            } else if change < 0 && expected_indent >= INDENT_WIDTH {
                expected_indent -= INDENT_WIDTH;
            }
        }

        offenses
    }
}

// ============================================================================
// 7. SpaceAfterComma
// ============================================================================

/// Detects missing space after commas.
pub struct SpaceAfterComma;

impl Cop for SpaceAfterComma {
    fn name(&self) -> &str {
        "Layout/SpaceAfterComma"
    }

    fn category(&self) -> Category {
        Category::Layout
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for missing space after commas"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            let chars: Vec<char> = line.chars().collect();
            
            for (i, &ch) in chars.iter().enumerate() {
                if ch == ',' {
                    // Skip if inside string or comment
                    if source.in_string_or_comment(line_number, i + 1) {
                        continue;
                    }
                    
                    // Check if followed by non-space (and not end of line)
                    if i + 1 < chars.len() {
                        let next_char = chars[i + 1];
                        if next_char != ' ' && next_char != '\t' && next_char != '\n' {
                            offenses.push(Offense::new(
                                self.name(),
                                "Space missing after comma.",
                                self.severity(),
                                Location::new(line_number, i + 2, 1),
                            ));
                        }
                    }
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 8. SpaceAroundOperators
// ============================================================================

/// Detects missing spaces around operators.
pub struct SpaceAroundOperators;

impl SpaceAroundOperators {
    fn operator_regex() -> &'static Regex {
        static RE: OnceLock<Regex> = OnceLock::new();
        RE.get_or_init(|| {
            // Match operators: =>, ==, !=, <=, >=, +=, -=, &&, ||, =, +, -, *, /
            // But we need to check context to avoid unary operators
            Regex::new(r"(=>|==|!=|<=|>=|\+=|-=|&&|\|\||[=+\-*/])").unwrap()
        })
    }
    
    fn is_likely_unary(line: &str, pos: usize, op: &str) -> bool {
        // Check if operator is likely unary (not binary)
        if op != "+" && op != "-" {
            return false;
        }
        
        // Look at what comes before
        if pos == 0 {
            return true;
        }
        
        let before = &line[..pos].trim_end();
        if before.is_empty() {
            return true;
        }
        
        // Check if preceded by operator or opening punctuation
        let last_char = before.chars().last().unwrap_or(' ');
        matches!(last_char, '=' | '(' | '[' | '{' | ',' | '!' | '<' | '>' | '&' | '|' | '+' | '-' | '*' | '/')
    }
}

impl Cop for SpaceAroundOperators {
    fn name(&self) -> &str {
        "Layout/SpaceAroundOperators"
    }

    fn category(&self) -> Category {
        Category::Layout
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for missing spaces around operators"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let re = Self::operator_regex();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for cap in re.find_iter(line) {
                let pos = cap.start();
                let op = cap.as_str();
                
                // Skip if inside string or comment
                if source.in_string_or_comment(line_number, pos + 1) {
                    continue;
                }
                
                // Skip unary operators
                if Self::is_likely_unary(line, pos, op) {
                    continue;
                }
                
                let op_len = op.len();
                
                // Check space before operator
                let has_space_before = if pos > 0 {
                    line.chars().nth(pos - 1).map(|c| c.is_whitespace()).unwrap_or(false)
                } else {
                    true // Beginning of line
                };
                
                // Check space after operator
                let has_space_after = if pos + op_len < line.len() {
                    line.chars().nth(pos + op_len).map(|c| c.is_whitespace()).unwrap_or(false)
                } else {
                    true // End of line
                };
                
                if !has_space_before || !has_space_after {
                    let msg = if !has_space_before && !has_space_after {
                        format!("Space missing around operator `{}`.", op)
                    } else if !has_space_before {
                        format!("Space missing before operator `{}`.", op)
                    } else {
                        format!("Space missing after operator `{}`.", op)
                    };
                    
                    offenses.push(Offense::new(
                        self.name(),
                        msg,
                        self.severity(),
                        Location::new(line_number, pos + 1, op_len),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 9. EmptyLineBetweenDefs
// ============================================================================

/// Detects missing empty line between method definitions.
pub struct EmptyLineBetweenDefs;

impl Cop for EmptyLineBetweenDefs {
    fn name(&self) -> &str {
        "Layout/EmptyLineBetweenDefs"
    }

    fn category(&self) -> Category {
        Category::Layout
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for missing empty line between method definitions"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        
        if source.is_empty() {
            return offenses;
        }

        let mut last_def_end_line: Option<usize> = None;
        let mut depth = 0;
        let mut in_method = false;
        
        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            let trimmed = line.trim();
            
            // Track def/end pairs
            if trimmed.starts_with("def ") {
                // Check if previous method ended without blank line
                if let Some(last_end) = last_def_end_line {
                    // Check if there's at least one blank line between
                    let has_blank = (last_end + 1..line_number).any(|ln| {
                        source.line(ln).map(|l| l.trim().is_empty()).unwrap_or(false)
                    });

                    if !has_blank {
                        offenses.push(Offense::new(
                            self.name(),
                            "Use empty lines between method definitions.",
                            self.severity(),
                            Location::new(line_number, 1, 0),
                        ));
                    }
                }
                
                in_method = true;
                depth = 1;
            } else if in_method {
                // Track nested blocks
                if trimmed.starts_with("def ") || trimmed.starts_with("class ") || 
                   trimmed.starts_with("module ") || trimmed.starts_with("if ") || 
                   trimmed.starts_with("unless ") || trimmed.starts_with("while ") || 
                   trimmed.starts_with("until ") || trimmed.starts_with("for ") || 
                   trimmed.starts_with("case ") || trimmed.starts_with("begin ") || 
                   trimmed.ends_with(" do") {
                    depth += 1;
                } else if trimmed == "end" || trimmed.starts_with("end ") {
                    depth -= 1;
                    if depth == 0 {
                        // End of method
                        last_def_end_line = Some(line_number);
                        in_method = false;
                    }
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 10. SpaceInsideParens
// ============================================================================

/// Detects spaces immediately inside parentheses.
pub struct SpaceInsideParens;

impl Cop for SpaceInsideParens {
    fn name(&self) -> &str {
        "Layout/SpaceInsideParens"
    }

    fn category(&self) -> Category {
        Category::Layout
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for spaces immediately inside parentheses"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            let chars: Vec<char> = line.chars().collect();
            
            for (i, &ch) in chars.iter().enumerate() {
                // Skip if inside string or comment
                if source.in_string_or_comment(line_number, i + 1) {
                    continue;
                }
                
                if ch == '(' {
                    // Check for space after opening paren
                    if i + 1 < chars.len() && chars[i + 1] == ' ' {
                        // Make sure it's not followed by another space or closing paren
                        if i + 2 < chars.len() && chars[i + 2] != ')' {
                            offenses.push(Offense::new(
                                self.name(),
                                "Space inside opening parenthesis.",
                                self.severity(),
                                Location::new(line_number, i + 2, 1),
                            ));
                        }
                    }
                } else if ch == ')' {
                    // Check for space before closing paren
                    if i > 0 && chars[i - 1] == ' ' {
                        // Make sure it's not preceded by opening paren
                        if i < 2 || chars[i - 2] != '(' {
                            offenses.push(Offense::new(
                                self.name(),
                                "Space inside closing parenthesis.",
                                self.severity(),
                                Location::new(line_number, i, 1),
                            ));
                        }
                    }
                }
            }
        }

        offenses
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn test_source(content: &str) -> SourceFile {
        SourceFile::from_string(PathBuf::from("test.rb"), content.to_string())
    }

    // ========================================================================
    // TrailingWhitespace tests
    // ========================================================================

    #[test]
    fn test_trailing_whitespace_no_offense() {
        let source = test_source("def foo\n  bar\nend\n");
        let cop = TrailingWhitespace;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_trailing_whitespace_with_spaces() {
        let source = test_source("def foo  \n  bar\nend\n");
        let cop = TrailingWhitespace;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert_eq!(offenses[0].location.line, 1);
    }

    #[test]
    fn test_trailing_whitespace_skip_blank_lines() {
        let source = test_source("def foo\n  \nend\n");
        let cop = TrailingWhitespace;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_trailing_whitespace_empty_file() {
        let source = test_source("");
        let cop = TrailingWhitespace;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_trailing_whitespace_multiple_lines() {
        let source = test_source("line1  \nline2\nline3 \n");
        let cop = TrailingWhitespace;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 2);
    }

    // ========================================================================
    // TrailingEmptyLines tests
    // ========================================================================

    #[test]
    fn test_trailing_empty_lines_correct() {
        let source = test_source("def foo\n  bar\nend\n");
        let cop = TrailingEmptyLines;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_trailing_empty_lines_missing_newline() {
        let source = test_source("def foo\n  bar\nend");
        let cop = TrailingEmptyLines;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert!(offenses[0].message.contains("should end with a newline"));
    }

    #[test]
    fn test_trailing_empty_lines_too_many() {
        let source = test_source("def foo\n  bar\nend\n\n\n");
        let cop = TrailingEmptyLines;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert!(offenses[0].message.contains("trailing blank lines"));
    }

    #[test]
    fn test_trailing_empty_lines_empty_file() {
        let source = test_source("");
        let cop = TrailingEmptyLines;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // ========================================================================
    // LeadingEmptyLines tests
    // ========================================================================

    #[test]
    fn test_leading_empty_lines_no_offense() {
        let source = test_source("def foo\n  bar\nend\n");
        let cop = LeadingEmptyLines;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_leading_empty_lines_with_leading() {
        let source = test_source("\n\ndef foo\n  bar\nend\n");
        let cop = LeadingEmptyLines;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert!(offenses[0].message.contains("2 leading empty lines"));
    }

    #[test]
    fn test_leading_empty_lines_empty_file() {
        let source = test_source("");
        let cop = LeadingEmptyLines;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_leading_empty_lines_single_leading() {
        let source = test_source("\ncode\n");
        let cop = LeadingEmptyLines;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    // ========================================================================
    // EndOfLine tests
    // ========================================================================

    #[test]
    fn test_end_of_line_lf_only() {
        let source = test_source("def foo\n  bar\nend\n");
        let cop = EndOfLine;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_end_of_line_crlf() {
        let source = SourceFile::from_string(
            PathBuf::from("test.rb"),
            "def foo\r\n  bar\r\nend\r\n".to_string(),
        );
        let cop = EndOfLine;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 3);
    }

    #[test]
    fn test_end_of_line_mixed() {
        let source = SourceFile::from_string(
            PathBuf::from("test.rb"),
            "line1\nline2\r\nline3\n".to_string(),
        );
        let cop = EndOfLine;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_end_of_line_empty() {
        let source = test_source("");
        let cop = EndOfLine;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // ========================================================================
    // IndentationStyle tests
    // ========================================================================

    #[test]
    fn test_indentation_style_spaces_only() {
        let source = test_source("def foo\n  bar\nend\n");
        let cop = IndentationStyle;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_indentation_style_with_tabs() {
        let source = test_source("def foo\n\tbar\nend\n");
        let cop = IndentationStyle;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert!(offenses[0].message.contains("spaces for indentation"));
    }

    #[test]
    fn test_indentation_style_tabs_not_leading() {
        let source = test_source("def foo\n  bar\tbaz\nend\n");
        let cop = IndentationStyle;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_indentation_style_empty() {
        let source = test_source("");
        let cop = IndentationStyle;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // ========================================================================
    // IndentationWidth tests
    // ========================================================================

    #[test]
    fn test_indentation_width_correct() {
        let source = test_source("def foo\n  bar\nend\n");
        let cop = IndentationWidth;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_indentation_width_incorrect() {
        let source = test_source("def foo\n    bar\nend\n");
        let cop = IndentationWidth;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert!(offenses[0].message.contains("expected 2 spaces, found 4"));
    }

    #[test]
    fn test_indentation_width_nested() {
        let source = test_source("class Foo\n  def bar\n    baz\n  end\nend\n");
        let cop = IndentationWidth;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_indentation_width_empty() {
        let source = test_source("");
        let cop = IndentationWidth;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_indentation_width_if_block() {
        let source = test_source("if condition\n  do_something\nend\n");
        let cop = IndentationWidth;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // ========================================================================
    // SpaceAfterComma tests
    // ========================================================================

    #[test]
    fn test_space_after_comma_correct() {
        let source = test_source("foo(a, b, c)\n");
        let cop = SpaceAfterComma;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_space_after_comma_missing() {
        let source = test_source("foo(a,b,c)\n");
        let cop = SpaceAfterComma;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 2);
    }

    #[test]
    fn test_space_after_comma_in_string() {
        let source = test_source("x = \"a,b,c\"\n");
        let cop = SpaceAfterComma;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_space_after_comma_in_comment() {
        let source = test_source("# a,b,c\n");
        let cop = SpaceAfterComma;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_space_after_comma_end_of_line() {
        let source = test_source("foo(a,\n  b)\n");
        let cop = SpaceAfterComma;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // ========================================================================
    // SpaceAroundOperators tests
    // ========================================================================

    #[test]
    fn test_space_around_operators_correct() {
        let source = test_source("x = 1 + 2\n");
        let cop = SpaceAroundOperators;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_space_around_operators_missing() {
        let source = test_source("x=1+2\n");
        let cop = SpaceAroundOperators;
        let offenses = cop.check(&source);
        assert!(offenses.len() >= 2); // At least = and +
    }

    #[test]
    fn test_space_around_operators_unary() {
        let source = test_source("x = -1\n");
        let cop = SpaceAroundOperators;
        let offenses = cop.check(&source);
        // Should not report the unary minus
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_space_around_operators_in_string() {
        let source = test_source("x = \"1+2\"\n");
        let cop = SpaceAroundOperators;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_space_around_operators_comparison() {
        let source = test_source("if x==5\n  foo\nend\n");
        let cop = SpaceAroundOperators;
        let offenses = cop.check(&source);
        assert!(!offenses.is_empty());
    }

    // ========================================================================
    // EmptyLineBetweenDefs tests
    // ========================================================================

    #[test]
    fn test_empty_line_between_defs_correct() {
        let source = test_source("def foo\n  bar\nend\n\ndef baz\n  qux\nend\n");
        let cop = EmptyLineBetweenDefs;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_empty_line_between_defs_missing() {
        let source = test_source("def foo\n  bar\nend\ndef baz\n  qux\nend\n");
        let cop = EmptyLineBetweenDefs;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_empty_line_between_defs_single_def() {
        let source = test_source("def foo\n  bar\nend\n");
        let cop = EmptyLineBetweenDefs;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_empty_line_between_defs_nested() {
        let source = test_source("class Foo\n  def bar\n    baz\n  end\n\n  def qux\n    quux\n  end\nend\n");
        let cop = EmptyLineBetweenDefs;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_empty_line_between_defs_empty() {
        let source = test_source("");
        let cop = EmptyLineBetweenDefs;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // ========================================================================
    // SpaceInsideParens tests
    // ========================================================================

    #[test]
    fn test_space_inside_parens_no_offense() {
        let source = test_source("foo(bar)\n");
        let cop = SpaceInsideParens;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_space_inside_parens_after_opening() {
        let source = test_source("foo( bar)\n");
        let cop = SpaceInsideParens;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert!(offenses[0].message.contains("opening parenthesis"));
    }

    #[test]
    fn test_space_inside_parens_before_closing() {
        let source = test_source("foo(bar )\n");
        let cop = SpaceInsideParens;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert!(offenses[0].message.contains("closing parenthesis"));
    }

    #[test]
    fn test_space_inside_parens_both() {
        let source = test_source("foo( bar )\n");
        let cop = SpaceInsideParens;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 2);
    }

    #[test]
    fn test_space_inside_parens_empty_parens() {
        let source = test_source("foo( )\n");
        let cop = SpaceInsideParens;
        let offenses = cop.check(&source);
        // Empty parens with space should not trigger (edge case)
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_space_inside_parens_in_string() {
        let source = test_source("x = \"foo( bar )\"\n");
        let cop = SpaceInsideParens;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }
}
