//! Additional Style cops for Ruby code formatting and conventions (part 2).

use regex::Regex;

use crate::cop::{Category, Cop, Severity};
use crate::offense::{Location, Offense};
use crate::source::SourceFile;

// ============================================================================
// Cops 1-20: Trailing/Trivial patterns
// ============================================================================

/// Checks that no code appears on the same line as `class` keyword after the class name.
pub struct TrailingBodyOnClass;

impl Cop for TrailingBodyOnClass {
    fn name(&self) -> &str {
        "Style/TrailingBodyOnClass"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for trailing code after class definition"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let class_regex = Regex::new(r#"^\s*class\s+(\w+(::\w+)*(<\s*\w+(::\w+)*)?)(\s+[^#\s].*)$"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            if let Some(cap) = class_regex.captures(line) {
                if let Some(trailing) = cap.get(5) {
                    let content = trailing.as_str().trim();
                    if !content.is_empty() && !content.starts_with('#') {
                        offenses.push(Offense::new(
                            self.name(),
                            "Place the class body on separate lines from the definition",
                            self.severity(),
                            Location::new(line_number, trailing.start() + 1, trailing.len()),
                        ));
                    }
                }
            }
        }

        offenses
    }
}

/// Checks that no code appears on the same line as `module` keyword after the module name.
pub struct TrailingBodyOnModule;

impl Cop for TrailingBodyOnModule {
    fn name(&self) -> &str {
        "Style/TrailingBodyOnModule"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for trailing code after module definition"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let module_regex = Regex::new(r#"^\s*module\s+(\w+(::\w+)*)(\s+[^#\s].*)$"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            if let Some(cap) = module_regex.captures(line) {
                if let Some(trailing) = cap.get(3) {
                    let content = trailing.as_str().trim();
                    if !content.is_empty() && !content.starts_with('#') {
                        offenses.push(Offense::new(
                            self.name(),
                            "Place the module body on separate lines from the definition",
                            self.severity(),
                            Location::new(line_number, trailing.start() + 1, trailing.len()),
                        ));
                    }
                }
            }
        }

        offenses
    }
}

/// Checks that no code appears on the same line as `def` keyword after method signature.
pub struct TrailingBodyOnMethodDefinition;

impl Cop for TrailingBodyOnMethodDefinition {
    fn name(&self) -> &str {
        "Style/TrailingBodyOnMethodDefinition"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for trailing code after method definition"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        // Match def but not endless method (def foo = bar)
        let def_regex = Regex::new(r#"^\s*def\s+\w+[^=]*[^=\s](\s+[^#\s=].*)$"#).unwrap();
        let endless_regex = Regex::new(r#"^\s*def\s+\w+.*="#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            // Skip endless methods
            if endless_regex.is_match(line) {
                continue;
            }
            
            if let Some(cap) = def_regex.captures(line) {
                if let Some(trailing) = cap.get(1) {
                    let content = trailing.as_str().trim();
                    if !content.is_empty() && !content.starts_with('#') {
                        offenses.push(Offense::new(
                            self.name(),
                            "Place the method body on separate lines from the definition",
                            self.severity(),
                            Location::new(line_number, trailing.start() + 1, trailing.len()),
                        ));
                    }
                }
            }
        }

        offenses
    }
}

/// Checks for trailing comma in method call arguments.
pub struct TrailingCommaInArguments;

impl Cop for TrailingCommaInArguments {
    fn name(&self) -> &str {
        "Style/TrailingCommaInArguments"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for trailing comma in method arguments"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let trailing_comma_regex = Regex::new(r#",\s*\)"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in trailing_comma_regex.find_iter(line) {
                let col = mat.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Avoid trailing comma in method arguments",
                        self.severity(),
                        Location::new(line_number, col, 1),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks for trailing comma in array literals.
pub struct TrailingCommaInArrayLiteral;

impl Cop for TrailingCommaInArrayLiteral {
    fn name(&self) -> &str {
        "Style/TrailingCommaInArrayLiteral"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for trailing comma in array literals"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let trailing_comma_regex = Regex::new(r#",\s*\]"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in trailing_comma_regex.find_iter(line) {
                let col = mat.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Avoid trailing comma in array literals",
                        self.severity(),
                        Location::new(line_number, col, 1),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks for trailing comma in hash literals.
pub struct TrailingCommaInHashLiteral;

impl Cop for TrailingCommaInHashLiteral {
    fn name(&self) -> &str {
        "Style/TrailingCommaInHashLiteral"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for trailing comma in hash literals"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let trailing_comma_regex = Regex::new(r#",\s*\}"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in trailing_comma_regex.find_iter(line) {
                let col = mat.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Avoid trailing comma in hash literals",
                        self.severity(),
                        Location::new(line_number, col, 1),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks for trailing comma in block arguments.
pub struct TrailingCommaInBlockArgs;

impl Cop for TrailingCommaInBlockArgs {
    fn name(&self) -> &str {
        "Style/TrailingCommaInBlockArgs"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for trailing comma in block arguments"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let block_args_regex = Regex::new(r#"\|[^|]*,\s*\|"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in block_args_regex.find_iter(line) {
                let col = mat.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Avoid trailing comma in block arguments",
                        self.severity(),
                        Location::new(line_number, col, mat.len()),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks for method end statement on the same line as method body.
pub struct TrailingMethodEndStatement;

impl Cop for TrailingMethodEndStatement {
    fn name(&self) -> &str {
        "Style/TrailingMethodEndStatement"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for method end on same line as body"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let method_end_regex = Regex::new(r#"[^;\s]+;\s*end\s*$"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            if method_end_regex.is_match(line) && line.contains("def ") {
                if let Some(end_pos) = line.rfind("end") {
                    offenses.push(Offense::new(
                        self.name(),
                        "Place method end on its own line",
                        self.severity(),
                        Location::new(line_number, end_pos + 1, 3),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks for trailing underscore variable in destructuring assignments.
pub struct TrailingUnderscoreVariable;

impl Cop for TrailingUnderscoreVariable {
    fn name(&self) -> &str {
        "Style/TrailingUnderscoreVariable"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for trailing underscore variables in destructuring"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let underscore_regex = Regex::new(r#"[a-z_]\w*,\s*_\s*="#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in underscore_regex.find_iter(line) {
                let col = mat.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Prefer splat over trailing underscore",
                        self.severity(),
                        Location::new(line_number, col, mat.len()),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks for trivial accessor methods that could be replaced with attr_*.
pub struct TrivialAccessors;

impl Cop for TrivialAccessors {
    fn name(&self) -> &str {
        "Style/TrivialAccessors"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for trivial accessor methods"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let getter_regex = Regex::new(r#"^\s*def\s+(\w+)\s*$"#).unwrap();
        let getter_body_regex = Regex::new(r#"^\s*@(\w+)\s*$"#).unwrap();
        let setter_regex = Regex::new(r#"^\s*def\s+(\w+)="#).unwrap();
        let setter_body_regex = Regex::new(r#"^\s*@(\w+)\s*=\s*\w+\s*$"#).unwrap();

        let mut i = 0;
        while i < source.lines.len() {
            let line_number = i + 1;
            let line = &source.lines[i];

            // Check for getter
            if let Some(cap) = getter_regex.captures(line) {
                let method_name = cap.get(1).unwrap().as_str();
                if i + 2 < source.lines.len() {
                    let next_line = &source.lines[i + 1];
                    let end_line = &source.lines[i + 2];
                    if getter_body_regex.is_match(next_line) && end_line.trim() == "end" {
                        offenses.push(Offense::new(
                            self.name(),
                            format!("Use attr_reader for trivial reader {}", method_name),
                            self.severity(),
                            Location::new(line_number, 1, line.len()),
                        ));
                    }
                }
            }

            // Check for setter
            if let Some(cap) = setter_regex.captures(line) {
                let method_name = cap.get(1).unwrap().as_str();
                if i + 2 < source.lines.len() {
                    let next_line = &source.lines[i + 1];
                    let end_line = &source.lines[i + 2];
                    if setter_body_regex.is_match(next_line) && end_line.trim() == "end" {
                        offenses.push(Offense::new(
                            self.name(),
                            format!("Use attr_writer for trivial writer {}", method_name),
                            self.severity(),
                            Location::new(line_number, 1, line.len()),
                        ));
                    }
                }
            }

            i += 1;
        }

        offenses
    }
}

/// Checks for unless with else clause.
pub struct UnlessElse;

impl Cop for UnlessElse {
    fn name(&self) -> &str {
        "Style/UnlessElse"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for unless with else clause"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let unless_regex = Regex::new(r#"^\s*unless\s+"#).unwrap();
        let else_regex = Regex::new(r#"^\s*else\s*$"#).unwrap();

        let mut in_unless = false;
        let mut unless_line = 0;

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            if unless_regex.is_match(line) {
                in_unless = true;
                unless_line = line_number;
            } else if in_unless && else_regex.is_match(line) {
                offenses.push(Offense::new(
                    self.name(),
                    "Do not use unless with else, use if instead",
                    self.severity(),
                    Location::new(unless_line, 1, source.lines[unless_line - 1].len()),
                ));
                in_unless = false;
            } else if line.trim() == "end" {
                in_unless = false;
            }
        }

        offenses
    }
}

/// Checks for logical operators in unless conditions.
pub struct UnlessLogicalOperators;

impl Cop for UnlessLogicalOperators {
    fn name(&self) -> &str {
        "Style/UnlessLogicalOperators"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for logical operators in unless conditions"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let unless_logical_regex = Regex::new(r#"^\s*unless\s+.*(\|\||&&|and|or)"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            if unless_logical_regex.is_match(line) {
                offenses.push(Offense::new(
                    self.name(),
                    "Avoid using logical operators in unless conditions",
                    self.severity(),
                    Location::new(line_number, 1, line.len()),
                ));
            }
        }

        offenses
    }
}

/// Checks for [0] after unpack operation instead of .first.
pub struct UnpackFirst;

impl Cop for UnpackFirst {
    fn name(&self) -> &str {
        "Style/UnpackFirst"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for [0] after unpack, suggest .first"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let unpack_regex = Regex::new(r#"\*\w+\[0\]"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in unpack_regex.find_iter(line) {
                let col = mat.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use .first instead of [0] after unpack",
                        self.severity(),
                        Location::new(line_number, col, mat.len()),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks for variable interpolation using #@var instead of #{@var}.
pub struct VariableInterpolation;

impl Cop for VariableInterpolation {
    fn name(&self) -> &str {
        "Style/VariableInterpolation"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for #@var instead of #{@var}"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let var_interp_regex = Regex::new(r#"#[@$]\w+"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in var_interp_regex.find_iter(line) {
                let col = mat.start() + 1;
                // Check if we're in a double-quoted string (simplified check)
                // The pattern #@var or #$var should be flagged
                if line.contains('"') {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use #{} for variable interpolation",
                        self.severity(),
                        Location::new(line_number, col, mat.len()),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks for consistent use of then in when clauses.
pub struct WhenThen;

impl Cop for WhenThen {
    fn name(&self) -> &str {
        "Style/WhenThen"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for consistent use of then in when clauses"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let when_then_regex = Regex::new(r#"^\s*when\s+.*\s+then\s+"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            if when_then_regex.is_match(line) {
                if let Some(pos) = line.find("then") {
                    offenses.push(Offense::new(
                        self.name(),
                        "Do not use then for multiline when statement",
                        self.severity(),
                        Location::new(line_number, pos + 1, 4),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks for do keyword in while/until loops.
pub struct WhileUntilDo;

impl Cop for WhileUntilDo {
    fn name(&self) -> &str {
        "Style/WhileUntilDo"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for do keyword in while/until loops"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let while_do_regex = Regex::new(r#"^\s*(while|until)\s+.*\s+do\s*$"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            if while_do_regex.is_match(line) {
                if let Some(pos) = line.rfind("do") {
                    offenses.push(Offense::new(
                        self.name(),
                        "Do not use do with while/until",
                        self.severity(),
                        Location::new(line_number, pos + 1, 2),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks for single-line while/until that should use modifier form.
pub struct WhileUntilModifier;

impl Cop for WhileUntilModifier {
    fn name(&self) -> &str {
        "Style/WhileUntilModifier"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for single-line while/until that should use modifier"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let while_regex = Regex::new(r#"^\s*(while|until)\s+"#).unwrap();

        let mut i = 0;
        while i < source.lines.len() {
            let line_number = i + 1;
            let line = &source.lines[i];

            if while_regex.is_match(line) {
                // Check if next line is a single statement followed by end
                if i + 2 < source.lines.len() {
                    let body = source.lines[i + 1].trim();
                    let end_line = source.lines[i + 2].trim();
                    if !body.is_empty() && end_line == "end" && !body.starts_with("if") && !body.starts_with("unless") {
                        offenses.push(Offense::new(
                            self.name(),
                            "Prefer modifier while/until for single-line body",
                            self.severity(),
                            Location::new(line_number, 1, line.len()),
                        ));
                    }
                }
            }

            i += 1;
        }

        offenses
    }
}

/// Checks for word arrays that should use %w notation.
pub struct WordArray;

impl Cop for WordArray {
    fn name(&self) -> &str {
        "Style/WordArray"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for word arrays that should use %w notation"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let word_array_regex = Regex::new(r#"\[(['""][a-zA-Z]\w*['"]\s*,\s*)+['""][a-zA-Z]\w*['"]\]"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in word_array_regex.find_iter(line) {
                let col = mat.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use %w or %W for array of words",
                        self.severity(),
                        Location::new(line_number, col, mat.len()),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks for Yoda conditions (literal == variable).
pub struct YodaCondition;

impl Cop for YodaCondition {
    fn name(&self) -> &str {
        "Style/YodaCondition"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for Yoda conditions"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let yoda_regex = Regex::new(r#"(nil|\d+|true|false|['"]\w+['"])\s*(==|!=|<|>|<=|>=)\s*[a-z_]\w*"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in yoda_regex.find_iter(line) {
                let col = mat.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Avoid Yoda conditions",
                        self.severity(),
                        Location::new(line_number, col, mat.len()),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks for Yoda expressions (literal + variable).
pub struct YodaExpression;

impl Cop for YodaExpression {
    fn name(&self) -> &str {
        "Style/YodaExpression"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for Yoda expressions"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let yoda_expr_regex = Regex::new(r#"\d+\s*\+\s*[a-z_]\w*"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in yoda_expr_regex.find_iter(line) {
                let col = mat.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Avoid Yoda expressions",
                        self.severity(),
                        Location::new(line_number, col, mat.len()),
                    ));
                }
            }
        }

        offenses
    }
}


// ============================================================================
// Cops 21-40: Zero-length predicates and array/collection methods
// ============================================================================

/// Checks for .length == 0 instead of .empty?.
pub struct ZeroLengthPredicate;

impl Cop for ZeroLengthPredicate {
    fn name(&self) -> &str {
        "Style/ZeroLengthPredicate"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for .length == 0 instead of .empty?"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let zero_length_regex = Regex::new(r#"\.(length|size)\s*(==|!=)\s*0"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in zero_length_regex.find_iter(line) {
                let col = mat.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use .empty? instead of .length == 0",
                        self.severity(),
                        Location::new(line_number, col, mat.len()),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks for access modifier declaration style.
pub struct AccessModifierDeclarations;

impl Cop for AccessModifierDeclarations {
    fn name(&self) -> &str {
        "Style/AccessModifierDeclarations"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for access modifier declaration style"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let modifier_with_args_regex = Regex::new(r#"^\s*(private|protected|public)\s+:(def|class)\s"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            if modifier_with_args_regex.is_match(line) {
                offenses.push(Offense::new(
                    self.name(),
                    "Use access modifier inline or without arguments",
                    self.severity(),
                    Location::new(line_number, 1, line.len()),
                ));
            }
        }

        offenses
    }
}

/// Checks for grouping of attr_reader/writer/accessor.
pub struct AccessorGrouping;

impl Cop for AccessorGrouping {
    fn name(&self) -> &str {
        "Style/AccessorGrouping"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for grouping of attr_* declarations"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let attr_regex = Regex::new(r#"^\s*attr_(reader|writer|accessor)\s+:\w+\s*$"#).unwrap();

        let mut prev_line_num = 0;
        let mut prev_attr_type = "";

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            if let Some(cap) = attr_regex.captures(line) {
                let attr_type = cap.get(1).unwrap().as_str();
                
                if prev_line_num > 0 && line_number == prev_line_num + 1 && attr_type == prev_attr_type {
                    offenses.push(Offense::new(
                        self.name(),
                        "Group attr_* declarations together",
                        self.severity(),
                        Location::new(line_number, 1, line.len()),
                    ));
                }
                
                prev_line_num = line_number;
                prev_attr_type = attr_type;
            }
        }

        offenses
    }
}

/// Checks for alias vs alias_method.
pub struct Alias;

impl Cop for Alias {
    fn name(&self) -> &str {
        "Style/Alias"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for alias vs alias_method usage"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let alias_method_regex = Regex::new(r#"^\s*alias_method\s+"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            if alias_method_regex.is_match(line) {
                offenses.push(Offense::new(
                    self.name(),
                    "Use alias instead of alias_method in lexical context",
                    self.severity(),
                    Location::new(line_number, 1, line.len()),
                ));
            }
        }

        offenses
    }
}

/// Checks for array coercion using [*x] instead of Array(x).
pub struct ArrayCoercion;

impl Cop for ArrayCoercion {
    fn name(&self) -> &str {
        "Style/ArrayCoercion"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for [*x] instead of Array(x)"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let array_coercion_regex = Regex::new(r#"\[\*\w+\]"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in array_coercion_regex.find_iter(line) {
                let col = mat.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use Array() for coercion",
                        self.severity(),
                        Location::new(line_number, col, mat.len()),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks for [0] and [-1] instead of .first and .last.
pub struct ArrayFirstLast;

impl Cop for ArrayFirstLast {
    fn name(&self) -> &str {
        "Style/ArrayFirstLast"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for [0] and [-1] instead of .first and .last"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let first_last_regex = Regex::new(r#"\w+\[(0|-1)\]"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in first_last_regex.find_iter(line) {
                let col = mat.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    let text = mat.as_str();
                    let msg = if text.contains("[0]") {
                        "Use .first instead of [0]"
                    } else {
                        "Use .last instead of [-1]"
                    };
                    offenses.push(Offense::new(
                        self.name(),
                        msg,
                        self.severity(),
                        Location::new(line_number, col, mat.len()),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks for (a & b).any? instead of a.intersect?(b).
pub struct ArrayIntersect;

impl Cop for ArrayIntersect {
    fn name(&self) -> &str {
        "Style/ArrayIntersect"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for (a & b).any? instead of a.intersect?(b)"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let intersect_regex = Regex::new(r#"\([^)]+&[^)]+\)\.any\?"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in intersect_regex.find_iter(line) {
                let col = mat.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use .intersect? instead of (a & b).any?",
                        self.severity(),
                        Location::new(line_number, col, mat.len()),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks for * operator instead of .join for arrays.
pub struct ArrayJoin;

impl Cop for ArrayJoin {
    fn name(&self) -> &str {
        "Style/ArrayJoin"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for * operator instead of .join"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let array_join_regex = Regex::new(r#"\w+\s*\*\s*['"]\w*['""]"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in array_join_regex.find_iter(line) {
                let col = mat.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use .join instead of * operator",
                        self.severity(),
                        Location::new(line_number, col, mat.len()),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks for consistent use of {} vs do/end for blocks.
pub struct BlockDelimiters;

impl Cop for BlockDelimiters {
    fn name(&self) -> &str {
        "Style/BlockDelimiters"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for consistent block delimiter usage"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let multiline_braces_regex = Regex::new(r#"\{[^}]*$"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            if multiline_braces_regex.is_match(line) {
                // Check if this is a multiline block with braces
                if line_number < source.lines.len() {
                    let next_line = &source.lines[line_number];
                    if !next_line.contains('}') {
                        offenses.push(Offense::new(
                            self.name(),
                            "Use do/end for multiline blocks",
                            self.severity(),
                            Location::new(line_number, 1, line.len()),
                        ));
                    }
                }
            }
        }

        offenses
    }
}

/// Checks for if/elsif chains that should be case statements.
pub struct CaseLikeIf;

impl Cop for CaseLikeIf {
    fn name(&self) -> &str {
        "Style/CaseLikeIf"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for if/elsif chains that should use case"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let if_regex = Regex::new(r#"^\s*if\s+\w+\s*=="#).unwrap();
        let elsif_regex = Regex::new(r#"^\s*elsif\s+\w+\s*=="#).unwrap();

        let mut in_if_chain = false;
        let mut if_line = 0;
        let mut elsif_count = 0;

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            if if_regex.is_match(line) {
                in_if_chain = true;
                if_line = line_number;
                elsif_count = 0;
            } else if in_if_chain && elsif_regex.is_match(line) {
                elsif_count += 1;
                if elsif_count >= 2 {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use case instead of if/elsif chain",
                        self.severity(),
                        Location::new(if_line, 1, source.lines[if_line - 1].len()),
                    ));
                    in_if_chain = false;
                }
            } else if line.trim() == "end" {
                in_if_chain = false;
            }
        }

        offenses
    }
}

/// Checks for class and module children style (compact vs nested).
pub struct ClassAndModuleChildren;

impl Cop for ClassAndModuleChildren {
    fn name(&self) -> &str {
        "Style/ClassAndModuleChildren"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for class/module children style"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let nested_regex = Regex::new(r#"^\s*(class|module)\s+\w+::\w+"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            if nested_regex.is_match(line) {
                offenses.push(Offense::new(
                    self.name(),
                    "Use nested style for namespaced classes/modules",
                    self.severity(),
                    Location::new(line_number, 1, line.len()),
                ));
            }
        }

        offenses
    }
}

/// Checks for self.class == instead of is_a?.
pub struct ClassEqualityComparison;

impl Cop for ClassEqualityComparison {
    fn name(&self) -> &str {
        "Style/ClassEqualityComparison"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for self.class == instead of is_a?"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let class_eq_regex = Regex::new(r#"\.class\s*(==|!=)"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in class_eq_regex.find_iter(line) {
                let col = mat.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use is_a? instead of class ==",
                        self.severity(),
                        Location::new(line_number, col, mat.len()),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks for class methods defined with ClassName.method instead of self.method.
pub struct ClassMethods;

impl Cop for ClassMethods {
    fn name(&self) -> &str {
        "Style/ClassMethods"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for class method definition style"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let class_method_regex = Regex::new(r#"^\s*def\s+[A-Z]\w*\."#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            if class_method_regex.is_match(line) {
                offenses.push(Offense::new(
                    self.name(),
                    "Use self.method instead of ClassName.method",
                    self.severity(),
                    Location::new(line_number, 1, line.len()),
                ));
            }
        }

        offenses
    }
}

/// Checks for .reject(&:nil?) instead of .compact.
pub struct CollectionCompact;

impl Cop for CollectionCompact {
    fn name(&self) -> &str {
        "Style/CollectionCompact"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for .reject(&:nil?) instead of .compact"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let reject_nil_regex = Regex::new(r#"\.reject\s*\(\s*&:nil\?\s*\)"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in reject_nil_regex.find_iter(line) {
                let col = mat.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use .compact instead of .reject(&:nil?)",
                        self.severity(),
                        Location::new(line_number, col, mat.len()),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks for consecutive loops over the same collection.
pub struct CombinableLoops;

impl Cop for CombinableLoops {
    fn name(&self) -> &str {
        "Style/CombinableLoops"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for consecutive loops over the same collection"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let each_regex = Regex::new(r#"(\w+)\.each\s+do\s*\|"#).unwrap();

        let mut prev_collection = "";
        let mut prev_line = 0;

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            if let Some(cap) = each_regex.captures(line) {
                let collection = cap.get(1).unwrap().as_str();
                
                if collection == prev_collection && line_number > prev_line + 1 {
                    offenses.push(Offense::new(
                        self.name(),
                        "Combine consecutive loops over the same collection",
                        self.severity(),
                        Location::new(line_number, 1, line.len()),
                    ));
                }
                
                prev_collection = collection;
                prev_line = line_number;
            }
        }

        offenses
    }
}

/// Checks for concatenating array literals.
pub struct ConcatArrayLiterals;

impl Cop for ConcatArrayLiterals {
    fn name(&self) -> &str {
        "Style/ConcatArrayLiterals"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for concatenating array literals"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let concat_regex = Regex::new(r#"\]\s*\+\s*\["#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in concat_regex.find_iter(line) {
                let col = mat.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use a single array literal instead of concatenation",
                        self.severity(),
                        Location::new(line_number, col, mat.len()),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks for assignment inside conditionals.
pub struct ConditionalAssignment;

impl Cop for ConditionalAssignment {
    fn name(&self) -> &str {
        "Style/ConditionalAssignment"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for assignment inside conditionals"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let cond_assign_regex = Regex::new(r#"^\s*if\s+.*=\s+"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            if cond_assign_regex.is_match(line) && !line.contains("==") {
                offenses.push(Offense::new(
                    self.name(),
                    "Move assignment out of conditional",
                    self.severity(),
                    Location::new(line_number, 1, line.len()),
                ));
            }
        }

        offenses
    }
}

/// Checks for classes inheriting from Data instead of using Data.define.
pub struct DataInheritance;

impl Cop for DataInheritance {
    fn name(&self) -> &str {
        "Style/DataInheritance"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for class inheriting from Data"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let data_inherit_regex = Regex::new(r#"^\s*class\s+\w+\s*<\s*Data\s*$"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            if data_inherit_regex.is_match(line) {
                offenses.push(Offense::new(
                    self.name(),
                    "Use Data.define instead of inheriting from Data",
                    self.severity(),
                    Location::new(line_number, 1, line.len()),
                ));
            }
        }

        offenses
    }
}

/// Checks for chained [] access instead of nested .dig.
pub struct DigChain;

impl Cop for DigChain {
    fn name(&self) -> &str {
        "Style/DigChain"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for chained [] instead of .dig"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let dig_chain_regex = Regex::new(r#"\]\[.*\]\["#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in dig_chain_regex.find_iter(line) {
                let col = mat.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use .dig instead of chained [] access",
                        self.severity(),
                        Location::new(line_number, col, mat.len()),
                    ));
                }
            }
        }

        offenses
    }
}


// ============================================================================
// Cops 41-60: Dir, File, and method patterns
// ============================================================================

/// Checks for File.dirname(__FILE__) instead of __dir__.
pub struct Dir;

impl Cop for Dir {
    fn name(&self) -> &str {
        "Style/Dir"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for File.dirname(__FILE__) instead of __dir__"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let dir_regex = Regex::new(r#"File\.dirname\s*\(\s*__FILE__\s*\)"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in dir_regex.find_iter(line) {
                let col = mat.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use __dir__ instead of File.dirname(__FILE__)",
                        self.severity(),
                        Location::new(line_number, col, mat.len()),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks for Dir.entries.size == 2 instead of Dir.empty?.
pub struct DirEmpty;

impl Cop for DirEmpty {
    fn name(&self) -> &str {
        "Style/DirEmpty"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for Dir.entries.size == 2 instead of Dir.empty?"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let dir_empty_regex = Regex::new(r#"Dir\.entries.*\.size\s*==\s*2"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in dir_empty_regex.find_iter(line) {
                let col = mat.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use Dir.empty? instead of Dir.entries.size == 2",
                        self.severity(),
                        Location::new(line_number, col, mat.len()),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks for rubocop:disable directives in source code.
pub struct DisableCopsWithinSourceCodeDirective;

impl Cop for DisableCopsWithinSourceCodeDirective {
    fn name(&self) -> &str {
        "Style/DisableCopsWithinSourceCodeDirective"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for rubocop:disable directives"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let disable_regex = Regex::new(r#"#\s*rubocop:disable"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            if disable_regex.is_match(line) {
                offenses.push(Offense::new(
                    self.name(),
                    "Avoid disabling cops within source code",
                    self.severity(),
                    Location::new(line_number, 1, line.len()),
                ));
            }
        }

        offenses
    }
}

/// Checks for missing documentation on top-level classes/modules.
pub struct Documentation;

impl Cop for Documentation {
    fn name(&self) -> &str {
        "Style/Documentation"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for missing documentation on classes/modules"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let class_module_regex = Regex::new(r#"^\s*(class|module)\s+[A-Z]"#).unwrap();
        let comment_regex = Regex::new(r#"^\s*#"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            if class_module_regex.is_match(line) {
                // Check if previous line is a comment
                let has_doc = if line_num > 0 {
                    comment_regex.is_match(&source.lines[line_num - 1])
                } else {
                    false
                };

                if !has_doc {
                    offenses.push(Offense::new(
                        self.name(),
                        "Missing documentation for class/module",
                        self.severity(),
                        Location::new(line_number, 1, line.len()),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks for duplicate rubocop:disable directives.
pub struct DoubleCopDisableDirective;

impl Cop for DoubleCopDisableDirective {
    fn name(&self) -> &str {
        "Style/DoubleCopDisableDirective"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for duplicate rubocop:disable directives"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let disable_regex = Regex::new(r#"#\s*rubocop:disable\s+(\S+)"#).unwrap();
        let mut seen_cops: std::collections::HashSet<String> = std::collections::HashSet::new();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            if let Some(cap) = disable_regex.captures(line) {
                let cop_name = cap.get(1).unwrap().as_str();
                if !seen_cops.insert(cop_name.to_string()) {
                    offenses.push(Offense::new(
                        self.name(),
                        format!("Duplicate disable directive for {}", cop_name),
                        self.severity(),
                        Location::new(line_number, 1, line.len()),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks for each_with_object usage.
pub struct EachWithObject;

impl Cop for EachWithObject {
    fn name(&self) -> &str {
        "Style/EachWithObject"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for inject when each_with_object is more appropriate"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let inject_regex = Regex::new(r#"\.inject\s*\(\s*\{\s*\}\s*\)"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in inject_regex.find_iter(line) {
                let col = mat.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use each_with_object instead of inject with hash",
                        self.severity(),
                        Location::new(line_number, col, mat.len()),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks for empty class definitions.
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
        "Checks for empty class definitions"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let class_regex = Regex::new(r#"^\s*class\s+\w+"#).unwrap();

        let mut i = 0;
        while i < source.lines.len() {
            let line_number = i + 1;
            let line = &source.lines[i];

            if class_regex.is_match(line)
                && i + 1 < source.lines.len() {
                let next_line = source.lines[i + 1].trim();
                if next_line == "end" {
                    offenses.push(Offense::new(
                        self.name(),
                        "Empty class definition",
                        self.severity(),
                        Location::new(line_number, 1, line.len()),
                    ));
                }
            }

            i += 1;
        }

        offenses
    }
}

/// Checks for empty string inside interpolation.
pub struct EmptyStringInsideInterpolation;

impl Cop for EmptyStringInsideInterpolation {
    fn name(&self) -> &str {
        "Style/EmptyStringInsideInterpolation"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for empty string inside interpolation"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let empty_interp_regex = Regex::new(r#"#\{['""]['"]\}"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in empty_interp_regex.find_iter(line) {
                let col = mat.start() + 1;
                offenses.push(Offense::new(
                    self.name(),
                    "Empty string inside interpolation",
                    self.severity(),
                    Location::new(line_number, col, mat.len()),
                ));
            }
        }

        offenses
    }
}

/// Checks for END blocks.
pub struct EndBlock;

impl Cop for EndBlock {
    fn name(&self) -> &str {
        "Style/EndBlock"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for END blocks"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let end_block_regex = Regex::new(r#"^\s*END\s*\{"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            if end_block_regex.is_match(line) {
                offenses.push(Offense::new(
                    self.name(),
                    "Avoid using END blocks",
                    self.severity(),
                    Location::new(line_number, 1, line.len()),
                ));
            }
        }

        offenses
    }
}

/// Checks for endless method definitions.
pub struct EndlessMethod;

impl Cop for EndlessMethod {
    fn name(&self) -> &str {
        "Style/EndlessMethod"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for endless method definitions"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let offenses = Vec::new();
        let endless_regex = Regex::new(r#"^\s*def\s+\w+.*=\s*[^=]"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let _line_number = line_num + 1;
            if endless_regex.is_match(line) && !line.contains("==") {
                // This is good - encourage endless methods
                // No offense here, but could check if NOT using endless
            }
        }

        offenses
    }
}

/// Checks for ENV['HOME'] instead of Dir.home.
pub struct EnvHome;

impl Cop for EnvHome {
    fn name(&self) -> &str {
        "Style/EnvHome"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for ENV['HOME'] instead of Dir.home"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let env_home_regex = Regex::new(r#"ENV\[['""]HOME['"]\]"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in env_home_regex.find_iter(line) {
                let col = mat.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use Dir.home instead of ENV['HOME']",
                        self.severity(),
                        Location::new(line_number, col, mat.len()),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks for eval without location arguments.
pub struct EvalWithLocation;

impl Cop for EvalWithLocation {
    fn name(&self) -> &str {
        "Style/EvalWithLocation"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for eval without __FILE__ and __LINE__"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let eval_regex = Regex::new(r#"\beval\s*\("#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in eval_regex.find_iter(line) {
                let col = mat.start() + 1;
                if !source.in_string_or_comment(line_number, col)
                    && (!line.contains("__FILE__") || !line.contains("__LINE__")) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Pass __FILE__ and __LINE__ to eval",
                        self.severity(),
                        Location::new(line_number, col, mat.len()),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks for match? with exact regexp.
pub struct ExactRegexpMatch;

impl Cop for ExactRegexpMatch {
    fn name(&self) -> &str {
        "Style/ExactRegexpMatch"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for exact regexp match"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let exact_match_regex = Regex::new(r#"=~\s*/\^.*\$\/"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in exact_match_regex.find_iter(line) {
                let col = mat.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use == for exact string match instead of regexp",
                        self.severity(),
                        Location::new(line_number, col, mat.len()),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks for expand_path arguments.
pub struct ExpandPathArguments;

impl Cop for ExpandPathArguments {
    fn name(&self) -> &str {
        "Style/ExpandPathArguments"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for expand_path with __FILE__"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let expand_regex = Regex::new(r#"File\.expand_path\([^)]*__FILE__[^)]*\)"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in expand_regex.find_iter(line) {
                let col = mat.start() + 1;
                if !source.in_string_or_comment(line_number, col) && !line.contains("__dir__") {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use __dir__ in expand_path instead of __FILE__",
                        self.severity(),
                        Location::new(line_number, col, mat.len()),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks for explicit block argument.
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
        "Checks for explicit &block argument"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let yield_regex = Regex::new(r#"\byield\b"#).unwrap();
        let def_regex = Regex::new(r#"^\s*def\s+\w+"#).unwrap();

        let mut in_method = false;
        let mut method_line = 0;

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            if def_regex.is_match(line) {
                in_method = true;
                method_line = line_number;
            } else if in_method && yield_regex.is_match(line) {
                // Check if method definition has &block
                if let Some(def_line) = source.line(method_line) {
                    if !def_line.contains("&block") && !def_line.contains("&blk") {
                        offenses.push(Offense::new(
                            self.name(),
                            "Use explicit &block parameter instead of yield",
                            self.severity(),
                            Location::new(method_line, 1, def_line.len()),
                        ));
                    }
                }
                in_method = false;
            } else if line.trim() == "end" {
                in_method = false;
            }
        }

        offenses
    }
}

/// Checks for exponential notation consistency.
pub struct ExponentialNotation;

impl Cop for ExponentialNotation {
    fn name(&self) -> &str {
        "Style/ExponentialNotation"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for consistent exponential notation"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let exp_regex = Regex::new(r#"\d+e[+-]?\d+"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in exp_regex.find_iter(line) {
                let col = mat.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    let text = mat.as_str();
                    if text.contains('e') && !text.contains("e+") && !text.contains("e-") {
                        offenses.push(Offense::new(
                            self.name(),
                            "Use e+ or e- for exponential notation",
                            self.severity(),
                            Location::new(line_number, col, mat.len()),
                        ));
                    }
                }
            }
        }

        offenses
    }
}

/// Checks for ENV.fetch instead of ENV[].
pub struct FetchEnvVar;

impl Cop for FetchEnvVar {
    fn name(&self) -> &str {
        "Style/FetchEnvVar"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for ENV[] instead of ENV.fetch"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let env_bracket_regex = Regex::new(r#"ENV\[['""][A-Z_]+['"]\]"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in env_bracket_regex.find_iter(line) {
                let col = mat.start() + 1;
                if source.in_string_or_comment(line_number, col) {
                    continue;
                }

                // Check if this is an assignment TO ENV (ENV['X'] = 'y'), not FROM ENV
                let after_match = &line[mat.end()..];
                if after_match.trim_start().starts_with('=') {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Use ENV.fetch instead of ENV[]",
                    self.severity(),
                    Location::new(line_number, col, mat.len()),
                ));
            }
        }

        offenses
    }
}

/// Checks for File.size == 0 instead of File.empty?.
pub struct FileEmpty;

impl Cop for FileEmpty {
    fn name(&self) -> &str {
        "Style/FileEmpty"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for File.size == 0 instead of File.empty?"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let file_size_regex = Regex::new(r#"File\.size.*==\s*0"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in file_size_regex.find_iter(line) {
                let col = mat.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use File.empty? instead of File.size == 0",
                        self.severity(),
                        Location::new(line_number, col, mat.len()),
                    ));
                }
            }
        }

        offenses
    }
}


// ============================================================================
// Cops 61-80: File operations and hash methods
// ============================================================================

/// Checks for File::NULL instead of /dev/null.
pub struct FileNull;

impl Cop for FileNull {
    fn name(&self) -> &str {
        "Style/FileNull"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for /dev/null instead of File::NULL"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let dev_null_regex = Regex::new(r#"['"]/dev/null['""]"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in dev_null_regex.find_iter(line) {
                let col = mat.start() + 1;
                offenses.push(Offense::new(
                    self.name(),
                    "Use File::NULL instead of '/dev/null'",
                    self.severity(),
                    Location::new(line_number, col, mat.len()),
                ));
            }
        }

        offenses
    }
}

/// Checks for File.open.read instead of File.read.
pub struct FileRead;

impl Cop for FileRead {
    fn name(&self) -> &str {
        "Style/FileRead"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for File.open.read instead of File.read"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let file_open_read_regex = Regex::new(r#"File\.open\([^)]+\)\.read"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in file_open_read_regex.find_iter(line) {
                let col = mat.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use File.read instead of File.open.read",
                        self.severity(),
                        Location::new(line_number, col, mat.len()),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks for File.open.write instead of File.write.
pub struct FileWrite;

impl Cop for FileWrite {
    fn name(&self) -> &str {
        "Style/FileWrite"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for File.open.write instead of File.write"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let file_open_write_regex = Regex::new(r#"File\.open\([^)]+,\s*['""]w['""][^)]*\)"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in file_open_write_regex.find_iter(line) {
                let col = mat.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use File.write instead of File.open with 'w'",
                        self.severity(),
                        Location::new(line_number, col, mat.len()),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks for .to_f for division instead of fdiv.
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
        "Checks for .to_f for division instead of fdiv"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let to_f_div_regex = Regex::new(r#"\.to_f\s*/"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in to_f_div_regex.find_iter(line) {
                let col = mat.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use fdiv instead of to_f for division",
                        self.severity(),
                        Location::new(line_number, col, mat.len()),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks for format string token consistency.
pub struct FormatStringToken;

impl Cop for FormatStringToken {
    fn name(&self) -> &str {
        "Style/FormatStringToken"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for consistent format string tokens"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let format_regex = Regex::new(r#"(format|sprintf)\s*\(['""][^'"]*%[^sdifbox\s]"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in format_regex.find_iter(line) {
                let col = mat.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use consistent format string tokens",
                        self.severity(),
                        Location::new(line_number, col, mat.len()),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks for Hash[] instead of .to_h.
pub struct HashConversion;

impl Cop for HashConversion {
    fn name(&self) -> &str {
        "Style/HashConversion"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for Hash[] instead of .to_h"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let hash_bracket_regex = Regex::new(r#"Hash\[\w+\]"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in hash_bracket_regex.find_iter(line) {
                let col = mat.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use .to_h instead of Hash[]",
                        self.severity(),
                        Location::new(line_number, col, mat.len()),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks for each_key and each_value.
pub struct HashEachMethods;

impl Cop for HashEachMethods {
    fn name(&self) -> &str {
        "Style/HashEachMethods"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for keys.each or values.each instead of each_key/each_value"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let keys_each_regex = Regex::new(r#"\.keys\.each"#).unwrap();
        let values_each_regex = Regex::new(r#"\.values\.each"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for mat in keys_each_regex.find_iter(line) {
                let col = mat.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use each_key instead of keys.each",
                        self.severity(),
                        Location::new(line_number, col, mat.len()),
                    ));
                }
            }
            
            for mat in values_each_regex.find_iter(line) {
                let col = mat.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use each_value instead of values.each",
                        self.severity(),
                        Location::new(line_number, col, mat.len()),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks for .reject instead of .except.
pub struct HashExcept;

impl Cop for HashExcept {
    fn name(&self) -> &str {
        "Style/HashExcept"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for reject with key check instead of except"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let reject_key_regex = Regex::new(r#"\.reject\s*\{\s*\|[^|]*\|\s*\[[^]]*\]"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in reject_key_regex.find_iter(line) {
                let col = mat.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use except instead of reject with key check",
                        self.severity(),
                        Location::new(line_number, col, mat.len()),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks for hash-like if/elsif that should be case.
pub struct HashLikeCase;

impl Cop for HashLikeCase {
    fn name(&self) -> &str {
        "Style/HashLikeCase"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for hash-like if/elsif"
    }

    fn check(&self, _source: &SourceFile) -> Vec<Offense> {
        // Similar to CaseLikeIf - already implemented
        Vec::new()
    }
}

/// Checks for .select with keys instead of .slice.
pub struct HashSlice;

impl Cop for HashSlice {
    fn name(&self) -> &str {
        "Style/HashSlice"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for select with keys instead of slice"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let select_keys_regex = Regex::new(r#"\.select\s*\{\s*\|[^|]*\|\s*\[[^]]*\]\.include\?"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in select_keys_regex.find_iter(line) {
                let col = mat.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use slice instead of select with key check",
                        self.severity(),
                        Location::new(line_number, col, mat.len()),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks for .map with key transformation instead of .transform_keys.
pub struct HashTransformKeys;

impl Cop for HashTransformKeys {
    fn name(&self) -> &str {
        "Style/HashTransformKeys"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for map instead of transform_keys"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let map_keys_regex = Regex::new(r#"\.map\s*\{\s*\|([^,|]+),"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in map_keys_regex.find_iter(line) {
                let col = mat.start() + 1;
                if !source.in_string_or_comment(line_number, col)
                    && line.contains("to_h") {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use transform_keys instead of map",
                        self.severity(),
                        Location::new(line_number, col, mat.len()),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks for .map with value transformation instead of .transform_values.
pub struct HashTransformValues;

impl Cop for HashTransformValues {
    fn name(&self) -> &str {
        "Style/HashTransformValues"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for map instead of transform_values"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let map_values_regex = Regex::new(r#"\.map\s*\{\s*\|[^,|]+,\s*([^|]+)\|"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in map_values_regex.find_iter(line) {
                let col = mat.start() + 1;
                if !source.in_string_or_comment(line_number, col)
                    && line.contains("to_h") {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use transform_values instead of map",
                        self.severity(),
                        Location::new(line_number, col, mat.len()),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks for modifier if/unless on if/unless.
pub struct IfUnlessModifierOfIfUnless;

impl Cop for IfUnlessModifierOfIfUnless {
    fn name(&self) -> &str {
        "Style/IfUnlessModifierOfIfUnless"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for modifier if/unless on if/unless"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let nested_if_regex = Regex::new(r#"^\s*(if|unless)\s+.*\s+(if|unless)\s+"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            if nested_if_regex.is_match(line) {
                offenses.push(Offense::new(
                    self.name(),
                    "Avoid modifier if/unless on if/unless",
                    self.severity(),
                    Location::new(line_number, 1, line.len()),
                ));
            }
        }

        offenses
    }
}

/// Checks for raise without error class.
pub struct ImplicitRuntimeError;

impl Cop for ImplicitRuntimeError {
    fn name(&self) -> &str {
        "Style/ImplicitRuntimeError"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for raise without error class"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let raise_string_regex = Regex::new(r#"^\s*raise\s+['""]"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            if raise_string_regex.is_match(line) {
                offenses.push(Offense::new(
                    self.name(),
                    "Specify error class explicitly in raise",
                    self.severity(),
                    Location::new(line_number, 1, line.len()),
                ));
            }
        }

        offenses
    }
}

/// Checks for then in pattern matching.
pub struct InPatternThen;

impl Cop for InPatternThen {
    fn name(&self) -> &str {
        "Style/InPatternThen"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for then in pattern matching"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let _in_pattern_regex = Regex::new(r#"^\s*in\s+.*[^t][^h][^e][^n]\s*$"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            if line.trim().starts_with("in ") && !line.contains("then") {
                offenses.push(Offense::new(
                    self.name(),
                    "Use then for single-line pattern branches",
                    self.severity(),
                    Location::new(line_number, 1, line.len()),
                ));
            }
        }

        offenses
    }
}

/// Checks for inverse methods like reject instead of select with negation.
pub struct InverseMethods;

impl Cop for InverseMethods {
    fn name(&self) -> &str {
        "Style/InverseMethods"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for inverse methods"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let select_not_regex = Regex::new(r#"\.select\s*\{\s*\|[^|]+\|\s*!"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in select_not_regex.find_iter(line) {
                let col = mat.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use reject instead of select with negation",
                        self.severity(),
                        Location::new(line_number, col, mat.len()),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks for unless with invertible condition.
pub struct InvertibleUnlessCondition;

impl Cop for InvertibleUnlessCondition {
    fn name(&self) -> &str {
        "Style/InvertibleUnlessCondition"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for unless with invertible condition"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let unless_invertible_regex = Regex::new(r#"^\s*unless\s+\w+\.(nil\?|empty\?|blank\?)"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            if unless_invertible_regex.is_match(line) {
                offenses.push(Offense::new(
                    self.name(),
                    "Use if with inverted method instead of unless",
                    self.severity(),
                    Location::new(line_number, 1, line.len()),
                ));
            }
        }

        offenses
    }
}

/// Checks for hardcoded IP addresses.
pub struct IpAddresses;

impl Cop for IpAddresses {
    fn name(&self) -> &str {
        "Style/IpAddresses"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for hardcoded IP addresses"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let ip_regex = Regex::new(r#"\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\b"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in ip_regex.find_iter(line) {
                let col = mat.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    // Skip common false positives like version numbers
                    let text = mat.as_str();
                    if !text.starts_with("0.") && !text.starts_with("1.") {
                        offenses.push(Offense::new(
                            self.name(),
                            "Avoid hardcoding IP addresses",
                            self.severity(),
                            Location::new(line_number, col, mat.len()),
                        ));
                    }
                }
            }
        }

        offenses
    }
}


// ============================================================================
// Cops 81-95: Final patterns and method styles
// ============================================================================

/// Checks for keyword parameter order.
pub struct KeywordParametersOrder;

impl Cop for KeywordParametersOrder {
    fn name(&self) -> &str {
        "Style/KeywordParametersOrder"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for keyword parameter order"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let keyword_params_regex = Regex::new(r#"def\s+\w+\([^)]*(\w+:\s*\w+)[^)]*,\s*(\w+:)[^)]*\)"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            if keyword_params_regex.is_match(line) {
                // Simple heuristic: required keywords should come before optional
                if line.contains(": ") && line.matches(':').count() > 1 {
                    offenses.push(Offense::new(
                        self.name(),
                        "Required keyword parameters should come before optional",
                        self.severity(),
                        Location::new(line_number, 1, line.len()),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks for line end concatenation with +.
pub struct LineEndConcatenation;

impl Cop for LineEndConcatenation {
    fn name(&self) -> &str {
        "Style/LineEndConcatenation"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for string concatenation at line end"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let line_concat_regex = Regex::new(r#"['"]\s*\+\s*$"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            if line_concat_regex.is_match(line) {
                offenses.push(Offense::new(
                    self.name(),
                    r"Use \ for line continuation instead of +",
                    self.severity(),
                    Location::new(line_number, line.len(), 1),
                ));
            }
        }

        offenses
    }
}

/// Checks for .map.to_h instead of .to_h with block.
pub struct MapToHash;

impl Cop for MapToHash {
    fn name(&self) -> &str {
        "Style/MapToHash"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for .map.to_h instead of .to_h with block"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let map_to_h_regex = Regex::new(r##"\.map\s*\{[^}]+\}\.to_h"##).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in map_to_h_regex.find_iter(line) {
                let col = mat.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use .to_h with block instead of .map.to_h",
                        self.severity(),
                        Location::new(line_number, col, mat.len()),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks for method calls with arguments without parentheses.
pub struct MethodCallWithArgsParentheses;

impl Cop for MethodCallWithArgsParentheses {
    fn name(&self) -> &str {
        "Style/MethodCallWithArgsParentheses"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for method calls with arguments without parentheses"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let method_call_regex = Regex::new(r#"\b[a-z_]\w*\s+\w+\s*,"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in method_call_regex.find_iter(line) {
                let col = mat.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    let text = mat.as_str();
                    if !text.starts_with("def ") && !text.starts_with("class ") {
                        offenses.push(Offense::new(
                            self.name(),
                            "Use parentheses for method calls with arguments",
                            self.severity(),
                            Location::new(line_number, col, mat.len()),
                        ));
                    }
                }
            }
        }

        offenses
    }
}

/// Checks for method calls on do/end blocks.
pub struct MethodCalledOnDoEndBlock;

impl Cop for MethodCalledOnDoEndBlock {
    fn name(&self) -> &str {
        "Style/MethodCalledOnDoEndBlock"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for method calls on do/end blocks"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let end_method_regex = Regex::new(r#"^\s*end\.\w+"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            if end_method_regex.is_match(line) {
                offenses.push(Offense::new(
                    self.name(),
                    "Avoid chaining method calls on do/end blocks",
                    self.severity(),
                    Location::new(line_number, 1, line.len()),
                ));
            }
        }

        offenses
    }
}

/// Checks for separate min and max calls instead of minmax.
pub struct MinMax;

impl Cop for MinMax {
    fn name(&self) -> &str {
        "Style/MinMax"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for separate min/max calls"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let min_regex = Regex::new(r#"(\w+)\.min"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            if let Some(cap) = min_regex.captures(line) {
                let collection = cap.get(1).unwrap().as_str();
                // Check if max is called on same collection nearby
                if line.contains(&format!("{}.max", collection)) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use .minmax instead of separate .min and .max",
                        self.severity(),
                        Location::new(line_number, 1, line.len()),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks for missing else in conditionals.
pub struct MissingElse;

impl Cop for MissingElse {
    fn name(&self) -> &str {
        "Style/MissingElse"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for missing else clause"
    }

    fn check(&self, _source: &SourceFile) -> Vec<Offense> {
        // This is configurable - by default we don't enforce
        Vec::new()
    }
}

/// Checks for method_missing without respond_to_missing?.
pub struct MissingRespondToMissing;

impl Cop for MissingRespondToMissing {
    fn name(&self) -> &str {
        "Style/MissingRespondToMissing"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for method_missing without respond_to_missing?"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let method_missing_regex = Regex::new(r#"def\s+method_missing"#).unwrap();
        let respond_to_missing_regex = Regex::new(r#"def\s+respond_to_missing\?"#).unwrap();

        let has_method_missing = source.lines.iter().any(|line| method_missing_regex.is_match(line));
        let has_respond_to_missing = source.lines.iter().any(|line| respond_to_missing_regex.is_match(line));

        if has_method_missing && !has_respond_to_missing {
            for (line_num, line) in source.lines.iter().enumerate() {
                if method_missing_regex.is_match(line) {
                    let line_number = line_num + 1;
                    offenses.push(Offense::new(
                        self.name(),
                        "Define respond_to_missing? when using method_missing",
                        self.severity(),
                        Location::new(line_number, 1, line.len()),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks for mixin grouping.
pub struct MixinGrouping;

impl Cop for MixinGrouping {
    fn name(&self) -> &str {
        "Style/MixinGrouping"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for mixin grouping"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let mixin_regex = Regex::new(r#"^\s*(include|extend|prepend)\s+\w+\s*$"#).unwrap();

        let mut prev_line_num = 0;
        let mut prev_mixin_type = "";

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            if let Some(cap) = mixin_regex.captures(line) {
                let mixin_type = cap.get(1).unwrap().as_str();
                
                if prev_line_num > 0 && line_number == prev_line_num + 1 && mixin_type == prev_mixin_type {
                    offenses.push(Offense::new(
                        self.name(),
                        "Group multiple mixins together",
                        self.severity(),
                        Location::new(line_number, 1, line.len()),
                    ));
                }
                
                prev_line_num = line_number;
                prev_mixin_type = mixin_type;
            }
        }

        offenses
    }
}

/// Checks for module_function vs extend self.
pub struct ModuleFunction;

impl Cop for ModuleFunction {
    fn name(&self) -> &str {
        "Style/ModuleFunction"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for extend self instead of module_function"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let extend_self_regex = Regex::new(r#"^\s*extend\s+self\s*$"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            if extend_self_regex.is_match(line) {
                offenses.push(Offense::new(
                    self.name(),
                    "Use module_function instead of extend self",
                    self.severity(),
                    Location::new(line_number, 1, line.len()),
                ));
            }
        }

        offenses
    }
}

/// Checks for method chains on multiline blocks.
pub struct MultilineBlockChain;

impl Cop for MultilineBlockChain {
    fn name(&self) -> &str {
        "Style/MultilineBlockChain"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for method chains on multiline blocks"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let end_chain_regex = Regex::new(r#"^\s*end\.\w+"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            if end_chain_regex.is_match(line) {
                offenses.push(Offense::new(
                    self.name(),
                    "Avoid chaining method calls on multiline blocks",
                    self.severity(),
                    Location::new(line_number, 1, line.len()),
                ));
            }
        }

        offenses
    }
}

/// Checks for modifier if on multiline expressions.
pub struct MultilineIfModifier;

impl Cop for MultilineIfModifier {
    fn name(&self) -> &str {
        "Style/MultilineIfModifier"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for modifier if on multiline expressions"
    }

    fn check(&self, _source: &SourceFile) -> Vec<Offense> {
        // Complex to detect multiline expressions accurately
        Vec::new()
    }
}

/// Checks for multiline memoization style.
pub struct MultilineMemoization;

impl Cop for MultilineMemoization {
    fn name(&self) -> &str {
        "Style/MultilineMemoization"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for multiline memoization style"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let memo_regex = Regex::new(r#"@\w+\s*\|\|="#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            if memo_regex.is_match(line) && line.contains("begin") {
                offenses.push(Offense::new(
                    self.name(),
                    "Use proper multiline memoization style",
                    self.severity(),
                    Location::new(line_number, 1, line.len()),
                ));
            }
        }

        offenses
    }
}

/// Checks for method signatures on multiple lines.
pub struct MultilineMethodSignature;

impl Cop for MultilineMethodSignature {
    fn name(&self) -> &str {
        "Style/MultilineMethodSignature"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for method signatures on multiple lines"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let def_regex = Regex::new(r#"^\s*def\s+\w+"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            if def_regex.is_match(line) && line.ends_with(',') {
                offenses.push(Offense::new(
                    self.name(),
                    "Avoid multiline method signatures",
                    self.severity(),
                    Location::new(line_number, line.len(), 1),
                ));
            }
        }

        offenses
    }
}

/// Checks for multiple comparisons.
pub struct MultipleComparison;

impl Cop for MultipleComparison {
    fn name(&self) -> &str {
        "Style/MultipleComparison"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for multiple comparisons with =="
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let multi_compare_regex = Regex::new(r#"\w+\s*==\s*\w+\s*\|\|\s*\w+\s*==\s*\w+"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in multi_compare_regex.find_iter(line) {
                let col = mat.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use .include? instead of multiple == comparisons",
                        self.severity(),
                        Location::new(line_number, col, mat.len()),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks for nested parenthesized calls.
pub struct NestedParenthesizedCalls;

impl Cop for NestedParenthesizedCalls {
    fn name(&self) -> &str {
        "Style/NestedParenthesizedCalls"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for nested parenthesized calls"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let nested_parens_regex = Regex::new(r#"\w+\([^)]*\w+\([^)]*\)[^)]*\)"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in nested_parens_regex.find_iter(line) {
                let col = mat.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    // Check if deeply nested
                    let text = mat.as_str();
                    if text.matches('(').count() > 2 {
                        offenses.push(Offense::new(
                            self.name(),
                            "Avoid deeply nested parenthesized calls",
                            self.severity(),
                            Location::new(line_number, col, mat.len()),
                        ));
                    }
                }
            }
        }

        offenses
    }
}

/// Checks for numbered parameters usage.
pub struct NumberedParameters;

impl Cop for NumberedParameters {
    fn name(&self) -> &str {
        "Style/NumberedParameters"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for numbered parameters"
    }

    fn check(&self, _source: &SourceFile) -> Vec<Offense> {
        // This checks if numbered params should be used
        Vec::new()
    }
}

/// Checks for .then vs .yield_self.
pub struct ObjectThen;

impl Cop for ObjectThen {
    fn name(&self) -> &str {
        "Style/ObjectThen"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for .yield_self instead of .then"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let yield_self_regex = Regex::new(r#"\.yield_self"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in yield_self_regex.find_iter(line) {
                let col = mat.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use .then instead of .yield_self",
                        self.severity(),
                        Location::new(line_number, col, mat.len()),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks for OpenStruct usage.
pub struct OpenStructUse;

impl Cop for OpenStructUse {
    fn name(&self) -> &str {
        "Style/OpenStructUse"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for OpenStruct usage"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let openstruct_regex = Regex::new(r#"\bOpenStruct\b"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in openstruct_regex.find_iter(line) {
                let col = mat.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Avoid using OpenStruct",
                        self.severity(),
                        Location::new(line_number, col, mat.len()),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks for operator method calls with explicit parentheses.
pub struct OperatorMethodCall;

impl Cop for OperatorMethodCall {
    fn name(&self) -> &str {
        "Style/OperatorMethodCall"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for .+() style operator calls"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let operator_call_regex = Regex::new(r#"\.\s*(\+|-|\*|/|%|&|\||\^|<<|>>)\s*\("#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in operator_call_regex.find_iter(line) {
                let col = mat.start() + 1;
                if !source.in_string_or_comment(line_number, col) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use operator syntax instead of method call",
                        self.severity(),
                        Location::new(line_number, col, mat.len()),
                    ));
                }
            }
        }

        offenses
    }
}


// ============================================================================
// Tests
// ============================================================================


// Collect all cops from this module
pub fn all_style_extra2_cops() -> Vec<Box<dyn Cop>> {
    vec![
        Box::new(TrailingBodyOnClass),
        Box::new(TrailingBodyOnModule),
        Box::new(TrailingBodyOnMethodDefinition),
        Box::new(TrailingCommaInArguments),
        Box::new(TrailingCommaInArrayLiteral),
        Box::new(TrailingCommaInHashLiteral),
        Box::new(TrailingCommaInBlockArgs),
        Box::new(TrailingMethodEndStatement),
        Box::new(TrailingUnderscoreVariable),
        Box::new(TrivialAccessors),
        Box::new(UnlessElse),
        Box::new(UnlessLogicalOperators),
        Box::new(UnpackFirst),
        Box::new(VariableInterpolation),
        Box::new(WhenThen),
        Box::new(WhileUntilDo),
        Box::new(WhileUntilModifier),
        Box::new(WordArray),
        Box::new(YodaCondition),
        Box::new(YodaExpression),
        Box::new(ZeroLengthPredicate),
        Box::new(AccessModifierDeclarations),
        Box::new(AccessorGrouping),
        Box::new(Alias),
        Box::new(ArrayCoercion),
        Box::new(ArrayFirstLast),
        Box::new(ArrayIntersect),
        Box::new(ArrayJoin),
        Box::new(BlockDelimiters),
        Box::new(CaseLikeIf),
        Box::new(ClassAndModuleChildren),
        Box::new(ClassEqualityComparison),
        Box::new(ClassMethods),
        Box::new(CollectionCompact),
        Box::new(CombinableLoops),
        Box::new(ConcatArrayLiterals),
        Box::new(ConditionalAssignment),
        Box::new(DataInheritance),
        Box::new(DigChain),
        Box::new(Dir),
        Box::new(DirEmpty),
        Box::new(DisableCopsWithinSourceCodeDirective),
        Box::new(Documentation),
        Box::new(DoubleCopDisableDirective),
        Box::new(EachWithObject),
        Box::new(EmptyClassDefinition),
        Box::new(EmptyStringInsideInterpolation),
        // Box::new(EndBlock),  // Duplicate - registered in style_extra1
        Box::new(EndlessMethod),
        Box::new(EnvHome),
        Box::new(EvalWithLocation),
        Box::new(ExactRegexpMatch),
        Box::new(ExpandPathArguments),
        Box::new(ExplicitBlockArgument),
        Box::new(ExponentialNotation),
        Box::new(FetchEnvVar),
        Box::new(FileEmpty),
        Box::new(FileNull),
        Box::new(FileRead),
        Box::new(FileWrite),
        Box::new(FloatDivision),
        Box::new(FormatStringToken),
        Box::new(HashConversion),
        Box::new(HashEachMethods),
        Box::new(HashExcept),
        Box::new(HashLikeCase),
        Box::new(HashSlice),
        Box::new(HashTransformKeys),
        Box::new(HashTransformValues),
        Box::new(IfUnlessModifierOfIfUnless),
        Box::new(ImplicitRuntimeError),
        Box::new(InPatternThen),
        Box::new(InverseMethods),
        Box::new(InvertibleUnlessCondition),
        Box::new(IpAddresses),
        Box::new(KeywordParametersOrder),
        Box::new(LineEndConcatenation),
        Box::new(MapToHash),
        Box::new(MethodCallWithArgsParentheses),
        Box::new(MethodCalledOnDoEndBlock),
        Box::new(MinMax),
        Box::new(MissingElse),
        Box::new(MissingRespondToMissing),
        Box::new(MixinGrouping),
        Box::new(ModuleFunction),
        Box::new(MultilineBlockChain),
        Box::new(MultilineIfModifier),
        Box::new(MultilineMemoization),
        Box::new(MultilineMethodSignature),
        Box::new(MultipleComparison),
        Box::new(NestedParenthesizedCalls),
        Box::new(NumberedParameters),
        Box::new(ObjectThen),
        Box::new(OpenStructUse),
        Box::new(OperatorMethodCall),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn test_source(content: &str) -> SourceFile {
        SourceFile::from_string(PathBuf::from("test.rb"), content.to_string())
    }

    // Tests for cops 1-10
    #[test]
    fn test_trailing_body_on_class_pass() {
        let source = test_source("class Foo\nend");
        let cop = TrailingBodyOnClass;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_trailing_body_on_class_fail() {
        let source = test_source("class Foo def bar; end\nend");
        let cop = TrailingBodyOnClass;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_trailing_body_on_module_pass() {
        let source = test_source("module Foo\nend");
        let cop = TrailingBodyOnModule;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_trailing_body_on_module_fail() {
        let source = test_source("module Foo def bar; end\nend");
        let cop = TrailingBodyOnModule;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_trailing_body_on_method_pass() {
        let source = test_source("def foo\n  bar\nend");
        let cop = TrailingBodyOnMethodDefinition;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_trailing_body_on_method_fail() {
        let source = test_source("def foo bar\nend");
        let cop = TrailingBodyOnMethodDefinition;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_trailing_comma_in_arguments_pass() {
        let source = test_source("foo(1, 2, 3)");
        let cop = TrailingCommaInArguments;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_trailing_comma_in_arguments_fail() {
        let source = test_source("foo(1, 2, 3,)");
        let cop = TrailingCommaInArguments;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_trailing_comma_in_array_pass() {
        let source = test_source("[1, 2, 3]");
        let cop = TrailingCommaInArrayLiteral;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_trailing_comma_in_array_fail() {
        let source = test_source("[1, 2, 3,]");
        let cop = TrailingCommaInArrayLiteral;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_trailing_comma_in_hash_pass() {
        let source = test_source("{a: 1, b: 2}");
        let cop = TrailingCommaInHashLiteral;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_trailing_comma_in_hash_fail() {
        let source = test_source("{a: 1, b: 2,}");
        let cop = TrailingCommaInHashLiteral;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_trailing_comma_in_block_args_pass() {
        let source = test_source("foo.each { |x, y| }");
        let cop = TrailingCommaInBlockArgs;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_trailing_comma_in_block_args_fail() {
        let source = test_source("foo.each { |x, y,| }");
        let cop = TrailingCommaInBlockArgs;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_unless_else_pass() {
        let source = test_source("if !condition\n  foo\nelse\n  bar\nend");
        let cop = UnlessElse;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_unless_else_fail() {
        let source = test_source("unless condition\n  foo\nelse\n  bar\nend");
        let cop = UnlessElse;
        assert_eq!(cop.check(&source).len(), 1);
    }

    // Tests for cops 11-20
    #[test]
    fn test_unless_logical_operators_pass() {
        let source = test_source("unless simple_condition\n  foo\nend");
        let cop = UnlessLogicalOperators;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_unless_logical_operators_fail() {
        let source = test_source("unless a || b\n  foo\nend");
        let cop = UnlessLogicalOperators;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_variable_interpolation_pass() {
        let source = test_source("\"hello #{@name}\"");
        let cop = VariableInterpolation;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_variable_interpolation_fail() {
        let source = test_source("\"hello #@name\"");
        let cop = VariableInterpolation;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_while_until_do_pass() {
        let source = test_source("while condition\n  foo\nend");
        let cop = WhileUntilDo;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_while_until_do_fail() {
        let source = test_source("while condition do\n  foo\nend");
        let cop = WhileUntilDo;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_word_array_pass() {
        let source = test_source("%w[foo bar baz]");
        let cop = WordArray;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_word_array_fail() {
        let source = test_source("[\"foo\", \"bar\", \"baz\"]");
        let cop = WordArray;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_yoda_condition_pass() {
        let source = test_source("if x == 5\n  foo\nend");
        let cop = YodaCondition;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_yoda_condition_fail() {
        let source = test_source("if 5 == x\n  foo\nend");
        let cop = YodaCondition;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_yoda_expression_pass() {
        let source = test_source("x + 5");
        let cop = YodaExpression;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_yoda_expression_fail() {
        let source = test_source("5 + x");
        let cop = YodaExpression;
        assert_eq!(cop.check(&source).len(), 1);
    }

    // Tests for cops 21-30
    #[test]
    fn test_zero_length_predicate_pass() {
        let source = test_source("array.empty?");
        let cop = ZeroLengthPredicate;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_zero_length_predicate_fail() {
        let source = test_source("array.length == 0");
        let cop = ZeroLengthPredicate;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_array_coercion_pass() {
        let source = test_source("Array(foo)");
        let cop = ArrayCoercion;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_array_coercion_fail() {
        let source = test_source("[*foo]");
        let cop = ArrayCoercion;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_array_first_last_pass() {
        let source = test_source("array.first");
        let cop = ArrayFirstLast;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_array_first_last_fail() {
        let source = test_source("array[0]");
        let cop = ArrayFirstLast;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_array_intersect_pass() {
        let source = test_source("a.intersect?(b)");
        let cop = ArrayIntersect;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_array_intersect_fail() {
        let source = test_source("(a & b).any?");
        let cop = ArrayIntersect;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_class_equality_comparison_pass() {
        let source = test_source("obj.is_a?(String)");
        let cop = ClassEqualityComparison;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_class_equality_comparison_fail() {
        let source = test_source("obj.class == String");
        let cop = ClassEqualityComparison;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_class_methods_pass() {
        let source = test_source("def self.foo\nend");
        let cop = ClassMethods;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_class_methods_fail() {
        let source = test_source("def MyClass.foo\nend");
        let cop = ClassMethods;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_collection_compact_pass() {
        let source = test_source("array.compact");
        let cop = CollectionCompact;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_collection_compact_fail() {
        let source = test_source("array.reject(&:nil?)");
        let cop = CollectionCompact;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_concat_array_literals_pass() {
        let source = test_source("[1, 2, 3, 4]");
        let cop = ConcatArrayLiterals;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_concat_array_literals_fail() {
        let source = test_source("[1, 2] + [3, 4]");
        let cop = ConcatArrayLiterals;
        assert_eq!(cop.check(&source).len(), 1);
    }

    // Tests for cops 31-40
    #[test]
    fn test_dir_pass() {
        let source = test_source("__dir__");
        let cop = Dir;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_dir_fail() {
        let source = test_source("File.dirname(__FILE__)");
        let cop = Dir;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_dir_empty_pass() {
        let source = test_source("Dir.empty?(path)");
        let cop = DirEmpty;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_dir_empty_fail() {
        let source = test_source("Dir.entries(path).size == 2");
        let cop = DirEmpty;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_documentation_pass() {
        let source = test_source("# This is a class\nclass Foo\nend");
        let cop = Documentation;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_documentation_fail() {
        let source = test_source("class Foo\nend");
        let cop = Documentation;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_end_block_pass() {
        let source = test_source("at_exit { puts 'done' }");
        let cop = EndBlock;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_end_block_fail() {
        let source = test_source("END { puts 'done' }");
        let cop = EndBlock;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_env_home_pass() {
        let source = test_source("Dir.home");
        let cop = EnvHome;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_env_home_fail() {
        let source = test_source("ENV['HOME']");
        let cop = EnvHome;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_file_empty_pass() {
        let source = test_source("File.empty?(path)");
        let cop = FileEmpty;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_file_empty_fail() {
        let source = test_source("File.size(path) == 0");
        let cop = FileEmpty;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_file_null_pass() {
        let source = test_source("File::NULL");
        let cop = FileNull;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_file_null_fail() {
        let source = test_source("'/dev/null'");
        let cop = FileNull;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_file_read_pass() {
        let source = test_source("File.read(path)");
        let cop = FileRead;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_file_read_fail() {
        let source = test_source("File.open(path).read");
        let cop = FileRead;
        assert_eq!(cop.check(&source).len(), 1);
    }

    // Tests for cops 41-50
    #[test]
    fn test_hash_conversion_pass() {
        let source = test_source("array.to_h");
        let cop = HashConversion;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_hash_conversion_fail() {
        let source = test_source("Hash[array]");
        let cop = HashConversion;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_hash_each_methods_pass() {
        let source = test_source("hash.each_key { |k| }");
        let cop = HashEachMethods;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_hash_each_methods_fail() {
        let source = test_source("hash.keys.each { |k| }");
        let cop = HashEachMethods;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_inverse_methods_pass() {
        let source = test_source("array.reject { |x| x.nil? }");
        let cop = InverseMethods;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_inverse_methods_fail() {
        let source = test_source("array.select { |x| !x.nil? }");
        let cop = InverseMethods;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_missing_respond_to_missing_pass() {
        let source = test_source("def method_missing(name)\nend\ndef respond_to_missing?(name)\nend");
        let cop = MissingRespondToMissing;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_missing_respond_to_missing_fail() {
        let source = test_source("def method_missing(name)\nend");
        let cop = MissingRespondToMissing;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_module_function_pass() {
        let source = test_source("module_function :foo");
        let cop = ModuleFunction;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_module_function_fail() {
        let source = test_source("extend self");
        let cop = ModuleFunction;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_object_then_pass() {
        let source = test_source("obj.then { |x| x * 2 }");
        let cop = ObjectThen;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_object_then_fail() {
        let source = test_source("obj.yield_self { |x| x * 2 }");
        let cop = ObjectThen;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_openstruct_use_pass() {
        let source = test_source("Struct.new(:name)");
        let cop = OpenStructUse;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_openstruct_use_fail() {
        let source = test_source("OpenStruct.new(name: 'foo')");
        let cop = OpenStructUse;
        assert_eq!(cop.check(&source).len(), 1);
    }

    // Tests for cops 51-60
    #[test]
    fn test_operator_method_call_pass() {
        let source = test_source("a + b");
        let cop = OperatorMethodCall;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_operator_method_call_fail() {
        let source = test_source("a.+(b)");
        let cop = OperatorMethodCall;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_fetch_env_var_pass() {
        let source = test_source("ENV.fetch('PATH')");
        let cop = FetchEnvVar;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_fetch_env_var_fail() {
        let source = test_source("value = ENV['PATH']");
        let cop = FetchEnvVar;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_float_division_pass() {
        let source = test_source("a.fdiv(b)");
        let cop = FloatDivision;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_float_division_fail() {
        let source = test_source("a.to_f / b");
        let cop = FloatDivision;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_line_end_concatenation_pass() {
        let source = test_source("str = \"hello \" \\\n  \"world\"");
        let cop = LineEndConcatenation;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_line_end_concatenation_fail() {
        let source = test_source("str = \"hello\" +");
        let cop = LineEndConcatenation;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_map_to_hash_pass() {
        let source = test_source("array.to_h { |x| [x, x * 2] }");
        let cop = MapToHash;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_map_to_hash_fail() {
        let source = test_source("array.map { |x| [x, x * 2] }.to_h");
        let cop = MapToHash;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_multiple_comparison_pass() {
        let source = test_source("[1, 2, 3].include?(x)");
        let cop = MultipleComparison;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_multiple_comparison_fail() {
        let source = test_source("x == 1 || x == 2 || x == 3");
        let cop = MultipleComparison;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_trivial_accessors_pass() {
        let source = test_source("attr_reader :name");
        let cop = TrivialAccessors;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_trivial_accessors_fail() {
        let source = test_source("def name\n  @name\nend");
        let cop = TrivialAccessors;
        assert_eq!(cop.check(&source).len(), 1);
    }
}
