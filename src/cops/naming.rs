//! Naming convention cops for Ruby code.
//!
//! These cops enforce Ruby naming conventions:
//! - Methods: snake_case
//! - Variables: snake_case
//! - Constants: SCREAMING_SNAKE_CASE
//! - Classes/Modules: PascalCase

use once_cell::sync::Lazy;
use regex::Regex;

use crate::cop::{Category, Cop, Severity};
use crate::offense::{Location, Offense};
use crate::source::SourceFile;

// Compile regexes once at startup
static METHOD_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"^\s*def\s+(?:self\.)?([a-zA-Z_][a-zA-Z0-9_]*[?!=]?|\[\]=?|<=>|<<|>>|==|!=|<=|>=|\+|-|\*|/|%|\*\*|\||&|\^|~|!|<|>)"
    ).unwrap()
});

static VALID_METHOD_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-z_][a-z0-9_]*[?!=]?$").unwrap()
});

static ASSIGNMENT_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"([a-zA-Z_][a-zA-Z0-9_]*)\s*=[^=>~!]").unwrap()
});

static VALID_VARIABLE_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-z_][a-z0-9_]*$").unwrap()
});

static CLASS_MODULE_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^\s*(class|module)\s+([a-zA-Z_][a-zA-Z0-9_]*)").unwrap()
});

static CONSTANT_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?:^|[^a-zA-Z0-9_])([A-Z][a-zA-Z0-9_]*)\s*=[^=>~!]").unwrap()
});

static VALID_PASCAL_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[A-Z][a-zA-Z0-9]*$").unwrap()
});

static VALID_CONSTANT_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[A-Z][A-Z0-9_]*$").unwrap()
});

/// Checks that method names use snake_case.
///
/// # Examples
///
/// ```ruby
/// # good
/// def some_method
/// def method_name?
/// def method!
/// def []
/// def <=>
///
/// # bad
/// def SomeMethod
/// def methodName
/// ```
pub struct MethodName;

impl Cop for MethodName {
    fn name(&self) -> &str {
        "Naming/MethodName"
    }

    fn category(&self) -> Category {
        Category::Naming
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Method names should use snake_case"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;

            // Check if this is a method definition
            if let Some(captures) = METHOD_PATTERN.captures(line) {
                if let Some(method_match) = captures.get(1) {
                    let method_name = method_match.as_str();

                    // Calculate column as character position (1-based)
                    let byte_offset = method_match.start();
                    let column = line[..byte_offset].chars().count() + 1;

                    // Skip if inside string or comment
                    if source.in_string_or_comment(line_number, column) {
                        continue;
                    }

                    // Check if it's a special operator method (always valid)
                    let special_methods = [
                        "[]", "[]=", "<=>", "<<", ">>", "==", "!=", "<=", ">=",
                        "+", "-", "*", "/", "%", "**", "|", "&", "^", "~", "!", "<", ">",
                    ];
                    if special_methods.contains(&method_name) {
                        continue;
                    }

                    // Validate snake_case
                    if !VALID_METHOD_PATTERN.is_match(method_name) {
                        offenses.push(Offense::new(
                            self.name(),
                            format!("Method name `{}` should use snake_case", method_name),
                            self.severity(),
                            Location::new(line_number, column, method_name.len()),
                        ));
                    }
                }
            }
        }

        offenses
    }
}

/// Checks that local variable names use snake_case.
///
/// # Examples
///
/// ```ruby
/// # good
/// some_variable = 1
/// variable_name = 2
/// _private = 3
///
/// # bad
/// SomeVariable = 1
/// variableName = 2
/// ```
pub struct VariableName;

impl Cop for VariableName {
    fn name(&self) -> &str {
        "Naming/VariableName"
    }

    fn category(&self) -> Category {
        Category::Naming
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Variable names should use snake_case"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;

            for captures in ASSIGNMENT_PATTERN.captures_iter(line) {
                if let Some(var_match) = captures.get(1) {
                    let var_name = var_match.as_str();

                    // Calculate column as character position (1-based)
                    let byte_offset = var_match.start();
                    let column = line[..byte_offset].chars().count() + 1;

                    // Skip if inside string or comment
                    if source.in_string_or_comment(line_number, column) {
                        continue;
                    }

                    // Skip constants (names starting with uppercase are constants in Ruby)
                    if var_name.starts_with(|c: char| c.is_uppercase()) {
                        continue;
                    }

                    // Check if preceded by @ or $
                    // Calculate character position before the variable name
                    let chars_before = line[..byte_offset].chars().collect::<Vec<char>>();
                    if let Some(&last_char) = chars_before.last() {
                        if last_char == '@' || last_char == '$' {
                            continue;
                        }
                    }
                    // Check for @@ (class variable)
                    if chars_before.len() >= 2 {
                        let last_two = &chars_before[chars_before.len() - 2..];
                        if last_two == ['@', '@'] {
                            continue;
                        }
                    }

                    // Validate snake_case
                    if !VALID_VARIABLE_PATTERN.is_match(var_name) {
                        offenses.push(Offense::new(
                            self.name(),
                            format!("Variable name `{}` should use snake_case", var_name),
                            self.severity(),
                            Location::new(line_number, column, var_name.len()),
                        ));
                    }
                }
            }
        }

        offenses
    }
}

/// Checks that constants use SCREAMING_SNAKE_CASE and classes/modules use PascalCase.
///
/// # Examples
///
/// ```ruby
/// # good
/// SOME_CONSTANT = 1
/// MAX_VALUE = 100
/// class MyClass
/// module MyModule
///
/// # bad
/// SomeConstant = 1
/// some_constant = 1
/// class myClass
/// module my_module
/// ```
pub struct ConstantName;

impl Cop for ConstantName {
    fn name(&self) -> &str {
        "Naming/ConstantName"
    }

    fn category(&self) -> Category {
        Category::Naming
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Constants should use SCREAMING_SNAKE_CASE; classes and modules should use PascalCase"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;

            // Check class/module names
            if let Some(captures) = CLASS_MODULE_PATTERN.captures(line) {
                if let Some(name_match) = captures.get(2) {
                    let name = name_match.as_str();

                    // Calculate column as character position (1-based)
                    let byte_offset = name_match.start();
                    let column = line[..byte_offset].chars().count() + 1;

                    // Skip if inside string or comment
                    if source.in_string_or_comment(line_number, column) {
                        continue;
                    }

                    // Validate PascalCase
                    if !VALID_PASCAL_PATTERN.is_match(name) {
                        offenses.push(Offense::new(
                            self.name(),
                            format!(
                                "Class/Module name `{}` should use PascalCase",
                                name
                            ),
                            self.severity(),
                            Location::new(line_number, column, name.len()),
                        ));
                    }
                }
            }

            // Check constant names
            for captures in CONSTANT_PATTERN.captures_iter(line) {
                if let Some(const_match) = captures.get(1) {
                    let const_name = const_match.as_str();

                    // Calculate column as character position (1-based)
                    let byte_offset = const_match.start();
                    let column = line[..byte_offset].chars().count() + 1;

                    // Skip if inside string or comment
                    if source.in_string_or_comment(line_number, column) {
                        continue;
                    }

                    // Check if this is part of a class/module definition (skip those)
                    if CLASS_MODULE_PATTERN.is_match(line) {
                        continue;
                    }

                    // Validate SCREAMING_SNAKE_CASE
                    if !VALID_CONSTANT_PATTERN.is_match(const_name) {
                        offenses.push(Offense::new(
                            self.name(),
                            format!(
                                "Constant name `{}` should use SCREAMING_SNAKE_CASE",
                                const_name
                            ),
                            self.severity(),
                            Location::new(line_number, column, const_name.len()),
                        ));
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

    // MethodName tests
    mod method_name {
        use super::*;

        #[test]
        fn test_valid_snake_case_methods() {
            let source = test_source(
                r#"
def some_method
end

def another_method
end

def method_with_numbers123
end

def _private_method
end
"#,
            );
            let cop = MethodName;
            let offenses = cop.check(&source);
            assert_eq!(offenses.len(), 0, "Valid snake_case methods should not create offenses");
        }

        #[test]
        fn test_valid_predicate_and_bang_methods() {
            let source = test_source(
                r#"
def valid?
end

def save!
end

def equals=
end
"#,
            );
            let cop = MethodName;
            let offenses = cop.check(&source);
            assert_eq!(offenses.len(), 0, "Predicate and bang methods should be valid");
        }

        #[test]
        fn test_valid_special_operator_methods() {
            let source = test_source(
                r#"
def []
end

def []=
end

def <=>
end

def <<
end

def >>
end

def ==
end

def +
end

def -
end

def *
end

def **
end
"#,
            );
            let cop = MethodName;
            let offenses = cop.check(&source);
            assert_eq!(offenses.len(), 0, "Special operator methods should be valid");
        }

        #[test]
        fn test_invalid_camel_case_method() {
            let source = test_source("def someMethod\nend\n");
            let cop = MethodName;
            let offenses = cop.check(&source);
            assert_eq!(offenses.len(), 1);
            assert!(offenses[0].message.contains("someMethod"));
            assert!(offenses[0].message.contains("snake_case"));
        }

        #[test]
        fn test_invalid_pascal_case_method() {
            let source = test_source("def SomeMethod\nend\n");
            let cop = MethodName;
            let offenses = cop.check(&source);
            assert_eq!(offenses.len(), 1);
            assert!(offenses[0].message.contains("SomeMethod"));
        }

        #[test]
        fn test_valid_self_methods() {
            let source = test_source(
                r#"
def self.class_method
end

def self.another_class_method
end
"#,
            );
            let cop = MethodName;
            let offenses = cop.check(&source);
            assert_eq!(offenses.len(), 0, "Class methods with self. should be valid");
        }

        #[test]
        fn test_invalid_self_method() {
            let source = test_source("def self.ClassMethod\nend\n");
            let cop = MethodName;
            let offenses = cop.check(&source);
            assert_eq!(offenses.len(), 1);
        }

        #[test]
        fn test_method_in_string_ignored() {
            let source = test_source(r#"x = "def BadMethod""#);
            let cop = MethodName;
            let offenses = cop.check(&source);
            assert_eq!(offenses.len(), 0, "Methods in strings should be ignored");
        }

        #[test]
        fn test_method_in_comment_ignored() {
            let source = test_source("# def BadMethod\n");
            let cop = MethodName;
            let offenses = cop.check(&source);
            assert_eq!(offenses.len(), 0, "Methods in comments should be ignored");
        }

        #[test]
        fn test_empty_file() {
            let source = test_source("");
            let cop = MethodName;
            let offenses = cop.check(&source);
            assert_eq!(offenses.len(), 0);
        }

        #[test]
        fn test_multiple_invalid_methods() {
            let source = test_source(
                r#"
def BadMethod
end

def anotherBad
end

def YetAnother
end
"#,
            );
            let cop = MethodName;
            let offenses = cop.check(&source);
            assert_eq!(offenses.len(), 3, "Should detect all invalid method names");
        }
    }

    // VariableName tests
    mod variable_name {
        use super::*;

        #[test]
        fn test_valid_snake_case_variables() {
            let source = test_source(
                r#"
some_variable = 1
another_var = 2
var_with_numbers123 = 3
_private_var = 4
"#,
            );
            let cop = VariableName;
            let offenses = cop.check(&source);
            assert_eq!(offenses.len(), 0, "Valid snake_case variables should not create offenses");
        }

        #[test]
        fn test_invalid_camel_case_variable() {
            let source = test_source("someVariable = 1\n");
            let cop = VariableName;
            let offenses = cop.check(&source);
            assert_eq!(offenses.len(), 1);
            assert!(offenses[0].message.contains("someVariable"));
            assert!(offenses[0].message.contains("snake_case"));
        }

        #[test]
        fn test_pascal_case_treated_as_constant() {
            // In Ruby, names starting with uppercase are constants, not variables.
            // VariableName cop should skip them (ConstantName cop handles them).
            let source = test_source("SomeVariable = 1\n");
            let cop = VariableName;
            let offenses = cop.check(&source);
            assert_eq!(offenses.len(), 0, "Uppercase names are constants, not variables");
        }

        #[test]
        fn test_constants_ignored() {
            let source = test_source(
                r#"
SOME_CONSTANT = 1
MAX_VALUE = 100
PI = 3.14
"#,
            );
            let cop = VariableName;
            let offenses = cop.check(&source);
            assert_eq!(offenses.len(), 0, "Constants should be ignored by VariableName cop");
        }

        #[test]
        fn test_instance_variables_ignored() {
            let source = test_source(
                r#"
@instance_var = 1
@BadInstanceVar = 2
"#,
            );
            let cop = VariableName;
            let offenses = cop.check(&source);
            assert_eq!(offenses.len(), 0, "Instance variables should be ignored");
        }

        #[test]
        fn test_class_variables_ignored() {
            let source = test_source(
                r#"
@@class_var = 1
@@BadClassVar = 2
"#,
            );
            let cop = VariableName;
            let offenses = cop.check(&source);
            assert_eq!(offenses.len(), 0, "Class variables should be ignored");
        }

        #[test]
        fn test_global_variables_ignored() {
            let source = test_source(
                r#"
$global_var = 1
$BadGlobalVar = 2
"#,
            );
            let cop = VariableName;
            let offenses = cop.check(&source);
            assert_eq!(offenses.len(), 0, "Global variables should be ignored");
        }

        #[test]
        fn test_equality_operator_not_matched() {
            let source = test_source("if x == 5\nend\n");
            let cop = VariableName;
            let offenses = cop.check(&source);
            assert_eq!(offenses.len(), 0, "Equality operator should not be matched as assignment");
        }

        #[test]
        fn test_variable_in_string_ignored() {
            let source = test_source(r#"x = "BadVariable = 1""#);
            let cop = VariableName;
            let offenses = cop.check(&source);
            assert_eq!(offenses.len(), 0, "Variables in strings should be ignored");
        }

        #[test]
        fn test_variable_in_comment_ignored() {
            let source = test_source("# BadVariable = 1\n");
            let cop = VariableName;
            let offenses = cop.check(&source);
            assert_eq!(offenses.len(), 0, "Variables in comments should be ignored");
        }

        #[test]
        fn test_multiple_invalid_variables() {
            // In Ruby, names starting with uppercase are constants, not variables.
            // Only lowercase-starting camelCase names are flagged.
            let source = test_source(
                r#"
anotherBad = 1
someCamel = 2
also_Bad_Mix = 3
"#,
            );
            let cop = VariableName;
            let offenses = cop.check(&source);
            assert_eq!(offenses.len(), 3, "Should detect non-snake_case variable names");
        }

        #[test]
        fn test_empty_file() {
            let source = test_source("");
            let cop = VariableName;
            let offenses = cop.check(&source);
            assert_eq!(offenses.len(), 0);
        }
    }

    // ConstantName tests
    mod constant_name {
        use super::*;

        #[test]
        fn test_valid_screaming_snake_case_constants() {
            let source = test_source(
                r#"
SOME_CONSTANT = 1
MAX_VALUE = 100
PI = 3.14
CONSTANT_WITH_NUMBERS123 = 5
"#,
            );
            let cop = ConstantName;
            let offenses = cop.check(&source);
            assert_eq!(offenses.len(), 0, "Valid SCREAMING_SNAKE_CASE constants should not create offenses");
        }

        #[test]
        fn test_invalid_pascal_case_constant() {
            let source = test_source("SomeConstant = 1\n");
            let cop = ConstantName;
            let offenses = cop.check(&source);
            assert_eq!(offenses.len(), 1);
            assert!(offenses[0].message.contains("SomeConstant"));
            assert!(offenses[0].message.contains("SCREAMING_SNAKE_CASE"));
        }

        #[test]
        fn test_invalid_camel_case_constant() {
            let source = test_source("SomeValue = 1\n");
            let cop = ConstantName;
            let offenses = cop.check(&source);
            assert_eq!(offenses.len(), 1);
        }

        #[test]
        fn test_valid_class_names() {
            let source = test_source(
                r#"
class MyClass
end

class AnotherClass
end

class ClassWithNumbers123
end
"#,
            );
            let cop = ConstantName;
            let offenses = cop.check(&source);
            assert_eq!(offenses.len(), 0, "Valid PascalCase class names should not create offenses");
        }

        #[test]
        fn test_valid_module_names() {
            let source = test_source(
                r#"
module MyModule
end

module AnotherModule
end
"#,
            );
            let cop = ConstantName;
            let offenses = cop.check(&source);
            assert_eq!(offenses.len(), 0, "Valid PascalCase module names should not create offenses");
        }

        #[test]
        fn test_invalid_class_name_with_underscore() {
            let source = test_source("class My_Class\nend\n");
            let cop = ConstantName;
            let offenses = cop.check(&source);
            assert_eq!(offenses.len(), 1);
            assert!(offenses[0].message.contains("My_Class"));
            assert!(offenses[0].message.contains("PascalCase"));
        }

        #[test]
        fn test_invalid_module_name_snake_case() {
            let source = test_source("module my_module\nend\n");
            let cop = ConstantName;
            let offenses = cop.check(&source);
            assert_eq!(offenses.len(), 1);
            assert!(offenses[0].message.contains("my_module"));
        }

        #[test]
        fn test_class_in_string_ignored() {
            let source = test_source(r#"x = "class bad_class""#);
            let cop = ConstantName;
            let offenses = cop.check(&source);
            assert_eq!(offenses.len(), 0, "Classes in strings should be ignored");
        }

        #[test]
        fn test_constant_in_comment_ignored() {
            let source = test_source("# BadConstant = 1\n");
            let cop = ConstantName;
            let offenses = cop.check(&source);
            assert_eq!(offenses.len(), 0, "Constants in comments should be ignored");
        }

        #[test]
        fn test_multiple_invalid_constants() {
            let source = test_source(
                r#"
BadConstant = 1
AnotherBad = 2
class bad_class
end
module Bad_Module
end
"#,
            );
            let cop = ConstantName;
            let offenses = cop.check(&source);
            assert_eq!(offenses.len(), 4, "Should detect all invalid constant/class/module names");
        }

        #[test]
        fn test_empty_file() {
            let source = test_source("");
            let cop = ConstantName;
            let offenses = cop.check(&source);
            assert_eq!(offenses.len(), 0);
        }

        #[test]
        fn test_equality_operator_not_matched() {
            let source = test_source("if CONSTANT == 5\nend\n");
            let cop = ConstantName;
            let offenses = cop.check(&source);
            assert_eq!(offenses.len(), 0, "Equality operator should not be matched");
        }

        #[test]
        fn test_single_letter_constant_valid() {
            let source = test_source("X = 1\n");
            let cop = ConstantName;
            let offenses = cop.check(&source);
            assert_eq!(offenses.len(), 0, "Single uppercase letter constants should be valid");
        }
    }

    // Integration tests
    mod integration {
        use super::*;

        #[test]
        fn test_complex_file_with_multiple_naming_issues() {
            let source = test_source(
                r#"
class MyClass
  CONSTANT = 1
  BadConstant = 2

  def good_method
    local_var = 1
    camelVar = 2
  end

  def BadMethod
    x = 3
  end
end
"#,
            );

            let method_cop = MethodName;
            let var_cop = VariableName;
            let const_cop = ConstantName;

            let method_offenses = method_cop.check(&source);
            let var_offenses = var_cop.check(&source);
            let const_offenses = const_cop.check(&source);

            assert_eq!(method_offenses.len(), 1, "Should find 1 bad method (BadMethod)");
            assert_eq!(var_offenses.len(), 1, "Should find 1 bad variable (camelVar)");
            assert_eq!(const_offenses.len(), 1, "Should find 1 bad constant (BadConstant)");
        }

        #[test]
        fn test_cop_trait_implementations() {
            let method_cop = MethodName;
            assert_eq!(method_cop.name(), "Naming/MethodName");
            assert_eq!(method_cop.category(), Category::Naming);
            assert_eq!(method_cop.severity(), Severity::Convention);

            let var_cop = VariableName;
            assert_eq!(var_cop.name(), "Naming/VariableName");
            assert_eq!(var_cop.category(), Category::Naming);
            assert_eq!(var_cop.severity(), Severity::Convention);

            let const_cop = ConstantName;
            assert_eq!(const_cop.name(), "Naming/ConstantName");
            assert_eq!(const_cop.category(), Category::Naming);
            assert_eq!(const_cop.severity(), Severity::Convention);
        }
    }
}
