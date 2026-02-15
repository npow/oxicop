//! Additional Naming convention cops for Ruby code.

use once_cell::sync::Lazy;
use regex::Regex;

use crate::cop::{Category, Cop, Severity};
use crate::offense::{Location, Offense};
use crate::source::SourceFile;

// ============================================================================
// REGEX PATTERNS
// ============================================================================

static ACCESSOR_METHOD_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"^\s*def\s+(get_|set_)([a-z_][a-z0-9_]*)"#).unwrap()
});

static ASCII_IDENTIFIER_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"[^\x00-\x7F]"#).unwrap()
});

static BINARY_OPERATOR_PARAM_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"^\s*def\s+(\+|-|\*|/|%|==|!=|<|>|<=|>=|<=>|&|\||\^|<<|>>)\s*\(\s*([a-z_][a-z0-9_]*)\s*\)"#).unwrap()
});

static HEREDOC_DELIMITER_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"<<[-~]?([A-Za-z_][A-Za-z0-9_]*)"#).unwrap()
});

static PREDICATE_METHOD_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"^\s*def\s+(is_|has_|have_)([a-z_][a-z0-9_]*)\?"#).unwrap()
});

static RESCUE_VAR_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"\brescue\s+\S+\s*=>\s*([a-z_][a-z0-9_]*)"#).unwrap()
});

static FILE_NAME_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"^[a-z][a-z0-9_]*\.rb$"#).unwrap()
});

static CAMEL_CASE_CLASS: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"^\s*(class|module)\s+([A-Z][a-zA-Z0-9]*)"#).unwrap()
});

// ============================================================================
// COPS IMPLEMENTATION
// ============================================================================

/// Detects accessor method names with get_/set_ prefix
pub struct AccessorMethodName;

impl Cop for AccessorMethodName {
    fn name(&self) -> &str { "Naming/AccessorMethodName" }
    fn category(&self) -> Category { Category::Naming }
    fn severity(&self) -> Severity { Severity::Convention }
    fn description(&self) -> &str {
        "Accessor method names should not have get_/set_ prefix"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for cap in ACCESSOR_METHOD_PATTERN.captures_iter(line) {
                let full_match = cap.get(0).unwrap();
                let column = full_match.start() + 1;
                if source.in_string_or_comment(line_number, column) {
                    continue;
                }
                let prefix = cap.get(1).unwrap().as_str();
                let name = cap.get(2).unwrap().as_str();
                offenses.push(Offense::new(
                    self.name(),
                    format!("Use `{}` instead of `{}{}`", name, prefix, name),
                    self.severity(),
                    Location::new(line_number, column, full_match.len()),
                ));
            }
        }
        offenses
    }
}

/// Detects non-ASCII identifiers
pub struct AsciiIdentifiers;

impl Cop for AsciiIdentifiers {
    fn name(&self) -> &str { "Naming/AsciiIdentifiers" }
    fn category(&self) -> Category { Category::Naming }
    fn severity(&self) -> Severity { Severity::Convention }
    fn description(&self) -> &str {
        "Use only ASCII characters in identifiers"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let identifier_pattern = Regex::new(r#"\b([a-zA-Z_\u{0080}-\u{FFFF}][a-zA-Z0-9_\u{0080}-\u{FFFF}]*)\b"#).unwrap();
        
        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for cap in identifier_pattern.captures_iter(line) {
                let identifier = cap.get(1).unwrap().as_str();
                if ASCII_IDENTIFIER_PATTERN.is_match(identifier) {
                    let full_match = cap.get(0).unwrap();
                    let column = full_match.start() + 1;
                    if source.in_string_or_comment(line_number, column) {
                        continue;
                    }
                    offenses.push(Offense::new(
                        self.name(),
                        format!("Use only ASCII characters in identifiers: `{}`", identifier),
                        self.severity(),
                        Location::new(line_number, column, full_match.len()),
                    ));
                }
            }
        }
        offenses
    }
}

/// Detects binary operator parameter naming
pub struct BinaryOperatorParameterName;

impl Cop for BinaryOperatorParameterName {
    fn name(&self) -> &str { "Naming/BinaryOperatorParameterName" }
    fn category(&self) -> Category { Category::Naming }
    fn severity(&self) -> Severity { Severity::Convention }
    fn description(&self) -> &str {
        "Binary operator methods should use `other` for parameter name"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for cap in BINARY_OPERATOR_PARAM_PATTERN.captures_iter(line) {
                let param_name = cap.get(2).unwrap().as_str();
                if param_name != "other" {
                    let full_match = cap.get(0).unwrap();
                    let column = full_match.start() + 1;
                    if source.in_string_or_comment(line_number, column) {
                        continue;
                    }
                    offenses.push(Offense::new(
                        self.name(),
                        format!("Use `other` as parameter name for binary operators, not `{}`", param_name),
                        self.severity(),
                        Location::new(line_number, column, full_match.len()),
                    ));
                }
            }
        }
        offenses
    }
}

/// Detects block forwarding naming
pub struct BlockForwarding;

impl Cop for BlockForwarding {
    fn name(&self) -> &str { "Naming/BlockForwarding" }
    fn category(&self) -> Category { Category::Naming }
    fn severity(&self) -> Severity { Severity::Convention }
    fn description(&self) -> &str {
        "Use anonymous block forwarding"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let pattern = Regex::new(r#"def\s+\w+\([^)]*&block\)"#).unwrap();
        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in pattern.find_iter(line) {
                let column = mat.start() + 1;
                if source.in_string_or_comment(line_number, column) {
                    continue;
                }
                offenses.push(Offense::new(
                    self.name(),
                    "Use `&` for anonymous block forwarding instead of `&block`",
                    self.severity(),
                    Location::new(line_number, column, mat.len()),
                ));
            }
        }
        offenses
    }
}

/// Detects block parameter naming issues
pub struct BlockParameterName;

impl Cop for BlockParameterName {
    fn name(&self) -> &str { "Naming/BlockParameterName" }
    fn category(&self) -> Category { Category::Naming }
    fn severity(&self) -> Severity { Severity::Convention }
    fn description(&self) -> &str {
        "Block parameter names should be descriptive"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let pattern = Regex::new(r#"\{\s*\|([a-z])\|"#).unwrap();
        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for cap in pattern.captures_iter(line) {
                let param = cap.get(1).unwrap().as_str();
                if param.len() == 1 && param != "_" {
                    let full_match = cap.get(0).unwrap();
                    let column = full_match.start() + 1;
                    if source.in_string_or_comment(line_number, column) {
                        continue;
                    }
                    offenses.push(Offense::new(
                        self.name(),
                        format!("Block parameter `{}` is too short. Use descriptive names.", param),
                        self.severity(),
                        Location::new(line_number, column, full_match.len()),
                    ));
                }
            }
        }
        offenses
    }
}

/// Checks class/module names use CamelCase
pub struct ClassAndModuleCamelCase;

impl Cop for ClassAndModuleCamelCase {
    fn name(&self) -> &str { "Naming/ClassAndModuleCamelCase" }
    fn category(&self) -> Category { Category::Naming }
    fn severity(&self) -> Severity { Severity::Convention }
    fn description(&self) -> &str {
        "Class and module names should use CamelCase"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let pattern = Regex::new(r#"^\s*(class|module)\s+([a-z_][a-zA-Z0-9_]*)"#).unwrap();
        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for cap in pattern.captures_iter(line) {
                let full_match = cap.get(0).unwrap();
                let column = full_match.start() + 1;
                if source.in_string_or_comment(line_number, column) {
                    continue;
                }
                let name = cap.get(2).unwrap().as_str();
                offenses.push(Offense::new(
                    self.name(),
                    format!("Class/Module name `{}` should use CamelCase", name),
                    self.severity(),
                    Location::new(line_number, column, full_match.len()),
                ));
            }
        }
        offenses
    }
}

/// Checks file name conventions
pub struct FileName;

impl Cop for FileName {
    fn name(&self) -> &str { "Naming/FileName" }
    fn category(&self) -> Category { Category::Naming }
    fn severity(&self) -> Severity { Severity::Convention }
    fn description(&self) -> &str {
        "File names should use snake_case"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        if let Some(file_name) = source.path.file_name() {
            let name_str = file_name.to_string_lossy();
            if !FILE_NAME_PATTERN.is_match(&name_str) && name_str.ends_with(".rb") {
                offenses.push(Offense::new(
                    self.name(),
                    format!("File name `{}` should use snake_case", name_str),
                    self.severity(),
                    Location::new(1, 1, 0),
                ));
            }
        }
        offenses
    }
}

/// Checks heredoc delimiter case
pub struct HeredocDelimiterCase;

impl Cop for HeredocDelimiterCase {
    fn name(&self) -> &str { "Naming/HeredocDelimiterCase" }
    fn category(&self) -> Category { Category::Naming }
    fn severity(&self) -> Severity { Severity::Convention }
    fn description(&self) -> &str {
        "Heredoc delimiters should use SCREAMING_SNAKE_CASE"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for cap in HEREDOC_DELIMITER_PATTERN.captures_iter(line) {
                let delimiter = cap.get(1).unwrap().as_str();
                if delimiter.chars().any(|c| c.is_lowercase()) {
                    let full_match = cap.get(0).unwrap();
                    let column = full_match.start() + 1;
                    offenses.push(Offense::new(
                        self.name(),
                        format!("Heredoc delimiter `{}` should use SCREAMING_SNAKE_CASE", delimiter),
                        self.severity(),
                        Location::new(line_number, column, full_match.len()),
                    ));
                }
            }
        }
        offenses
    }
}

/// Checks heredoc delimiter naming
pub struct HeredocDelimiterNaming;

impl Cop for HeredocDelimiterNaming {
    fn name(&self) -> &str { "Naming/HeredocDelimiterNaming" }
    fn category(&self) -> Category { Category::Naming }
    fn severity(&self) -> Severity { Severity::Convention }
    fn description(&self) -> &str {
        "Heredoc delimiters should be meaningful"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for cap in HEREDOC_DELIMITER_PATTERN.captures_iter(line) {
                let delimiter = cap.get(1).unwrap().as_str();
                if delimiter == "EOS" || delimiter == "EOF" || delimiter.len() < 3 {
                    let full_match = cap.get(0).unwrap();
                    let column = full_match.start() + 1;
                    offenses.push(Offense::new(
                        self.name(),
                        format!("Use meaningful heredoc delimiter instead of `{}`", delimiter),
                        self.severity(),
                        Location::new(line_number, column, full_match.len()),
                    ));
                }
            }
        }
        offenses
    }
}

/// Checks for inclusive language
pub struct InclusiveLanguage;

impl Cop for InclusiveLanguage {
    fn name(&self) -> &str { "Naming/InclusiveLanguage" }
    fn category(&self) -> Category { Category::Naming }
    fn severity(&self) -> Severity { Severity::Convention }
    fn description(&self) -> &str {
        "Use inclusive terminology"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let terms = [
            ("whitelist", "allowlist"),
            ("blacklist", "denylist"),
            ("slave", "replica"),
            ("master", "primary"),
        ];
        
        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for (bad_term, good_term) in &terms {
                let pattern = Regex::new(&format!(r"\b{}\b", regex::escape(bad_term))).unwrap();
                for mat in pattern.find_iter(line) {
                    let column = mat.start() + 1;
                    if source.in_string_or_comment(line_number, column) {
                        continue;
                    }
                    offenses.push(Offense::new(
                        self.name(),
                        format!("Use `{}` instead of `{}`", good_term, bad_term),
                        self.severity(),
                        Location::new(line_number, column, mat.len()),
                    ));
                }
            }
        }
        offenses
    }
}

/// Checks memoized instance variable names
pub struct MemoizedInstanceVariableName;

impl Cop for MemoizedInstanceVariableName {
    fn name(&self) -> &str { "Naming/MemoizedInstanceVariableName" }
    fn category(&self) -> Category { Category::Naming }
    fn severity(&self) -> Severity { Severity::Convention }
    fn description(&self) -> &str {
        "Memoized instance variable should match method name"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let pattern = Regex::new(r#"def\s+([a-z_][a-z0-9_]*)\s.*@([a-z_][a-z0-9_]*)\s*\|\|="#).unwrap();
        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for cap in pattern.captures_iter(line) {
                let method_name = cap.get(1).unwrap().as_str();
                let var_name = cap.get(2).unwrap().as_str();
                if method_name != var_name {
                    let full_match = cap.get(0).unwrap();
                    let column = full_match.start() + 1;
                    offenses.push(Offense::new(
                        self.name(),
                        format!("Memoized variable `@{}` should match method name `{}`", var_name, method_name),
                        self.severity(),
                        Location::new(line_number, column, full_match.len()),
                    ));
                }
            }
        }
        offenses
    }
}

/// Checks method parameter name length
pub struct MethodParameterName;

impl Cop for MethodParameterName {
    fn name(&self) -> &str { "Naming/MethodParameterName" }
    fn category(&self) -> Category { Category::Naming }
    fn severity(&self) -> Severity { Severity::Convention }
    fn description(&self) -> &str {
        "Method parameter names should be at least 2 characters"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let pattern = Regex::new(r#"def\s+\w+\(([a-z])\)"#).unwrap();
        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for cap in pattern.captures_iter(line) {
                let param = cap.get(1).unwrap().as_str();
                if param.len() == 1 && param != "_" {
                    let full_match = cap.get(0).unwrap();
                    let column = full_match.start() + 1;
                    if source.in_string_or_comment(line_number, column) {
                        continue;
                    }
                    offenses.push(Offense::new(
                        self.name(),
                        format!("Parameter `{}` is too short. Use at least 2 characters.", param),
                        self.severity(),
                        Location::new(line_number, column, full_match.len()),
                    ));
                }
            }
        }
        offenses
    }
}

/// Checks predicate method naming
pub struct PredicateMethod;

impl Cop for PredicateMethod {
    fn name(&self) -> &str { "Naming/PredicateMethod" }
    fn category(&self) -> Category { Category::Naming }
    fn severity(&self) -> Severity { Severity::Convention }
    fn description(&self) -> &str {
        "Predicate methods should end with ?"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let pattern = Regex::new(r#"def\s+(is|has|should|can|will)_[a-z_][a-z0-9_]*\b"#).unwrap();
        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in pattern.find_iter(line) {
                let column = mat.start() + 1;
                if source.in_string_or_comment(line_number, column) {
                    continue;
                }
                offenses.push(Offense::new(
                    self.name(),
                    "Predicate methods should end with `?`",
                    self.severity(),
                    Location::new(line_number, column, mat.len()),
                ));
            }
        }
        offenses
    }
}

/// Checks predicate prefix naming
pub struct PredicatePrefix;

impl Cop for PredicatePrefix {
    fn name(&self) -> &str { "Naming/PredicatePrefix" }
    fn category(&self) -> Category { Category::Naming }
    fn severity(&self) -> Severity { Severity::Convention }
    fn description(&self) -> &str {
        "Avoid is_/has_ prefix on predicate methods"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for cap in PREDICATE_METHOD_PATTERN.captures_iter(line) {
                let full_match = cap.get(0).unwrap();
                let column = full_match.start() + 1;
                if source.in_string_or_comment(line_number, column) {
                    continue;
                }
                let prefix = cap.get(1).unwrap().as_str();
                let name = cap.get(2).unwrap().as_str();
                offenses.push(Offense::new(
                    self.name(),
                    format!("Avoid `{}` prefix. Use `{}?` instead of `{}{}?`", prefix, name, prefix, name),
                    self.severity(),
                    Location::new(line_number, column, full_match.len()),
                ));
            }
        }
        offenses
    }
}

/// Checks rescued exception variable name
pub struct RescuedExceptionsVariableName;

impl Cop for RescuedExceptionsVariableName {
    fn name(&self) -> &str { "Naming/RescuedExceptionsVariableName" }
    fn category(&self) -> Category { Category::Naming }
    fn severity(&self) -> Severity { Severity::Convention }
    fn description(&self) -> &str {
        "Use `e` for rescued exception variable"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for cap in RESCUE_VAR_PATTERN.captures_iter(line) {
                let var_name = cap.get(1).unwrap().as_str();
                if var_name != "e" && var_name != "_" {
                    let full_match = cap.get(0).unwrap();
                    let column = full_match.start() + 1;
                    if source.in_string_or_comment(line_number, column) {
                        continue;
                    }
                    offenses.push(Offense::new(
                        self.name(),
                        format!("Use `=> e` instead of `=> {}` for rescued exception", var_name),
                        self.severity(),
                        Location::new(line_number, column, full_match.len()),
                    ));
                }
            }
        }
        offenses
    }
}

/// Checks variable number style
pub struct VariableNumber;

impl Cop for VariableNumber {
    fn name(&self) -> &str { "Naming/VariableNumber" }
    fn category(&self) -> Category { Category::Naming }
    fn severity(&self) -> Severity { Severity::Convention }
    fn description(&self) -> &str {
        "Variable numbers should follow style conventions"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let pattern = Regex::new(r#"\b([a-z_][a-z_]*[0-9]+[a-z_][a-z0-9_]*)\b"#).unwrap();
        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for cap in pattern.captures_iter(line) {
                let var_name = cap.get(1).unwrap().as_str();
                let full_match = cap.get(0).unwrap();
                let column = full_match.start() + 1;
                if source.in_string_or_comment(line_number, column) {
                    continue;
                }
                offenses.push(Offense::new(
                    self.name(),
                    format!("Variable `{}` has poor number placement", var_name),
                    self.severity(),
                    Location::new(line_number, column, full_match.len()),
                ));
            }
        }
        offenses
    }
}

// ============================================================================
// TESTS
// ============================================================================


// Collect all cops from this module
pub fn all_naming_extra_cops() -> Vec<Box<dyn Cop>> {
    vec![
        Box::new(AccessorMethodName),
        Box::new(AsciiIdentifiers),
        Box::new(BinaryOperatorParameterName),
        Box::new(BlockForwarding),
        Box::new(BlockParameterName),
        Box::new(ClassAndModuleCamelCase),
        Box::new(FileName),
        Box::new(HeredocDelimiterCase),
        Box::new(HeredocDelimiterNaming),
        Box::new(InclusiveLanguage),
        Box::new(MemoizedInstanceVariableName),
        Box::new(MethodParameterName),
        Box::new(PredicateMethod),
        Box::new(PredicatePrefix),
        Box::new(RescuedExceptionsVariableName),
        Box::new(VariableNumber),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn test_source(content: &str) -> SourceFile {
        SourceFile::from_string(PathBuf::from("test.rb"), content.to_string())
    }

    #[test]
    fn test_accessor_method_name() {
        let cop = AccessorMethodName;
        let source = test_source("def get_name\n  @name\nend\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert!(offenses[0].message.contains("name"));
    }

    #[test]
    fn test_accessor_method_name_valid() {
        let cop = AccessorMethodName;
        let source = test_source("def name\n  @name\nend\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_ascii_identifiers() {
        let cop = AsciiIdentifiers;
        let source = test_source("def café\nend\n");
        let offenses = cop.check(&source);
        assert!(offenses.len() >= 1);
    }

    #[test]
    fn test_binary_operator_parameter_name() {
        let cop = BinaryOperatorParameterName;
        let source = test_source("def +(value)\n  @x + value\nend\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert!(offenses[0].message.contains("other"));
    }

    #[test]
    fn test_binary_operator_parameter_correct() {
        let cop = BinaryOperatorParameterName;
        let source = test_source("def +(other)\n  @x + other\nend\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_class_camel_case() {
        let cop = ClassAndModuleCamelCase;
        let source = test_source("class my_class\nend\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_heredoc_delimiter_case() {
        let cop = HeredocDelimiterCase;
        let source = test_source("sql = <<-Sql\nSELECT * FROM users\nSql\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_inclusive_language() {
        let cop = InclusiveLanguage;
        let source = test_source("whitelist = ['allowed']\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert!(offenses[0].message.contains("allowlist"));
    }

    #[test]
    fn test_method_parameter_name() {
        let cop = MethodParameterName;
        let source = test_source("def process(x)\n  x * 2\nend\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_predicate_prefix() {
        let cop = PredicatePrefix;
        let source = test_source("def is_valid?\n  true\nend\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert!(offenses[0].message.contains("valid?"));
    }

    #[test]
    fn test_rescued_exception_variable() {
        let cop = RescuedExceptionsVariableName;
        let source = test_source("rescue StandardError => error\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert!(offenses[0].message.contains("=> e"));
    }

    #[test]
    fn test_rescued_exception_variable_correct() {
        let cop = RescuedExceptionsVariableName;
        let source = test_source("rescue StandardError => e\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }
}
