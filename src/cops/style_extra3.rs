//! Additional Style cops (set 3) for Ruby code formatting and conventions.

use regex::Regex;

use crate::cop::{Category, Cop, Severity};
use crate::offense::{Location, Offense};
use crate::source::SourceFile;

// ============================================================================
// 1. OptionHash - use keyword args not options hash
// ============================================================================

pub struct OptionHash;

impl Cop for OptionHash {
    fn name(&self) -> &str {
        "Style/OptionHash"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Prefer keyword arguments over options hash"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let options_hash_regex = Regex::new(r#"def\s+\w+\([^)]*options\s*=\s*\{\}"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in options_hash_regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use keyword arguments instead of options hash",
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 2. OptionalArguments - optional args before required
// ============================================================================

pub struct OptionalArguments;

impl Cop for OptionalArguments {
    fn name(&self) -> &str {
        "Style/OptionalArguments"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Optional arguments should not appear before required arguments"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        // Pattern: def method(optional = value, required)
        let regex = Regex::new(r#"def\s+\w+\([^)]*=\s*[^,)]+,\s*\w+\s*[,)]"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Optional arguments should appear after required arguments",
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 3. ParallelAssignment - don't use 'a, b = 1, 2'
// ============================================================================

pub struct ParallelAssignment;

impl Cop for ParallelAssignment {
    fn name(&self) -> &str {
        "Style/ParallelAssignment"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Avoid parallel assignment"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"^\s*(\w+\s*,\s*\w+.*?)\s*=\s*(.+,)"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            if let Some(capture) = regex.captures(line) {
                if let Some(matched) = capture.get(0) {
                    let col = matched.start() + 1;
                    if !source.in_string_or_comment(line_number, col) {
                        offenses.push(Offense::new(
                            self.name(),
                            "Avoid parallel assignment",
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

// ============================================================================
// 4. PreferredHashMethods - use 'key?' not 'has_key?'
// ============================================================================

pub struct PreferredHashMethods;

impl Cop for PreferredHashMethods {
    fn name(&self) -> &str {
        "Style/PreferredHashMethods"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use preferred hash methods"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let has_key_regex = Regex::new(r#"\bhas_key\?"#).unwrap();
        let has_value_regex = Regex::new(r#"\bhas_value\?"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for capture in has_key_regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        r#"Use 'key?' instead of 'has_key?'"#,
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }

            for capture in has_value_regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        r#"Use 'value?' instead of 'has_value?'"#,
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 5. QuotedSymbols - consistent symbol quoting
// ============================================================================

pub struct QuotedSymbols;

impl Cop for QuotedSymbols {
    fn name(&self) -> &str {
        "Style/QuotedSymbols"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Prefer unquoted symbols when possible"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        // Match both single and double quoted symbols with simple identifiers
        let single_quoted = Regex::new(r#":'(\w+)'"#).unwrap();
        let double_quoted = Regex::new(r#":"(\w+)""#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;

            // Check single-quoted symbols
            for capture in single_quoted.captures_iter(line) {
                if let Some(matched) = capture.get(0) {
                    let col = matched.start() + 1;
                    if !source.in_string_or_comment(line_number, col) {
                        offenses.push(Offense::new(
                            self.name(),
                            "Use unquoted symbols when possible",
                            self.severity(),
                            Location::new(line_number, col, matched.len()),
                        ));
                    }
                }
            }

            // Check double-quoted symbols
            for capture in double_quoted.captures_iter(line) {
                if let Some(matched) = capture.get(0) {
                    let col = matched.start() + 1;
                    if !source.in_string_or_comment(line_number, col) {
                        offenses.push(Offense::new(
                            self.name(),
                            "Use unquoted symbols when possible",
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

// ============================================================================
// 6. RaiseArgs - consistent raise argument style
// ============================================================================

pub struct RaiseArgs;

impl Cop for RaiseArgs {
    fn name(&self) -> &str {
        "Style/RaiseArgs"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Prefer compact raise arguments"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let raise_regex = Regex::new(r#"\braise\s+\w+\.new\("#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in raise_regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        r#"Prefer 'raise Exception, 'message'' over 'raise Exception.new('message')'"#,
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 7. RandomWithOffset - use 'rand(n)' not 'rand(n) + 1'
// ============================================================================

pub struct RandomWithOffset;

impl Cop for RandomWithOffset {
    fn name(&self) -> &str {
        "Style/RandomWithOffset"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Prefer rand(N..M) over rand(n) + offset"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"\brand\(\d+\)\s*[+\-]\s*\d+"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        r#"Prefer 'rand(N..M)' over 'rand(n) + offset'"#,
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 8. RedundantArgument - redundant argument to method
// ============================================================================

pub struct RedundantArgument;

impl Cop for RedundantArgument {
    fn name(&self) -> &str {
        "Style/RedundantArgument"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Redundant argument passed to method"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"\.split\(['"]\\s\+['"]\)"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Redundant argument to split",
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 9. RedundantArrayConstructor - 'Array.new' with literal
// ============================================================================

pub struct RedundantArrayConstructor;

impl Cop for RedundantArrayConstructor {
    fn name(&self) -> &str {
        "Style/RedundantArrayConstructor"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use array literal instead of Array.new"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"\bArray\.new\(\[\]|\[\]\)"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        r#"Use '[]' instead of 'Array.new([])'"#,
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 10. RedundantArrayFlatten - redundant '.flatten' call
// ============================================================================

pub struct RedundantArrayFlatten;

impl Cop for RedundantArrayFlatten {
    fn name(&self) -> &str {
        "Style/RedundantArrayFlatten"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Redundant flatten call"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"\.flatten\(1\)\.flatten"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Redundant flatten call",
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 11. RedundantAssignment - assignment then immediate return
// ============================================================================

pub struct RedundantAssignment;

impl Cop for RedundantAssignment {
    fn name(&self) -> &str {
        "Style/RedundantAssignment"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Assignment followed by immediate return"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        for line_num in 1..source.line_count() {
            if let (Some(line1), Some(line2)) = (source.line(line_num), source.line(line_num + 1)) {
                let trimmed1 = line1.trim();
                let trimmed2 = line2.trim();
                
                // Check for pattern: var = expr; var
                if let Some(var_name) = trimmed1.strip_suffix(|c: char| c.is_whitespace() || c == '=').and_then(|s| {
                    if s.contains('=') {
                        s.split('=').next().map(|v| v.trim())
                    } else {
                        None
                    }
                }) {
                    if trimmed2 == var_name && !var_name.is_empty() {
                        if let Some(pos) = line1.find(var_name) {
                            offenses.push(Offense::new(
                                self.name(),
                                "Redundant assignment before return",
                                self.severity(),
                                Location::new(line_num, pos + 1, var_name.len()),
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
// 12. RedundantCapitalW - use '%w' not '%W' without interpolation
// ============================================================================

pub struct RedundantCapitalW;

impl Cop for RedundantCapitalW {
    fn name(&self) -> &str {
        "Style/RedundantCapitalW"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use %w instead of %W when no interpolation is needed"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"%W[(\[\{]"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            if !line.contains("#{") {
                for capture in regex.find_iter(line) {
                    let col = capture.start() + 1;
                    if !source.in_string_or_comment(line_number, col) {
                        offenses.push(Offense::new(
                            self.name(),
                            r#"Use '%w' instead of '%W' when interpolation is not needed"#,
                            self.severity(),
                            Location::new(line_number, col, capture.len()),
                        ));
                    }
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 13. RedundantCondition - 'x ? x : y' -> 'x || y'
// ============================================================================

pub struct RedundantCondition;

impl Cop for RedundantCondition {
    fn name(&self) -> &str {
        "Style/RedundantCondition"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use || instead of ternary with same condition and branch"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"(\w+)\s*\?\s*(\w+)\s*:"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.captures_iter(line) {
                let condition = capture.get(1).unwrap().as_str();
                let true_branch = capture.get(2).unwrap().as_str();

                // Only flag if condition and true branch are identical
                if condition != true_branch {
                    continue;
                }

                if let Some(matched) = capture.get(0) {
                    let col = matched.start() + 1;
                    if !source.in_string_or_comment(line_number, col) {
                        offenses.push(Offense::new(
                            self.name(),
                            r#"Use '||' instead of 'x ? x : y'"#,
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

// ============================================================================
// 14. RedundantConditional - 'if x then true else false end'
// ============================================================================

pub struct RedundantConditional;

impl Cop for RedundantConditional {
    fn name(&self) -> &str {
        "Style/RedundantConditional"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Redundant conditional returning boolean"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            let trimmed = line.trim();
            
            if trimmed.starts_with("if ") && line_number < source.line_count() {
                if let Some(next_line) = source.line(line_number + 1) {
                    let next_trimmed = next_line.trim();
                    if next_trimmed == "true" {
                        if let Some(else_line) = source.line(line_number + 2) {
                            if else_line.trim() == "else" {
                                if let Some(false_line) = source.line(line_number + 3) {
                                    if false_line.trim() == "false" {
                                        if let Some(pos) = line.find("if ") {
                                            offenses.push(Offense::new(
                                                self.name(),
                                                "Redundant conditional returning boolean value",
                                                self.severity(),
                                                Location::new(line_number, pos + 1, 3),
                                            ));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 15. RedundantConstantBase - redundant '::' prefix
// ============================================================================

pub struct RedundantConstantBase;

impl Cop for RedundantConstantBase {
    fn name(&self) -> &str {
        "Style/RedundantConstantBase"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Redundant :: prefix for top-level constant"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"::(String|Array|Hash|Integer|Float|Symbol|Object|Module|Class)\b"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        r#"Redundant '::' prefix for top-level constant"#,
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 16. RedundantCurrentDirectoryInPath - './' in require
// ============================================================================

pub struct RedundantCurrentDirectoryInPath;

impl Cop for RedundantCurrentDirectoryInPath {
    fn name(&self) -> &str {
        "Style/RedundantCurrentDirectoryInPath"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Redundant ./ in require path"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"require(_relative)?\s+['"]\./"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                offenses.push(Offense::new(
                    self.name(),
                    r#"Remove redundant './' from require path"#,
                    self.severity(),
                    Location::new(line_number, col, capture.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// 17. RedundantDoubleSplatHashBraces - '**{}' is redundant
// ============================================================================

pub struct RedundantDoubleSplatHashBraces;

impl Cop for RedundantDoubleSplatHashBraces {
    fn name(&self) -> &str {
        "Style/RedundantDoubleSplatHashBraces"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Redundant double splat with hash braces"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"\*\*\{\s*\w+:\s*\w+\s*\}"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        r#"Pass hash key-value pairs directly instead of using '**{}'"#,
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 18. RedundantEach - redundant '.each' in chain
// ============================================================================

pub struct RedundantEach;

impl Cop for RedundantEach {
    fn name(&self) -> &str {
        "Style/RedundantEach"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Redundant .each in method chain"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"\.each\.(map|select|reject|each_with_object)"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        r#"Remove redundant '.each' from method chain"#,
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 19. RedundantFetchBlock - 'fetch(:key) { default }' vs 'fetch(:key, default)'
// ============================================================================

pub struct RedundantFetchBlock;

impl Cop for RedundantFetchBlock {
    fn name(&self) -> &str {
        "Style/RedundantFetchBlock"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use fetch with default value instead of block when simple"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"\.fetch\([^)]+\)\s*\{\s*['""]?\w+['""]?\s*\}"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        r#"Use 'fetch(key, default)' instead of 'fetch(key) { default }'"#,
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 20. RedundantFileExtensionInRequire - '.rb' in require
// ============================================================================

pub struct RedundantFileExtensionInRequire;

impl Cop for RedundantFileExtensionInRequire {
    fn name(&self) -> &str {
        "Style/RedundantFileExtensionInRequire"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Remove .rb extension from require"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"require(_relative)?\s+['"][^'"]+\.rb['"]"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                offenses.push(Offense::new(
                    self.name(),
                    r#"Remove '.rb' extension from require"#,
                    self.severity(),
                    Location::new(line_number, col, capture.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// 21. RedundantFilterChain - '.select.first' -> '.detect'
// ============================================================================

pub struct RedundantFilterChain;

impl Cop for RedundantFilterChain {
    fn name(&self) -> &str {
        "Style/RedundantFilterChain"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use detect/find instead of select.first"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"\.select\s*\{[^}]+\}\s*\.(first|last|\[\d+\])"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        r#"Use 'detect'/'find' instead of 'select.first'"#,
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 22. RedundantFormat - redundant format/sprintf
// ============================================================================

pub struct RedundantFormat;

impl Cop for RedundantFormat {
    fn name(&self) -> &str {
        "Style/RedundantFormat"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Avoid redundant format/sprintf"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"(format|sprintf)\(['""]%s['"]\s*,"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Redundant use of format/sprintf with only %s",
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 23. RedundantHeredocDelimiterQuotes - quoted heredoc delimiter
// ============================================================================

pub struct RedundantHeredocDelimiterQuotes;

impl Cop for RedundantHeredocDelimiterQuotes {
    fn name(&self) -> &str {
        "Style/RedundantHeredocDelimiterQuotes"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Remove quotes from heredoc delimiter when not needed"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        // Match single-quoted heredocs
        let single_quote_regex = Regex::new(r#"<<[-~]?'(\w+)'"#).unwrap();
        // Match double-quoted heredocs
        let double_quote_regex = Regex::new(r#"<<[-~]?"(\w+)""#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            if !line.contains("#{") {
                // Check single-quoted heredocs
                for capture in single_quote_regex.captures_iter(line) {
                    if let Some(matched) = capture.get(0) {
                        let col = matched.start() + 1;
                        if !source.in_string_or_comment(line_number, col) {
                            offenses.push(Offense::new(
                                self.name(),
                                "Remove quotes from heredoc delimiter when interpolation is not used",
                                self.severity(),
                                Location::new(line_number, col, matched.len()),
                            ));
                        }
                    }
                }

                // Check double-quoted heredocs
                for capture in double_quote_regex.captures_iter(line) {
                    if let Some(matched) = capture.get(0) {
                        let col = matched.start() + 1;
                        if !source.in_string_or_comment(line_number, col) {
                            offenses.push(Offense::new(
                                self.name(),
                                "Remove quotes from heredoc delimiter when interpolation is not used",
                                self.severity(),
                                Location::new(line_number, col, matched.len()),
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
// 24. RedundantInitialize - empty initialize method
// ============================================================================

pub struct RedundantInitialize;

impl Cop for RedundantInitialize {
    fn name(&self) -> &str {
        "Style/RedundantInitialize"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Empty initialize method is redundant"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let mut in_initialize = false;
        let mut initialize_line = 0;

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            let trimmed = line.trim();

            if trimmed.starts_with("def initialize") {
                in_initialize = true;
                initialize_line = line_number;
            } else if in_initialize && trimmed == "end" {
                // Check if there was any content
                let mut has_content = false;
                for i in (initialize_line + 1)..line_number {
                    if let Some(mid_line) = source.line(i) {
                        if !mid_line.trim().is_empty() && !mid_line.trim().starts_with("#") {
                            has_content = true;
                            break;
                        }
                    }
                }
                
                if !has_content {
                    if let Some(init_line) = source.line(initialize_line) {
                        if let Some(pos) = init_line.find("def initialize") {
                            offenses.push(Offense::new(
                                self.name(),
                                "Empty initialize method is redundant",
                                self.severity(),
                                Location::new(initialize_line, pos + 1, 14),
                            ));
                        }
                    }
                }
                in_initialize = false;
            }
        }

        offenses
    }
}

// ============================================================================
// 25. RedundantInterpolationUnfreeze - redundant unfreeze after interpolation
// ============================================================================

pub struct RedundantInterpolationUnfreeze;

impl Cop for RedundantInterpolationUnfreeze {
    fn name(&self) -> &str {
        "Style/RedundantInterpolationUnfreeze"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Interpolated strings are already unfrozen"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#""[^"]*#\{[^}]+\}[^"]*"\.dup"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                offenses.push(Offense::new(
                    self.name(),
                    "Interpolated strings are already mutable",
                    self.severity(),
                    Location::new(line_number, col, capture.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// 26. RedundantLineContinuation - unnecessary '\' continuation
// ============================================================================

pub struct RedundantLineContinuation;

impl Cop for RedundantLineContinuation {
    fn name(&self) -> &str {
        "Style/RedundantLineContinuation"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Redundant line continuation"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            if line.trim_end().ends_with('\\') {
                // Check if next line starts with operator or method call
                if let Some(next_line) = source.line(line_number + 1) {
                    let next_trimmed = next_line.trim_start();
                    if next_trimmed.starts_with('.') || next_trimmed.starts_with("||") || next_trimmed.starts_with("&&") {
                        if let Some(pos) = line.rfind('\\') {
                            offenses.push(Offense::new(
                                self.name(),
                                "Redundant line continuation",
                                self.severity(),
                                Location::new(line_number, pos + 1, 1),
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
// 27. RedundantRegexpArgument - redundant regexp in gsub
// ============================================================================

pub struct RedundantRegexpArgument;

impl Cop for RedundantRegexpArgument {
    fn name(&self) -> &str {
        "Style/RedundantRegexpArgument"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use string argument instead of regexp when possible"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"\.(gsub|sub)\(/\w+/\)"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use string argument instead of simple regexp",
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 28. RedundantRegexpCharacterClass - '[a]' -> 'a' in regexp
// ============================================================================

pub struct RedundantRegexpCharacterClass;

impl Cop for RedundantRegexpCharacterClass {
    fn name(&self) -> &str {
        "Style/RedundantRegexpCharacterClass"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Redundant character class in regexp"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"/\[(\w)\]/"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.captures_iter(line) {
                if let Some(matched) = capture.get(0) {
                    let col = matched.start() + 1;
                    if !source.in_string_or_comment(line_number, col) {
                        offenses.push(Offense::new(
                            self.name(),
                            "Use literal character instead of single-character class",
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

// ============================================================================
// 29. RedundantRegexpConstructor - 'Regexp.new' with literal
// ============================================================================

pub struct RedundantRegexpConstructor;

impl Cop for RedundantRegexpConstructor {
    fn name(&self) -> &str {
        "Style/RedundantRegexpConstructor"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use regexp literal instead of Regexp.new"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"Regexp\.new\(['\"]"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        r#"Use regexp literal '/pattern/' instead of 'Regexp.new'"#,
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 30. RedundantRegexpEscape - unnecessary escape in regexp
// ============================================================================

pub struct RedundantRegexpEscape;

impl Cop for RedundantRegexpEscape {
    fn name(&self) -> &str {
        "Style/RedundantRegexpEscape"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Unnecessary escape in regexp"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"/[^\\]*\\[a-zA-Z0-9]/"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let text = capture.as_str();
                // Check for unnecessary escapes like backslash-a, backslash-z (not special)
                if text.contains(r#"\a"#) || text.contains(r#"\z"#) || text.contains(r#"\m"#) {
                    let col = capture.start() + 1;
                    if !source.in_string_or_comment(line_number, col) {
                        offenses.push(Offense::new(
                            self.name(),
                            "Unnecessary escape in regexp",
                            self.severity(),
                            Location::new(line_number, col, capture.len()),
                        ));
                    }
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 31. RedundantSelfAssignment - 'self.x = self.x'
// ============================================================================

pub struct RedundantSelfAssignment;

impl Cop for RedundantSelfAssignment {
    fn name(&self) -> &str {
        "Style/RedundantSelfAssignment"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Redundant self assignment"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"(self\.\w+)\s*=\s*(self\.\w+)\b"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.captures_iter(line) {
                let left = capture.get(1).unwrap().as_str();
                let right = capture.get(2).unwrap().as_str();

                // Only flag if both sides are identical
                if left != right {
                    continue;
                }

                if let Some(matched) = capture.get(0) {
                    let col = matched.start() + 1;
                    if !source.in_string_or_comment(line_number, col) {
                        offenses.push(Offense::new(
                            self.name(),
                            "Redundant self assignment",
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

// ============================================================================
// 32. RedundantSelfAssignmentBranch - assignment in only one branch
// ============================================================================

pub struct RedundantSelfAssignmentBranch;

impl Cop for RedundantSelfAssignmentBranch {
    fn name(&self) -> &str {
        "Style/RedundantSelfAssignmentBranch"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Redundant self assignment in conditional branch"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            let trimmed = line.trim();
            
            if trimmed.starts_with("if ") {
                // Look ahead for pattern: if cond; x = x; end
                if let Some(next_line) = source.line(line_number + 1) {
                    let next_trimmed = next_line.trim();
                    if let Some(var_pos) = next_trimmed.find(" = ") {
                        let var_name = next_trimmed[..var_pos].trim();
                        let value = next_trimmed[var_pos + 3..].trim();
                        if var_name == value {
                            if let Some(pos) = next_line.find(var_name) {
                                offenses.push(Offense::new(
                                    self.name(),
                                    "Redundant self assignment in conditional",
                                    self.severity(),
                                    Location::new(line_number + 1, pos + 1, var_name.len()),
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

// ============================================================================
// 33. RedundantSort - redundant sort before first/last
// ============================================================================

pub struct RedundantSort;

impl Cop for RedundantSort {
    fn name(&self) -> &str {
        "Style/RedundantSort"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use min/max instead of sort.first/last"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"\.sort\.(first|last)"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        r#"Use 'min'/'max' instead of 'sort.first'/'sort.last'"#,
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 34. RedundantSortBy - '.sort_by { |x| x }' -> '.sort'
// ============================================================================

pub struct RedundantSortBy;

impl Cop for RedundantSortBy {
    fn name(&self) -> &str {
        "Style/RedundantSortBy"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use sort instead of sort_by with identity block"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"\.sort_by\s*\{\s*\|(\w+)\|\s*(\w+)\s*\}"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.captures_iter(line) {
                let param = capture.get(1).unwrap().as_str();
                let body = capture.get(2).unwrap().as_str();

                // Only flag if parameter and body are identical (identity block)
                if param != body {
                    continue;
                }

                if let Some(matched) = capture.get(0) {
                    let col = matched.start() + 1;
                    if !source.in_string_or_comment(line_number, col) {
                        offenses.push(Offense::new(
                            self.name(),
                            r#"Use 'sort' instead of 'sort_by { |x| x }'"#,
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

// ============================================================================
// 35. RedundantStringEscape - unnecessary string escape
// ============================================================================

pub struct RedundantStringEscape;

impl Cop for RedundantStringEscape {
    fn name(&self) -> &str {
        "Style/RedundantStringEscape"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Unnecessary escape in string"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"'[^']*\\[a-z][^']*'"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let text = capture.as_str();
                // In single-quoted strings, only \' and \\ are valid escapes
                if text.contains(r#"\n"#) || text.contains(r#"\t"#) {
                    let col = capture.start() + 1;
                    offenses.push(Offense::new(
                        self.name(),
                        "Unnecessary escape in single-quoted string",
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 36. RequireOrder - require statements ordering
// ============================================================================

pub struct RequireOrder;

impl Cop for RequireOrder {
    fn name(&self) -> &str {
        "Style/RequireOrder"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Require statements should be ordered"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let mut last_require: Option<(usize, String)> = None;

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            let trimmed = line.trim();
            
            if trimmed.starts_with("require ") || trimmed.starts_with("require_relative ") {
                if let Some((_last_line, last_req)) = &last_require {
                    if trimmed < last_req.as_str() {
                        offenses.push(Offense::new(
                            self.name(),
                            "Require statements should be sorted alphabetically",
                            self.severity(),
                            Location::new(line_number, 1, line.len()),
                        ));
                    }
                }
                last_require = Some((line_number, trimmed.to_string()));
            } else if !trimmed.is_empty() && !trimmed.starts_with('#') {
                // Reset when we encounter non-require code
                last_require = None;
            }
        }

        offenses
    }
}

// ============================================================================
// 37. ReturnNil - explicit 'return nil' instead of bare 'return'
// ============================================================================

pub struct ReturnNil;

impl Cop for ReturnNil {
    fn name(&self) -> &str {
        "Style/ReturnNil"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        r#"Use 'return' instead of 'return nil'"#
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"\breturn\s+nil\b"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        r#"Use 'return' instead of 'return nil'"#,
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 38. ReturnNilInPredicateMethodDefinition - return nil in predicate
// ============================================================================

pub struct ReturnNilInPredicateMethodDefinition;

impl Cop for ReturnNilInPredicateMethodDefinition {
    fn name(&self) -> &str {
        "Style/ReturnNilInPredicateMethodDefinition"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Predicate methods should not return nil"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let def_regex = Regex::new(r#"def\s+(\w+\?)"#).unwrap();
        let return_nil_regex = Regex::new(r#"\breturn\s+nil\b"#).unwrap();

        let mut in_predicate = false;
        let mut _predicate_start = 0;

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            let trimmed = line.trim();

            if let Some(capture) = def_regex.captures(trimmed) {
                in_predicate = true;
                _predicate_start = line_number;
                if let Some(_method_name) = capture.get(1) {
                    // Predicatemethod found
                }
            }

            if in_predicate {
                if let Some(capture) = return_nil_regex.find(trimmed) {
                    let col = line.find("return").unwrap_or(0) + 1;
                    offenses.push(Offense::new(
                        self.name(),
                        "Predicate methods should return a boolean, not nil",
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }

                if trimmed == "end" {
                    in_predicate = false;
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 39. ReverseFind - '.reverse.find' -> '.reverse_each.find' or similar
// ============================================================================

pub struct ReverseFind;

impl Cop for ReverseFind {
    fn name(&self) -> &str {
        "Style/ReverseFind"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use reverse_each instead of reverse.find"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"\.reverse\.(find|detect)"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        r#"Use 'reverse_each.find' instead of 'reverse.find'"#,
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 40. Sample - use '.sample' not '.shuffle.first'
// ============================================================================

pub struct Sample;

impl Cop for Sample {
    fn name(&self) -> &str {
        "Style/Sample"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use sample instead of shuffle.first"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"\.shuffle\.(first|last|\[\d+\])"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        r#"Use 'sample' instead of 'shuffle.first'"#,
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// Continuing with remaining cops...

// ============================================================================
// 41. SelectByRegexp - use '.grep' not '.select { |x| x =~ }'
// ============================================================================

pub struct SelectByRegexp;

impl Cop for SelectByRegexp {
    fn name(&self) -> &str {
        "Style/SelectByRegexp"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use grep instead of select with regexp"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"\.select\s*\{\s*\|\w+\|\s*\w+\s*=~"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        r#"Use 'grep' instead of 'select' with regexp"#,
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 42. SendWithLiteralMethodName - 'send(:foo)' -> '.foo'
// ============================================================================

pub struct SendWithLiteralMethodName;

impl Cop for SendWithLiteralMethodName {
    fn name(&self) -> &str {
        "Style/SendWithLiteralMethodName"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use direct method call instead of send with literal"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"\.send\(:[a-zA-Z_]\w*\)"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        r#"Use direct method call instead of 'send' with symbol literal"#,
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 43. SingleArgumentDig - '.dig(:key)' -> '[:key]'
// ============================================================================

pub struct SingleArgumentDig;

impl Cop for SingleArgumentDig {
    fn name(&self) -> &str {
        "Style/SingleArgumentDig"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use [] instead of dig with single argument"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"\.dig\(:[a-zA-Z_]\w*\)"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        r#"Use '[]' instead of 'dig' with single argument"#,
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 44. SingleLineBlockParams - named block params for single-line
// ============================================================================

pub struct SingleLineBlockParams;

impl Cop for SingleLineBlockParams {
    fn name(&self) -> &str {
        "Style/SingleLineBlockParams"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use descriptive block parameter names"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"\{\s*\|([a-z])\|"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.captures_iter(line) {
                if let Some(param) = capture.get(1) {
                    if param.as_str().len() == 1 {
                        let col = param.start() + 1;
                        if !source.in_string_or_comment(line_number, col) {
                            offenses.push(Offense::new(
                                self.name(),
                                "Use descriptive block parameter names instead of single letter",
                                self.severity(),
                                Location::new(line_number, col, param.len()),
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
// 45. SingleLineDoEndBlock - don't use 'do/end' for single-line
// ============================================================================

pub struct SingleLineDoEndBlock;

impl Cop for SingleLineDoEndBlock {
    fn name(&self) -> &str {
        "Style/SingleLineDoEndBlock"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use braces for single-line blocks"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"\bdo\s+\|[^|]+\|[^|]+end\b"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        r#"Use '{...}' for single-line blocks instead of 'do...end'"#,
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 46. SlicingWithRange - use 'take'/'drop' not slice
// ============================================================================

pub struct SlicingWithRange;

impl Cop for SlicingWithRange {
    fn name(&self) -> &str {
        "Style/SlicingWithRange"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use first/last/take/drop instead of slice with range"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"\[0\.\.(\d+|\.)\]"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        r#"Use 'first' or 'take' instead of slice with range from 0"#,
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 47. SoleNestedConditional - combine nested if
// ============================================================================

pub struct SoleNestedConditional;

impl Cop for SoleNestedConditional {
    fn name(&self) -> &str {
        "Style/SoleNestedConditional"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Combine nested conditionals"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        for line_num in 1..source.line_count() {
            if let (Some(line1), Some(line2)) = (source.line(line_num), source.line(line_num + 1)) {
                let trim1 = line1.trim();
                let trim2 = line2.trim();
                
                if trim1.starts_with("if ") && trim2.starts_with("if ") {
                    if let Some(pos) = line2.find("if ") {
                        offenses.push(Offense::new(
                            self.name(),
                            r#"Combine nested conditionals with '&&'"#,
                            self.severity(),
                            Location::new(line_num + 1, pos + 1, 3),
                        ));
                    }
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 48. StabbyLambdaParentheses - parens with stabby lambda
// ============================================================================

pub struct StabbyLambdaParentheses;

impl Cop for StabbyLambdaParentheses {
    fn name(&self) -> &str {
        "Style/StabbyLambdaParentheses"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Require parentheses for stabby lambda arguments"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"->\s*\w+\s+"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use parentheses for stabby lambda arguments",
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 49. StaticClass - use module not class with only class methods
// ============================================================================

pub struct StaticClass;

impl Cop for StaticClass {
    fn name(&self) -> &str {
        "Style/StaticClass"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use module for classes with only class methods"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let mut in_class = false;
        let mut class_line = 0;
        let mut has_instance_method = false;

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            let trimmed = line.trim();

            if trimmed.starts_with("class ") {
                in_class = true;
                class_line = line_number;
                has_instance_method = false;
            } else if in_class {
                if trimmed.starts_with("def ") && !trimmed.starts_with("def self.") {
                    has_instance_method = true;
                }
                if trimmed == "end" {
                    if !has_instance_method {
                        if let Some(class_line_str) = source.line(class_line) {
                            if let Some(pos) = class_line_str.find("class ") {
                                offenses.push(Offense::new(
                                    self.name(),
                                    "Use module for classes with only class methods",
                                    self.severity(),
                                    Location::new(class_line, pos + 1, 5),
                                ));
                            }
                        }
                    }
                    in_class = false;
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 50. StderrPuts - use 'warn' not '$stderr.puts'
// ============================================================================

pub struct StderrPuts;

impl Cop for StderrPuts {
    fn name(&self) -> &str {
        "Style/StderrPuts"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use warn instead of $stderr.puts"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"\$stderr\.puts"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        r#"Use 'warn' instead of '$stderr.puts'"#,
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 51. StringChars - use '.chars' not '.split('')'
// ============================================================================

pub struct StringChars;

impl Cop for StringChars {
    fn name(&self) -> &str {
        "Style/StringChars"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use chars instead of split('')"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"\.split\(['"]{2}\)"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        r#"Use 'chars' instead of 'split('')'"#,
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 52. StringHashKeys - use symbol keys not string keys
// ============================================================================

pub struct StringHashKeys;

impl Cop for StringHashKeys {
    fn name(&self) -> &str {
        "Style/StringHashKeys"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use symbol keys instead of string keys in hashes"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"['"](\w+)['"]\s*=>"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.captures_iter(line) {
                if let Some(matched) = capture.get(0) {
                    let col = matched.start() + 1;
                    offenses.push(Offense::new(
                        self.name(),
                        "Use symbol keys instead of string keys",
                        self.severity(),
                        Location::new(line_number, col, matched.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 53. StringLiteralsInInterpolation - quote style in interpolation
// ============================================================================

pub struct StringLiteralsInInterpolation;

impl Cop for StringLiteralsInInterpolation {
    fn name(&self) -> &str {
        "Style/StringLiteralsInInterpolation"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use consistent quote style in interpolation"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"#\{[^}]*"[^"]*"[^}]*\}"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                offenses.push(Offense::new(
                    self.name(),
                    "Prefer single quotes in interpolation",
                    self.severity(),
                    Location::new(line_number, col, capture.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// 54. StringMethods - use 'tr' not 'gsub' for single chars
// ============================================================================

pub struct StringMethods;

impl Cop for StringMethods {
    fn name(&self) -> &str {
        "Style/StringMethods"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use tr instead of gsub for single character replacement"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"\.gsub\(['"]\w['"],\s*['"]\w['"]\)"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        r#"Use 'tr' instead of 'gsub' for single character replacement"#,
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 55. Strip - use '.strip' not '.lstrip.rstrip'
// ============================================================================

pub struct Strip;

impl Cop for Strip {
    fn name(&self) -> &str {
        "Style/Strip"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use strip instead of lstrip.rstrip"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"\.lstrip\.rstrip|\.rstrip\.lstrip"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        r#"Use 'strip' instead of 'lstrip.rstrip'"#,
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 56. SuperArguments - redundant args to super
// ============================================================================

pub struct SuperArguments;

impl Cop for SuperArguments {
    fn name(&self) -> &str {
        "Style/SuperArguments"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use super without arguments when passing all args"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let def_regex = Regex::new(r#"def\s+\w+\(([^)]+)\)"#).unwrap();
        let super_regex = Regex::new(r#"\bsuper\(([^)]+)\)"#).unwrap();

        let mut current_method_params = String::new();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            if let Some(capture) = def_regex.captures(line) {
                if let Some(params) = capture.get(1) {
                    current_method_params = params.as_str().to_string();
                }
            }

            if let Some(capture) = super_regex.captures(line) {
                if let Some(args) = capture.get(1) {
                    if args.as_str().trim() == current_method_params.trim() {
                        let col = capture.get(0).unwrap().start() + 1;
                        if !source.in_string_or_comment(line_number, col) {
                            offenses.push(Offense::new(
                                self.name(),
                                r#"Use 'super' without arguments when passing all method arguments"#,
                                self.severity(),
                                Location::new(line_number, col, capture.get(0).unwrap().len()),
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
// 57. SuperWithArgsParentheses - parens with super
// ============================================================================

pub struct SuperWithArgsParentheses;

impl Cop for SuperWithArgsParentheses {
    fn name(&self) -> &str {
        "Style/SuperWithArgsParentheses"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use parentheses with super when passing arguments"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"\bsuper\s+\w+"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) && !line.contains("super()") {
                    offenses.push(Offense::new(
                        self.name(),
                        r#"Use parentheses with 'super' when passing arguments"#,
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 58. SwapValues - use parallel assignment to swap
// ============================================================================

pub struct SwapValues;

impl Cop for SwapValues {
    fn name(&self) -> &str {
        "Style/SwapValues"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use parallel assignment to swap values"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        for line_num in 1..(source.line_count().saturating_sub(2)) {
            if let (Some(line1), Some(line2), Some(line3)) = 
                (source.line(line_num), source.line(line_num + 1), source.line(line_num + 2)) {
                let trim1 = line1.trim();
                let trim2 = line2.trim();
                let trim3 = line3.trim();
                
                // Pattern: temp = a; a = b; b = temp
                if trim1.starts_with("temp") && trim1.contains('=') &&
                   trim2.contains('=') && trim3.starts_with("temp") {
                    if let Some(pos) = line1.find("temp") {
                        offenses.push(Offense::new(
                            self.name(),
                            r#"Use parallel assignment to swap values: 'a, b = b, a'"#,
                            self.severity(),
                            Location::new(line_num, pos + 1, 4),
                        ));
                    }
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 59. TopLevelMethodDefinition - no methods at top level
// ============================================================================

pub struct TopLevelMethodDefinition;

impl Cop for TopLevelMethodDefinition {
    fn name(&self) -> &str {
        "Style/TopLevelMethodDefinition"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Define methods inside classes or modules"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let mut in_class_or_module = false;

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            let trimmed = line.trim();

            if trimmed.starts_with("class ") || trimmed.starts_with("module ") {
                in_class_or_module = true;
            } else if trimmed == "end" {
                in_class_or_module = false;
            } else if !in_class_or_module && trimmed.starts_with("def ") {
                if let Some(pos) = line.find("def ") {
                    offenses.push(Offense::new(
                        self.name(),
                        "Define methods inside classes or modules, not at top level",
                        self.severity(),
                        Location::new(line_number, pos + 1, 3),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 60. HashFetchChain - chain of '.fetch' calls
// ============================================================================

pub struct HashFetchChain;

impl Cop for HashFetchChain {
    fn name(&self) -> &str {
        "Style/HashFetchChain"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use dig instead of chained fetch calls"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"\.fetch\([^)]+\)\.fetch\("#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        r#"Use 'dig' instead of chained 'fetch' calls"#,
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 61. HashLookupMethod - consistent hash lookup
// ============================================================================

pub struct HashLookupMethod;

impl Cop for HashLookupMethod {
    fn name(&self) -> &str {
        "Style/HashLookupMethod"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use consistent hash lookup method"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"\.fetch\(:\w+,\s*nil\)"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        r#"Use '[]' instead of 'fetch' with nil default"#,
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 62. ConstantVisibility - visibility for constants
// ============================================================================

pub struct ConstantVisibility;

impl Cop for ConstantVisibility {
    fn name(&self) -> &str {
        "Style/ConstantVisibility"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Explicitly declare constant visibility"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"^\s*[A-Z][A-Z_]*\s*="#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            // Check previous line for visibility modifier
            let has_visibility = if line_num > 0 {
                if let Some(prev_line) = source.line(line_num) {
                    let prev_trim = prev_line.trim();
                    prev_trim == "private" || prev_trim == "public" || prev_trim == "protected"
                } else {
                    false
                }
            } else {
                false
            };

            if !has_visibility {
                for capture in regex.find_iter(line) {
                    let col = capture.start() + 1;
                    if !source.in_string_or_comment(line_number, col) {
                        offenses.push(Offense::new(
                            self.name(),
                            "Explicitly declare constant visibility",
                            self.severity(),
                            Location::new(line_number, col, capture.len()),
                        ));
                    }
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 63. AmbiguousEndlessMethodDefinition - ambiguous endless method
// ============================================================================

pub struct AmbiguousEndlessMethodDefinition;

impl Cop for AmbiguousEndlessMethodDefinition {
    fn name(&self) -> &str {
        "Style/AmbiguousEndlessMethodDefinition"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Ambiguous endless method definition"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"def\s+\w+\s*=\s*\w+"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            if !line.contains("(") && line.contains("def ") {
                for capture in regex.find_iter(line) {
                    let col = capture.start() + 1;
                    if !source.in_string_or_comment(line_number, col) {
                        offenses.push(Offense::new(
                            self.name(),
                            "Use parentheses to clarify endless method definition",
                            self.severity(),
                            Location::new(line_number, col, capture.len()),
                        ));
                    }
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 64. ArgumentsForwarding - use '...' forwarding
// ============================================================================

pub struct ArgumentsForwarding;

impl Cop for ArgumentsForwarding {
    fn name(&self) -> &str {
        "Style/ArgumentsForwarding"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use ... for argument forwarding in Ruby 2.7+"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"def\s+\w+\(\*args,\s*\*\*kwargs,\s*&block\)"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        r#"Use '...' for argument forwarding"#,
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 65. AutoResourceCleanup - use block form for resources
// ============================================================================

pub struct AutoResourceCleanup;

impl Cop for AutoResourceCleanup {
    fn name(&self) -> &str {
        "Style/AutoResourceCleanup"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use block form for automatic resource cleanup"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"File\.open\([^)]+\)"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if source.in_string_or_comment(line_number, col) {
                    continue;
                }

                // Check if followed by block (either { or do)
                let after_match = &line[capture.end()..];
                let trimmed = after_match.trim_start();
                if trimmed.starts_with('{') || trimmed.starts_with("do") {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Use block form of File.open for automatic resource cleanup",
                    self.severity(),
                    Location::new(line_number, col, capture.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// 66. BisectedAttrAccessor - split attr_accessor
// ============================================================================

pub struct BisectedAttrAccessor;

impl Cop for BisectedAttrAccessor {
    fn name(&self) -> &str {
        "Style/BisectedAttrAccessor"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Combine attr_reader and attr_writer into attr_accessor"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let reader_regex = Regex::new(r#"attr_reader\s+:(\w+)"#).unwrap();
        let writer_regex = Regex::new(r#"attr_writer\s+:(\w+)"#).unwrap();

        let mut readers = std::collections::HashSet::new();
        let mut writers = std::collections::HashMap::new();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for capture in reader_regex.captures_iter(line) {
                if let Some(attr) = capture.get(1) {
                    readers.insert(attr.as_str().to_string());
                }
            }
            
            for capture in writer_regex.captures_iter(line) {
                if let Some(attr) = capture.get(1) {
                    writers.insert(attr.as_str().to_string(), line_number);
                }
            }
        }

        for (attr, line_num) in writers {
            if readers.contains(&attr) {
                if let Some(line) = source.line(line_num) {
                    if let Some(pos) = line.find("attr_writer") {
                        offenses.push(Offense::new(
                            self.name(),
                            "Combine attr_reader and attr_writer into attr_accessor",
                            self.severity(),
                            Location::new(line_num, pos + 1, 11),
                        ));
                    }
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 67. BitwisePredicate - use 'anybits?'/'allbits?'
// ============================================================================

pub struct BitwisePredicate;

impl Cop for BitwisePredicate {
    fn name(&self) -> &str {
        "Style/BitwisePredicate"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use anybits? or allbits? for bitwise operations"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"\w+\s*&\s*\w+\s*[=><!]+\s*\d+"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        r#"Use 'anybits?' or 'allbits?' for bitwise predicates"#,
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 68. ClassMethodsDefinitions - class methods style
// ============================================================================

pub struct ClassMethodsDefinitions;

impl Cop for ClassMethodsDefinitions {
    fn name(&self) -> &str {
        "Style/ClassMethodsDefinitions"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use consistent style for class method definitions"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"def\s+self\.(\w+)"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        r#"Prefer 'class << self' for multiple class methods"#,
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 69. CollectionMethods - method name preferences
// ============================================================================

pub struct CollectionMethods;

impl Cop for CollectionMethods {
    fn name(&self) -> &str {
        "Style/CollectionMethods"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use preferred collection method names"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"\.(collect|inject|detect|find_all)[\s\{(]"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.captures_iter(line) {
                if let Some(method) = capture.get(1) {
                    let col = method.start() + 1;
                    if !source.in_string_or_comment(line_number, col) {
                        let preferred = match method.as_str() {
                            "collect" => "map",
                            "inject" => "reduce",
                            "detect" => "find",
                            "find_all" => "select",
                            _ => continue,
                        };
                        offenses.push(Offense::new(
                            self.name(),
                            format!("Use '{}' instead of '{}'", preferred, method.as_str()),
                            self.severity(),
                            Location::new(line_number, col, method.len()),
                        ));
                    }
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 70. CollectionQuerying - use '.any?'/'.none?' properly
// ============================================================================

pub struct CollectionQuerying;

impl Cop for CollectionQuerying {
    fn name(&self) -> &str {
        "Style/CollectionQuerying"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use collection querying methods efficiently"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"\.select\s*\{[^}]+\}\.(empty\?|any\?|none\?)"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        r#"Use 'any?' or 'none?' directly instead of 'select' followed by query"#,
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 71. CombinableDefined - combine 'defined?' checks
// ============================================================================

pub struct CombinableDefined;

impl Cop for CombinableDefined {
    fn name(&self) -> &str {
        "Style/CombinableDefined"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Combine multiple defined? checks"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"defined\?\(\w+\)\s+&&\s+defined\?\(\w+\)"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Combine multiple defined? checks",
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 72. ComparableBetween - use 'between?'
// ============================================================================

pub struct ComparableBetween;

impl Cop for ComparableBetween {
    fn name(&self) -> &str {
        "Style/ComparableBetween"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use between? for range comparisons"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"\w+\s*>=\s*\d+\s+&&\s+\w+\s*<=\s*\d+"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        r#"Use 'between?' instead of range comparison"#,
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 73. ComparableClamp - use 'clamp'
// ============================================================================

pub struct ComparableClamp;

impl Cop for ComparableClamp {
    fn name(&self) -> &str {
        "Style/ComparableClamp"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use clamp for min/max constraints"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"\[\[(\w+),\s*\d+\]\.max,\s*\d+\]\.min"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        r#"Use 'clamp' instead of nested min/max"#,
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 74. DocumentDynamicEvalDefinition - document eval definitions
// ============================================================================

pub struct DocumentDynamicEvalDefinition;

impl Cop for DocumentDynamicEvalDefinition {
    fn name(&self) -> &str {
        "Style/DocumentDynamicEvalDefinition"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Document dynamically evaluated code"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"\b(eval|class_eval|module_eval|instance_eval)\("#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            // Check if previous line has a comment
            let has_comment = if line_num > 0 {
                if let Some(prev_line) = source.line(line_num) {
                    prev_line.trim().starts_with('#')
                } else {
                    false
                }
            } else {
                false
            };

            if !has_comment {
                for capture in regex.find_iter(line) {
                    let col = capture.start() + 1;
                    if !source.in_string_or_comment(line_number, col) {
                        offenses.push(Offense::new(
                            self.name(),
                            "Document dynamically evaluated code with a comment",
                            self.severity(),
                            Location::new(line_number, col, capture.len()),
                        ));
                    }
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 75. DocumentationMethod - public methods should have docs
// ============================================================================

pub struct DocumentationMethod;

impl Cop for DocumentationMethod {
    fn name(&self) -> &str {
        "Style/DocumentationMethod"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Public methods should have documentation"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let def_regex = Regex::new(r#"^\s*def\s+[a-z_]"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            if def_regex.is_match(line) {
                // Check if previous line has a comment
                let has_doc = if line_num > 0 {
                    if let Some(prev_line) = source.line(line_num) {
                        prev_line.trim().starts_with('#')
                    } else {
                        false
                    }
                } else {
                    false
                };

                if !has_doc {
                    if let Some(pos) = line.find("def ") {
                        offenses.push(Offense::new(
                            self.name(),
                            "Public methods should be documented",
                            self.severity(),
                            Location::new(line_number, pos + 1, 3),
                        ));
                    }
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 76. EmptyClassDefinition - empty class
// ============================================================================

pub struct EmptyClassDefinition;

impl Cop for EmptyClassDefinition {
    fn name(&self) -> &str {
        "Style/EmptyClassDefinition"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Empty class definition"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let mut class_start = 0;
        let mut in_class = false;

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            let trimmed = line.trim();

            if trimmed.starts_with("class ") {
                in_class = true;
                class_start = line_number;
            } else if in_class && trimmed == "end" {
                // Check if empty
                let mut has_content = false;
                for i in (class_start + 1)..line_number {
                    if let Some(mid_line) = source.line(i) {
                        if !mid_line.trim().is_empty() && !mid_line.trim().starts_with('#') {
                            has_content = true;
                            break;
                        }
                    }
                }

                if !has_content {
                    if let Some(class_line) = source.line(class_start) {
                        if let Some(pos) = class_line.find("class ") {
                            offenses.push(Offense::new(
                                self.name(),
                                "Empty class definition",
                                self.severity(),
                                Location::new(class_start, pos + 1, 5),
                            ));
                        }
                    }
                }
                in_class = false;
            }
        }

        offenses
    }
}

// ============================================================================
// 77. ExplicitBlockArgument - explicit &block
// ============================================================================

pub struct ExplicitBlockArgument;

impl Cop for ExplicitBlockArgument {
    fn name(&self) -> &str {
        "Style/ExplicitBlockArgument"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use explicit &block parameter when yielding"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let def_regex = Regex::new(r#"def\s+\w+\([^)]*\)"#).unwrap();
        let mut in_method_without_block = false;
        let mut _method_start = 0;

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            let trimmed = line.trim();

            if let Some(_capture) = def_regex.captures(trimmed) {
                if !line.contains("&block") && !line.contains("&blk") {
                    in_method_without_block = true;
                    _method_start = line_number;
                } else {
                    in_method_without_block = false;
                }
            }

            if in_method_without_block && trimmed.contains("yield") {
                if let Some(pos) = line.find("yield") {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use explicit &block parameter instead of yield",
                        self.severity(),
                        Location::new(line_number, pos + 1, 5),
                    ));
                }
            }

            if trimmed == "end" {
                in_method_without_block = false;
            }
        }

        offenses
    }
}

// ============================================================================
// 78. FloatDivision - use fdiv
// ============================================================================

pub struct FloatDivision;

impl Cop for FloatDivision {
    fn name(&self) -> &str {
        "Style/FloatDivision"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use fdiv for float division"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"\d+\s*/\s*\d+\.0|\d+\.\d+\s*/\s*\d+"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        r#"Use 'fdiv' for float division"#,
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 79. ItAssignment - assignment to 'it'
// ============================================================================

pub struct ItAssignment;

impl Cop for ItAssignment {
    fn name(&self) -> &str {
        "Style/ItAssignment"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        r#"Do not assign to 'it'"#
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"\bit\s*=\s*"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        r#"Do not assign to reserved parameter name 'it'"#,
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 80. ItBlockParameter - use 'it' block parameter
// ============================================================================

pub struct ItBlockParameter;

impl Cop for ItBlockParameter {
    fn name(&self) -> &str {
        "Style/ItBlockParameter"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        r#"Use 'it' for single-parameter blocks in Ruby 3.4+"#
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"\{\s*\|(\w+)\|\s*(\w+)\s*\}"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.captures_iter(line) {
                let param = capture.get(1).unwrap().as_str();
                let body = capture.get(2).unwrap().as_str();

                // Only flag if parameter and body are identical (identity block)
                if param != body {
                    continue;
                }

                if let Some(matched) = capture.get(0) {
                    let col = matched.start() + 1;
                    if !source.in_string_or_comment(line_number, col) {
                        offenses.push(Offense::new(
                            self.name(),
                            r#"Use implicit 'it' parameter for identity blocks"#,
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

// ============================================================================
// 81. KeywordArgumentsMerging - merge keyword args
// ============================================================================

pub struct KeywordArgumentsMerging;

impl Cop for KeywordArgumentsMerging {
    fn name(&self) -> &str {
        "Style/KeywordArgumentsMerging"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Merge keyword arguments"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"\.merge\(\*\*\w+\)"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use double splat operator instead of merge",
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 82. MapCompactWithConditionalBlock - '.map.compact' -> '.filter_map'
// ============================================================================

pub struct MapCompactWithConditionalBlock;

impl Cop for MapCompactWithConditionalBlock {
    fn name(&self) -> &str {
        "Style/MapCompactWithConditionalBlock"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use filter_map instead of map.compact"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"\.map\s*\{[^}]+\}\s*\.compact"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        r#"Use 'filter_map' instead of 'map.compact'"#,
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 83. MapIntoArray - '.each_with_object([])' -> '.map'
// ============================================================================

pub struct MapIntoArray;

impl Cop for MapIntoArray {
    fn name(&self) -> &str {
        "Style/MapIntoArray"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use map instead of each_with_object([])"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"\.each_with_object\(\[\]\)"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        r#"Use 'map' instead of 'each_with_object([])'"#,
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 84. MapToSet - '.map.to_set' -> '.to_set { }'
// ============================================================================

pub struct MapToSet;

impl Cop for MapToSet {
    fn name(&self) -> &str {
        "Style/MapToSet"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use to_set with block instead of map.to_set"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"\.map\s*\{[^}]+\}\s*\.to_set"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        r#"Use 'to_set { }' instead of 'map { }.to_set'"#,
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 85. MinMaxComparison - redundant min/max comparison
// ============================================================================

pub struct MinMaxComparison;

impl Cop for MinMaxComparison {
    fn name(&self) -> &str {
        "Style/MinMaxComparison"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Redundant comparison with min/max result"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"\[(\w+),\s*\d+\]\.(min|max)\s*[<>=]+\s*\d+"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Redundant comparison with min/max result",
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 86. ModuleMemberExistenceCheck - use 'const_defined?'
// ============================================================================

pub struct ModuleMemberExistenceCheck;

impl Cop for ModuleMemberExistenceCheck {
    fn name(&self) -> &str {
        "Style/ModuleMemberExistenceCheck"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use const_defined? instead of constants.include?"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"\.constants\.include\?"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        r#"Use 'const_defined?' instead of 'constants.include?'"#,
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 87. MultilineInPatternThen - no 'then' in multiline pattern
// ============================================================================

pub struct MultilineInPatternThen;

impl Cop for MultilineInPatternThen {
    fn name(&self) -> &str {
        "Style/MultilineInPatternThen"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Do not use then in multiline pattern matching"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"^\s*in\s+.+\s+then\s*$"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        r#"Do not use 'then' in multiline pattern matching"#,
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 88. NegatedIfElseCondition - negated if with else
// ============================================================================

pub struct NegatedIfElseCondition;

impl Cop for NegatedIfElseCondition {
    fn name(&self) -> &str {
        "Style/NegatedIfElseCondition"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Avoid negated if with else branch"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"^\s*if\s+!"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            if regex.is_match(line) {
                // Look ahead for else
                for i in (line_number + 1)..(line_number + 20).min(source.line_count() + 1) {
                    if let Some(future_line) = source.line(i) {
                        if future_line.trim() == "else" {
                            if let Some(pos) = line.find("if !") {
                                offenses.push(Offense::new(
                                    self.name(),
                                    "Swap if and else branches to avoid negation",
                                    self.severity(),
                                    Location::new(line_number, pos + 1, 4),
                                ));
                            }
                            break;
                        } else if future_line.trim() == "end" {
                            break;
                        }
                    }
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 89. NegativeArrayIndex - use '.last' not '[-1]'
// ============================================================================

pub struct NegativeArrayIndex;

impl Cop for NegativeArrayIndex {
    fn name(&self) -> &str {
        "Style/NegativeArrayIndex"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use last instead of negative array index"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"\[-1\]"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        r#"Use 'last' instead of '[-1]'"#,
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 90. NestedFileDirname - nested 'File.dirname'
// ============================================================================

pub struct NestedFileDirname;

impl Cop for NestedFileDirname {
    fn name(&self) -> &str {
        "Style/NestedFileDirname"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use Pathname or simplify nested File.dirname"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"File\.dirname\(File\.dirname\("#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        r#"Use Pathname or simplify nested 'File.dirname'"#,
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 91. NumberedParametersLimit - limit numbered params
// ============================================================================

pub struct NumberedParametersLimit;

impl Cop for NumberedParametersLimit {
    fn name(&self) -> &str {
        "Style/NumberedParametersLimit"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Limit use of numbered parameters"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"\{[^}]*_[3-9][^}]*\}"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use named parameters instead of numbered parameters beyond _2",
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 92. SafeNavigationChainLength - limit '&.' chain length
// ============================================================================

pub struct SafeNavigationChainLength;

impl Cop for SafeNavigationChainLength {
    fn name(&self) -> &str {
        "Style/SafeNavigationChainLength"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Limit safe navigation chain length"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            let count = line.matches("&.").count();
            
            if count > 2 {
                if let Some(pos) = line.find("&.") {
                    offenses.push(Offense::new(
                        self.name(),
                        "Limit safe navigation chain length to 2",
                        self.severity(),
                        Location::new(line_number, pos + 1, 2),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 93. YAMLFileRead - use 'YAML.safe_load_file'
// ============================================================================

pub struct YAMLFileRead;

impl Cop for YAMLFileRead {
    fn name(&self) -> &str {
        "Style/YAMLFileRead"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use YAML.safe_load_file instead of YAML.load(File.read)"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"YAML\.(load|safe_load)\(File\.read\("#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        r#"Use 'YAML.safe_load_file' instead of 'YAML.load(File.read)'"#,
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// 94. FileTouch - use 'FileUtils.touch'
// ============================================================================

pub struct FileTouch;

impl Cop for FileTouch {
    fn name(&self) -> &str {
        "Style/FileTouch"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use FileUtils.touch instead of File.open with empty write"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let regex = Regex::new(r#"File\.open\([^)]+,\s*['"]w['"]\)\s*\{\s*\}"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for capture in regex.find_iter(line) {
                let col = capture.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        r#"Use 'FileUtils.touch' instead of empty 'File.open' with 'w' mode"#,
                        self.severity(),
                        Location::new(line_number, col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// Registry function to get all cops from this module
// ============================================================================

pub fn all_style_extra3_cops() -> Vec<Box<dyn Cop>> {
    vec![
        Box::new(OptionHash),
        Box::new(OptionalArguments),
        Box::new(ParallelAssignment),
        Box::new(PreferredHashMethods),
        Box::new(QuotedSymbols),
        // Box::new(RaiseArgs),  // Duplicate - registered in style_extra1
        Box::new(RandomWithOffset),
        Box::new(RedundantArgument),
        Box::new(RedundantArrayConstructor),
        Box::new(RedundantArrayFlatten),
        Box::new(RedundantAssignment),
        Box::new(RedundantCapitalW),
        Box::new(RedundantCondition),
        Box::new(RedundantConditional),
        Box::new(RedundantConstantBase),
        Box::new(RedundantCurrentDirectoryInPath),
        Box::new(RedundantDoubleSplatHashBraces),
        Box::new(RedundantEach),
        Box::new(RedundantFetchBlock),
        Box::new(RedundantFileExtensionInRequire),
        Box::new(RedundantFilterChain),
        Box::new(RedundantFormat),
        Box::new(RedundantHeredocDelimiterQuotes),
        Box::new(RedundantInitialize),
        Box::new(RedundantInterpolationUnfreeze),
        Box::new(RedundantLineContinuation),
        Box::new(RedundantRegexpArgument),
        Box::new(RedundantRegexpCharacterClass),
        Box::new(RedundantRegexpConstructor),
        Box::new(RedundantRegexpEscape),
        Box::new(RedundantSelfAssignment),
        Box::new(RedundantSelfAssignmentBranch),
        Box::new(RedundantSort),
        Box::new(RedundantSortBy),
        Box::new(RedundantStringEscape),
        Box::new(RequireOrder),
        Box::new(ReturnNil),
        Box::new(ReturnNilInPredicateMethodDefinition),
        Box::new(ReverseFind),
        Box::new(Sample),
        Box::new(SelectByRegexp),
        Box::new(SendWithLiteralMethodName),
        Box::new(SingleArgumentDig),
        Box::new(SingleLineBlockParams),
        Box::new(SingleLineDoEndBlock),
        Box::new(SlicingWithRange),
        Box::new(SoleNestedConditional),
        Box::new(StabbyLambdaParentheses),
        Box::new(StaticClass),
        // Box::new(StderrPuts),  // Duplicate - registered in style_extra1
        Box::new(StringChars),
        Box::new(StringHashKeys),
        Box::new(StringLiteralsInInterpolation),
        Box::new(StringMethods),
        Box::new(Strip),
        Box::new(SuperArguments),
        Box::new(SuperWithArgsParentheses),
        Box::new(SwapValues),
        Box::new(TopLevelMethodDefinition),
        Box::new(HashFetchChain),
        Box::new(HashLookupMethod),
        Box::new(ConstantVisibility),
        Box::new(AmbiguousEndlessMethodDefinition),
        Box::new(ArgumentsForwarding),
        Box::new(AutoResourceCleanup),
        Box::new(BisectedAttrAccessor),
        Box::new(BitwisePredicate),
        Box::new(ClassMethodsDefinitions),
        Box::new(CollectionMethods),
        Box::new(CollectionQuerying),
        Box::new(CombinableDefined),
        Box::new(ComparableBetween),
        Box::new(ComparableClamp),
        Box::new(DocumentDynamicEvalDefinition),
        Box::new(DocumentationMethod),
        // Box::new(EmptyClassDefinition),  // Duplicate - registered in style_extra2
        // Box::new(ExplicitBlockArgument),  // Duplicate - registered in style_extra2
        // Box::new(FloatDivision),  // Duplicate - registered in style_extra2
        Box::new(ItAssignment),
        Box::new(ItBlockParameter),
        Box::new(KeywordArgumentsMerging),
        Box::new(MapCompactWithConditionalBlock),
        Box::new(MapIntoArray),
        Box::new(MapToSet),
        Box::new(MinMaxComparison),
        Box::new(ModuleMemberExistenceCheck),
        Box::new(MultilineInPatternThen),
        Box::new(NegatedIfElseCondition),
        Box::new(NegativeArrayIndex),
        Box::new(NestedFileDirname),
        Box::new(NumberedParametersLimit),
        Box::new(SafeNavigationChainLength),
        Box::new(YAMLFileRead),
        Box::new(FileTouch),
    ]
}

// ============================================================================
// TESTS
// ============================================================================



#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn test_source(content: &str) -> SourceFile {
        SourceFile::from_string(PathBuf::from("test.rb"), content.to_string())
    }

    // ===== OptionHash Tests =====
    #[test]
    fn test_option_hash_detected() {
        let source = test_source("def foo(options = {})\nend\n");
        let cop = OptionHash;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert!(offenses[0].message.contains("keyword"));
    }

    #[test]
    fn test_option_hash_ok() {
        let source = test_source("def foo(name:, age: 18)\nend\n");
        let cop = OptionHash;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // ===== OptionalArguments Tests =====
    #[test]
    fn test_optional_arguments_wrong_order() {
        let source = test_source("def foo(opt = 1, required)\nend\n");
        let cop = OptionalArguments;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_optional_arguments_correct_order() {
        let source = test_source("def foo(required, opt = 1)\nend\n");
        let cop = OptionalArguments;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // ===== ParallelAssignment Tests =====
    #[test]
    fn test_parallel_assignment_detected() {
        let source = test_source("a, b = 1, 2\n");
        let cop = ParallelAssignment;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_parallel_assignment_ok() {
        let source = test_source("a = 1\nb = 2\n");
        let cop = ParallelAssignment;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // ===== PreferredHashMethods Tests =====
    #[test]
    fn test_preferred_hash_methods_has_key() {
        let source = test_source("hash.has_key?(:foo)\n");
        let cop = PreferredHashMethods;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert!(offenses[0].message.contains("key?"));
    }

    #[test]
    fn test_preferred_hash_methods_ok() {
        let source = test_source("hash.key?(:foo)\n");
        let cop = PreferredHashMethods;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // ===== QuotedSymbols Tests =====
    #[test]
    fn test_quoted_symbols_unnecessary() {
        let source = test_source("x = :\"hello\"\n");
        let cop = QuotedSymbols;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_quoted_symbols_ok() {
        let source = test_source("x = :hello\n");
        let cop = QuotedSymbols;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // ===== RaiseArgs Tests =====
    #[test]
    fn test_raise_args_verbose() {
        let source = test_source("raise StandardError.new('message')\n");
        let cop = RaiseArgs;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_raise_args_ok() {
        let source = test_source("raise StandardError, 'message'\n");
        let cop = RaiseArgs;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // ===== RandomWithOffset Tests =====
    #[test]
    fn test_random_with_offset_detected() {
        let source = test_source("x = rand(10) + 1\n");
        let cop = RandomWithOffset;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_random_with_offset_ok() {
        let source = test_source("x = rand(1..10)\n");
        let cop = RandomWithOffset;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // ===== PreferredHashMethods has_value Tests =====
    #[test]
    fn test_preferred_hash_methods_has_value() {
        let source = test_source("hash.has_value?(42)\n");
        let cop = PreferredHashMethods;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert!(offenses[0].message.contains("value?"));
    }

    // ===== RedundantCapitalW Tests =====
    #[test]
    fn test_redundant_capital_w_detected() {
        let source = test_source("arr = %W(foo bar)\n");
        let cop = RedundantCapitalW;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_redundant_capital_w_ok() {
        let source = test_source("arr = %w(foo bar)\n");
        let cop = RedundantCapitalW;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // ===== RedundantCondition Tests =====
    #[test]
    fn test_redundant_condition_detected() {
        let source = test_source("result = x ? x : y\n");
        let cop = RedundantCondition;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_redundant_condition_ok() {
        let source = test_source("result = x || y\n");
        let cop = RedundantCondition;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // ===== RedundantFileExtensionInRequire Tests =====
    #[test]
    fn test_redundant_file_extension_detected() {
        let source = test_source("require 'foo.rb'\n");
        let cop = RedundantFileExtensionInRequire;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_redundant_file_extension_ok() {
        let source = test_source("require 'foo'\n");
        let cop = RedundantFileExtensionInRequire;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // ===== Sample Tests =====
    #[test]
    fn test_sample_detected() {
        let source = test_source("x = arr.shuffle.first\n");
        let cop = Sample;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_sample_ok() {
        let source = test_source("x = arr.sample\n");
        let cop = Sample;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // ===== StderrPuts Tests =====
    #[test]
    fn test_stderr_puts_detected() {
        let source = test_source("$stderr.puts 'error'\n");
        let cop = StderrPuts;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_stderr_puts_ok() {
        let source = test_source("warn 'error'\n");
        let cop = StderrPuts;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // ===== StringChars Tests =====
    #[test]
    fn test_string_chars_detected() {
        let source = test_source("x = str.split('')\n");
        let cop = StringChars;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_string_chars_ok() {
        let source = test_source("x = str.chars\n");
        let cop = StringChars;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // ===== Strip Tests =====
    #[test]
    fn test_strip_detected() {
        let source = test_source("x = str.lstrip.rstrip\n");
        let cop = Strip;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_strip_ok() {
        let source = test_source("x = str.strip\n");
        let cop = Strip;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // ===== TopLevelMethodDefinition Tests =====
    #[test]
    fn test_top_level_method_detected() {
        let source = test_source("def foo\n  42\nend\n");
        let cop = TopLevelMethodDefinition;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_top_level_method_ok() {
        let source = test_source("class Foo\n  def bar\n    42\n  end\nend\n");
        let cop = TopLevelMethodDefinition;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // ===== NegativeArrayIndex Tests =====
    #[test]
    fn test_negative_array_index_detected() {
        let source = test_source("x = arr[-1]\n");
        let cop = NegativeArrayIndex;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_negative_array_index_ok() {
        let source = test_source("x = arr.last\n");
        let cop = NegativeArrayIndex;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // ===== YAMLFileRead Tests =====
    #[test]
    fn test_yaml_file_read_detected() {
        let source = test_source("data = YAML.load(File.read('config.yml'))\n");
        let cop = YAMLFileRead;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_yaml_file_read_ok() {
        let source = test_source("data = YAML.safe_load_file('config.yml')\n");
        let cop = YAMLFileRead;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // ===== Additional Tests for remaining cops =====

    // RedundantArrayConstructor
    #[test]
    fn test_redundant_array_constructor() {
        let source = test_source("x = Array.new([])\n");
        let cop = RedundantArrayConstructor;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_redundant_array_constructor_ok() {
        let source = test_source("x = []\n");
        let cop = RedundantArrayConstructor;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // RedundantSort
    #[test]
    fn test_redundant_sort() {
        let source = test_source("x = arr.sort.first\n");
        let cop = RedundantSort;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_redundant_sort_ok() {
        let source = test_source("x = arr.min\n");
        let cop = RedundantSort;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // RedundantSortBy
    #[test]
    fn test_redundant_sort_by() {
        let source = test_source("arr.sort_by { |x| x }\n");
        let cop = RedundantSortBy;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_redundant_sort_by_ok() {
        let source = test_source("arr.sort\n");
        let cop = RedundantSortBy;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // ReturnNil
    #[test]
    fn test_return_nil() {
        let source = test_source("def foo\n  return nil\nend\n");
        let cop = ReturnNil;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_return_nil_ok() {
        let source = test_source("def foo\n  return\nend\n");
        let cop = ReturnNil;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // SelectByRegexp
    #[test]
    fn test_select_by_regexp() {
        let source = test_source("arr.select { |x| x =~ /pattern/ }\n");
        let cop = SelectByRegexp;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_select_by_regexp_ok() {
        let source = test_source("arr.grep(/pattern/)\n");
        let cop = SelectByRegexp;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // SendWithLiteralMethodName
    #[test]
    fn test_send_with_literal() {
        let source = test_source("obj.send(:foo)\n");
        let cop = SendWithLiteralMethodName;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_send_with_literal_ok() {
        let source = test_source("obj.foo\n");
        let cop = SendWithLiteralMethodName;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // SingleArgumentDig
    #[test]
    fn test_single_argument_dig() {
        let source = test_source("hash.dig(:key)\n");
        let cop = SingleArgumentDig;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_single_argument_dig_ok() {
        let source = test_source("hash[:key]\n");
        let cop = SingleArgumentDig;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // StringHashKeys
    #[test]
    fn test_string_hash_keys() {
        let source = test_source("{ \"name\" => \"John\" }\n");
        let cop = StringHashKeys;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_string_hash_keys_ok() {
        let source = test_source("{ name: \"John\" }\n");
        let cop = StringHashKeys;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // StringMethods
    #[test]
    fn test_string_methods_gsub() {
        let source = test_source("str.gsub('a', 'b')\n");
        let cop = StringMethods;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_string_methods_ok() {
        let source = test_source("str.tr('a', 'b')\n");
        let cop = StringMethods;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // RedundantInitialize
    #[test]
    fn test_redundant_initialize() {
        let source = test_source("class Foo\n  def initialize\n  end\nend\n");
        let cop = RedundantInitialize;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_redundant_initialize_ok() {
        let source = test_source("class Foo\n  def initialize\n    @x = 1\n  end\nend\n");
        let cop = RedundantInitialize;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // RedundantFilterChain
    #[test]
    fn test_redundant_filter_chain() {
        let source = test_source("arr.select { |x| x > 5 }.first\n");
        let cop = RedundantFilterChain;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_redundant_filter_chain_ok() {
        let source = test_source("arr.detect { |x| x > 5 }\n");
        let cop = RedundantFilterChain;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // RedundantEach
    #[test]
    fn test_redundant_each() {
        let source = test_source("arr.each.map { |x| x * 2 }\n");
        let cop = RedundantEach;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_redundant_each_ok() {
        let source = test_source("arr.map { |x| x * 2 }\n");
        let cop = RedundantEach;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // RedundantRegexpConstructor
    #[test]
    fn test_redundant_regexp_constructor() {
        let source = test_source("pattern = Regexp.new('test')\n");
        let cop = RedundantRegexpConstructor;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_redundant_regexp_constructor_ok() {
        let source = test_source("pattern = /test/\n");
        let cop = RedundantRegexpConstructor;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // RedundantFormat
    #[test]
    fn test_redundant_format() {
        let source = test_source("format('%s', name)\n");
        let cop = RedundantFormat;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_redundant_format_ok() {
        let source = test_source("name.to_s\n");
        let cop = RedundantFormat;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // RedundantConstantBase
    #[test]
    fn test_redundant_constant_base() {
        let source = test_source("x = ::String.new\n");
        let cop = RedundantConstantBase;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_redundant_constant_base_ok() {
        let source = test_source("x = String.new\n");
        let cop = RedundantConstantBase;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // RedundantCurrentDirectoryInPath
    #[test]
    fn test_redundant_current_directory() {
        let source = test_source("require './lib/foo'\n");
        let cop = RedundantCurrentDirectoryInPath;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_redundant_current_directory_ok() {
        let source = test_source("require 'lib/foo'\n");
        let cop = RedundantCurrentDirectoryInPath;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // StabbyLambdaParentheses
    #[test]
    fn test_stabby_lambda_parens() {
        let source = test_source("-> x { x * 2 }\n");
        let cop = StabbyLambdaParentheses;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_stabby_lambda_parens_ok() {
        let source = test_source("->(x) { x * 2 }\n");
        let cop = StabbyLambdaParentheses;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // StaticClass
    #[test]
    fn test_static_class() {
        let source = test_source("class Utils\n  def self.foo\n  end\nend\n");
        let cop = StaticClass;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_static_class_ok() {
        let source = test_source("module Utils\n  def self.foo\n  end\nend\n");
        let cop = StaticClass;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // SlicingWithRange
    #[test]
    fn test_slicing_with_range() {
        let source = test_source("arr[0..5]\n");
        let cop = SlicingWithRange;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_slicing_with_range_ok() {
        let source = test_source("arr.first(5)\n");
        let cop = SlicingWithRange;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // ReverseFind
    #[test]
    fn test_reverse_find() {
        let source = test_source("arr.reverse.find { |x| x > 0 }\n");
        let cop = ReverseFind;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_reverse_find_ok() {
        let source = test_source("arr.reverse_each.find { |x| x > 0 }\n");
        let cop = ReverseFind;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // HashFetchChain
    #[test]
    fn test_hash_fetch_chain() {
        let source = test_source("hash.fetch(:a).fetch(:b)\n");
        let cop = HashFetchChain;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_hash_fetch_chain_ok() {
        let source = test_source("hash.dig(:a, :b)\n");
        let cop = HashFetchChain;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // CollectionMethods
    #[test]
    fn test_collection_methods_collect() {
        let source = test_source("arr.collect { |x| x * 2 }\n");
        let cop = CollectionMethods;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_collection_methods_ok() {
        let source = test_source("arr.map { |x| x * 2 }\n");
        let cop = CollectionMethods;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // AutoResourceCleanup
    #[test]
    fn test_auto_resource_cleanup() {
        let source = test_source("f = File.open('test.txt')\n");
        let cop = AutoResourceCleanup;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_auto_resource_cleanup_ok() {
        let source = test_source("File.open('test.txt') { |f| f.read }\n");
        let cop = AutoResourceCleanup;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // NestedFileDirname
    #[test]
    fn test_nested_file_dirname() {
        let source = test_source("path = File.dirname(File.dirname(__FILE__))\n");
        let cop = NestedFileDirname;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_nested_file_dirname_ok() {
        let source = test_source("path = Pathname.new(__FILE__).parent.parent\n");
        let cop = NestedFileDirname;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // EmptyClassDefinition
    #[test]
    fn test_empty_class_definition() {
        let source = test_source("class Foo\nend\n");
        let cop = EmptyClassDefinition;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_empty_class_definition_ok() {
        let source = test_source("class Foo\n  def bar\n  end\nend\n");
        let cop = EmptyClassDefinition;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // MapCompactWithConditionalBlock
    #[test]
    fn test_map_compact() {
        let source = test_source("arr.map { |x| x if x > 0 }.compact\n");
        let cop = MapCompactWithConditionalBlock;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_map_compact_ok() {
        let source = test_source("arr.filter_map { |x| x if x > 0 }\n");
        let cop = MapCompactWithConditionalBlock;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // MapIntoArray
    #[test]
    fn test_map_into_array() {
        let source = test_source("arr.each_with_object([]) { |x, acc| acc << x * 2 }\n");
        let cop = MapIntoArray;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_map_into_array_ok() {
        let source = test_source("arr.map { |x| x * 2 }\n");
        let cop = MapIntoArray;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // SafeNavigationChainLength
    #[test]
    fn test_safe_navigation_chain() {
        let source = test_source("obj&.foo&.bar&.baz\n");
        let cop = SafeNavigationChainLength;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_safe_navigation_chain_ok() {
        let source = test_source("obj&.foo&.bar\n");
        let cop = SafeNavigationChainLength;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // ComparableBetween
    #[test]
    fn test_comparable_between() {
        let source = test_source("if x >= 1 && x <= 10\nend\n");
        let cop = ComparableBetween;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_comparable_between_ok() {
        let source = test_source("if x.between?(1, 10)\nend\n");
        let cop = ComparableBetween;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // RedundantArgument
    #[test]
    fn test_redundant_argument() {
        let source = test_source("str.split('\\s+')\n");
        let cop = RedundantArgument;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_redundant_argument_ok() {
        let source = test_source("str.split\n");
        let cop = RedundantArgument;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }
}
