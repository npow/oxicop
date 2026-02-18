//! Additional Style cops for Ruby code formatting and conventions.

use regex::Regex;

use crate::cop::{Category, Cop, Severity};
use crate::offense::{Location, Offense};
use crate::source::SourceFile;

// ============================================================================
// Cop 1: AndOr - use && / || instead of and / or
// ============================================================================

pub struct AndOr;

impl Cop for AndOr {
    fn name(&self) -> &str {
        "Style/AndOr"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use && and || instead of and and or"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let and_or_regex = Regex::new(r#"\b(and|or)\b"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for capture in and_or_regex.captures_iter(line) {
                if let Some(matched) = capture.get(1) {
                    let start_col = matched.start() + 1;
                    
                    if source.in_string_or_comment(line_number, start_col) {
                        continue;
                    }

                    let word = matched.as_str();
                    let replacement = if word == "and" { "&&" } else { "||" };
                    
                    offenses.push(Offense::new(
                        self.name(),
                        format!("Use `{}` instead of `{}`", replacement, word),
                        self.severity(),
                        Location::new(line_number, start_col, matched.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 2: AsciiComments - comments should only contain ASCII
// ============================================================================

pub struct AsciiComments;

impl Cop for AsciiComments {
    fn name(&self) -> &str {
        "Style/AsciiComments"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use only ASCII characters in comments"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            if let Some(comment_start) = line.find('#') {
                let comment = &line[comment_start..];

                if !comment.is_ascii() {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use only ASCII characters in comments",
                        self.severity(),
                        Location::new(line_number, comment_start + 1, comment.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 3: Attr - use attr_reader/attr_writer instead of attr
// ============================================================================

pub struct Attr;

impl Cop for Attr {
    fn name(&self) -> &str {
        "Style/Attr"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use attr_reader or attr_writer instead of attr"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let attr_regex = Regex::new(r#"\battr\s+:"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for capture in attr_regex.find_iter(line) {
                let start_col = capture.start() + 1;
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Use `attr_reader` or `attr_writer` instead of `attr`",
                    self.severity(),
                    Location::new(line_number, start_col, capture.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 4: BarePercentLiterals - use %q instead of bare %
// ============================================================================

pub struct BarePercentLiterals;

impl Cop for BarePercentLiterals {
    fn name(&self) -> &str {
        "Style/BarePercentLiterals"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use %q or %Q for percent literals instead of bare %"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let bare_percent_regex = Regex::new(r#"%[(\{\[<]"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for capture in bare_percent_regex.find_iter(line) {
                let start_col = capture.start() + 1;
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Use `%q` or `%Q` for percent string literals",
                    self.severity(),
                    Location::new(line_number, start_col, capture.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 5: BeginBlock - no BEGIN blocks
// ============================================================================

pub struct BeginBlock;

impl Cop for BeginBlock {
    fn name(&self) -> &str {
        "Style/BeginBlock"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Avoid using BEGIN blocks"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let begin_block_regex = Regex::new(r#"^\s*BEGIN\s*\{"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            if let Some(capture) = begin_block_regex.find(line) {
                let start_col = capture.start() + 1;
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Avoid using BEGIN blocks",
                    self.severity(),
                    Location::new(line_number, start_col, capture.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 6: BlockComments - use # not =begin / =end
// ============================================================================

pub struct BlockComments;

impl Cop for BlockComments {
    fn name(&self) -> &str {
        "Style/BlockComments"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use # for comments instead of =begin/=end"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            let trimmed = line.trim();
            
            if trimmed == "=begin" || trimmed == "=end" {
                offenses.push(Offense::new(
                    self.name(),
                    "Use `#` for comments instead of block comments",
                    self.severity(),
                    Location::new(line_number, 1, line.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 7: CaseEquality - don't use ===
// ============================================================================

pub struct CaseEquality;

impl Cop for CaseEquality {
    fn name(&self) -> &str {
        "Style/CaseEquality"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Avoid using the case equality operator ==="
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            if let Some(pos) = line.find("===") {
                let start_col = pos + 1;
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Avoid using the case equality operator `===`",
                    self.severity(),
                    Location::new(line_number, start_col, 3),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 8: CharacterLiteral - use string instead of ?c
// ============================================================================

pub struct CharacterLiteral;

impl Cop for CharacterLiteral {
    fn name(&self) -> &str {
        "Style/CharacterLiteral"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use string literal instead of character literal"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let char_literal_regex = Regex::new(r#"\?[a-zA-Z]"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for capture in char_literal_regex.find_iter(line) {
                let start_col = capture.start() + 1;
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Use string literal instead of character literal",
                    self.severity(),
                    Location::new(line_number, start_col, capture.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 9: ClassCheck - use is_a? not kind_of?
// ============================================================================

pub struct ClassCheck;

impl Cop for ClassCheck {
    fn name(&self) -> &str {
        "Style/ClassCheck"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use is_a? instead of kind_of?"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let kind_of_regex = Regex::new(r#"\.kind_of\?"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for capture in kind_of_regex.find_iter(line) {
                let start_col = capture.start() + 1;
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Use `is_a?` instead of `kind_of?`",
                    self.severity(),
                    Location::new(line_number, start_col, capture.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 10: ClassVars - don't use @@ class variables
// ============================================================================

pub struct ClassVars;

impl Cop for ClassVars {
    fn name(&self) -> &str {
        "Style/ClassVars"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Avoid using class variables (@@)"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let class_var_regex = Regex::new(r#"@@[a-zA-Z_]\w*"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for capture in class_var_regex.find_iter(line) {
                let start_col = capture.start() + 1;
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Avoid using class variables",
                    self.severity(),
                    Location::new(line_number, start_col, capture.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 11: ColonMethodCall - don't use :: for method calls
// ============================================================================

pub struct ColonMethodCall;

impl Cop for ColonMethodCall {
    fn name(&self) -> &str {
        "Style/ColonMethodCall"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use . instead of :: for method calls"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let colon_method_regex = Regex::new(r#"::[a-z_]\w*[?!]?\s*\("#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for capture in colon_method_regex.find_iter(line) {
                let start_col = capture.start() + 1;
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Use `.` instead of `::` for method calls",
                    self.severity(),
                    Location::new(line_number, start_col, 2),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 12: ColonMethodDefinition - use def self.method not def self::method
// ============================================================================

pub struct ColonMethodDefinition;

impl Cop for ColonMethodDefinition {
    fn name(&self) -> &str {
        "Style/ColonMethodDefinition"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use def self.method instead of def self::method"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let colon_def_regex = Regex::new(r#"\bdef\s+\w+::\w+"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            if let Some(capture) = colon_def_regex.find(line) {
                let start_col = capture.start() + 1;
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Use `def self.method` instead of `def self::method`",
                    self.severity(),
                    Location::new(line_number, start_col, capture.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 13: CommandLiteral - use backticks vs %x
// ============================================================================

pub struct CommandLiteral;

impl Cop for CommandLiteral {
    fn name(&self) -> &str {
        "Style/CommandLiteral"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use backticks for command literals instead of %x"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let command_literal_regex = Regex::new(r#"%x[{(\[<]"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for capture in command_literal_regex.find_iter(line) {
                let start_col = capture.start() + 1;
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Use backticks for command literals instead of `%x`",
                    self.severity(),
                    Location::new(line_number, start_col, capture.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 14: CommentAnnotation - comment annotations format (TODO, FIXME, etc.)
// ============================================================================

pub struct CommentAnnotation;

impl Cop for CommentAnnotation {
    fn name(&self) -> &str {
        "Style/CommentAnnotation"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Comment annotations should be formatted correctly"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let annotation_regex = Regex::new(r#"#\s*(TODO|FIXME|OPTIMIZE|HACK|REVIEW|NOTE)([^:]|\s*$)"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for capture in annotation_regex.captures_iter(line) {
                if let Some(matched) = capture.get(0) {
                    let start_col = matched.start() + 1;
                    
                    offenses.push(Offense::new(
                        self.name(),
                        "Annotation keywords should be followed by a colon and space".to_string(),
                        self.severity(),
                        Location::new(line_number, start_col, matched.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 15: CommentedKeyword - no comments on same line as keyword
// ============================================================================

pub struct CommentedKeyword;

impl Cop for CommentedKeyword {
    fn name(&self) -> &str {
        "Style/CommentedKeyword"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Do not place comments on the same line as keywords"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let keywords = ["end", "begin"];

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            if let Some(comment_pos) = line.find('#') {
                let before_comment = &line[..comment_pos].trim();
                
                for keyword in &keywords {
                    if before_comment == keyword {
                        offenses.push(Offense::new(
                            self.name(),
                            format!("Do not place comments on the same line as the `{}` keyword", keyword),
                            self.severity(),
                            Location::new(line_number, comment_pos + 1, line.len() - comment_pos),
                        ));
                        break;
                    }
                }
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 16: Copyright - copyright comment at top of file
// ============================================================================

pub struct Copyright;

impl Cop for Copyright {
    fn name(&self) -> &str {
        "Style/Copyright"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Include a copyright notice at the top of the file"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        
        if source.is_empty() {
            return offenses;
        }

        let mut has_copyright = false;
        
        for i in 0..5.min(source.line_count()) {
            if let Some(line) = source.line(i + 1) {
                if line.to_lowercase().contains("copyright") {
                    has_copyright = true;
                    break;
                }
            }
        }

        if !has_copyright {
            offenses.push(Offense::new(
                self.name(),
                "Include a copyright notice at the top of the file",
                self.severity(),
                Location::new(1, 1, 1),
            ));
        }

        offenses
    }
}

// ============================================================================
// Cop 17: DateTime - use Time not DateTime
// ============================================================================

pub struct DateTime;

impl Cop for DateTime {
    fn name(&self) -> &str {
        "Style/DateTime"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use Time instead of DateTime"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let datetime_regex = Regex::new(r#"\bDateTime\b"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for capture in datetime_regex.find_iter(line) {
                let start_col = capture.start() + 1;
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Use `Time` instead of `DateTime`",
                    self.severity(),
                    Location::new(line_number, start_col, capture.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 18: DefWithParentheses - use parens for method def with args
// ============================================================================

pub struct DefWithParentheses;

impl Cop for DefWithParentheses {
    fn name(&self) -> &str {
        "Style/DefWithParentheses"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use parentheses in method definitions with parameters"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let def_without_parens_regex = Regex::new(r#"\bdef\s+\w+\s+[a-zA-Z_]"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            if let Some(capture) = def_without_parens_regex.find(line) {
                let start_col = capture.start() + 1;
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Use parentheses in method definitions with parameters",
                    self.severity(),
                    Location::new(line_number, start_col, capture.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 19: DoubleNegation - don't use !!
// ============================================================================

pub struct DoubleNegation;

impl Cop for DoubleNegation {
    fn name(&self) -> &str {
        "Style/DoubleNegation"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Avoid using double negation (!!)"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            if let Some(pos) = line.find("!!") {
                let start_col = pos + 1;
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Avoid using double negation `!!`",
                    self.severity(),
                    Location::new(line_number, start_col, 2),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 20: EachForSimpleLoop - use times instead of each on range
// ============================================================================

pub struct EachForSimpleLoop;

impl Cop for EachForSimpleLoop {
    fn name(&self) -> &str {
        "Style/EachForSimpleLoop"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use Integer#times instead of each on a range"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let each_range_regex = Regex::new(r#"\(0\.\.\.?\d+\)\.each"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for capture in each_range_regex.find_iter(line) {
                let start_col = capture.start() + 1;
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Use `Integer#times` instead of `each` on a range",
                    self.severity(),
                    Location::new(line_number, start_col, capture.len()),
                ));
            }
        }

        offenses
    }
}


// ============================================================================
// Cop 21: EmptyBlockParameter - don't use empty block params { || }
// ============================================================================

pub struct EmptyBlockParameter;

impl Cop for EmptyBlockParameter {
    fn name(&self) -> &str {
        "Style/EmptyBlockParameter"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Remove empty block parameters"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let empty_block_regex = Regex::new(r#"\{\s*\|\|"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;

            for capture in empty_block_regex.find_iter(line) {
                let start_col = capture.start() + 1;

                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Remove empty block parameters",
                    self.severity(),
                    Location::new(line_number, start_col, capture.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 22: EmptyCaseCondition - case without condition
// ============================================================================

pub struct EmptyCaseCondition;

impl Cop for EmptyCaseCondition {
    fn name(&self) -> &str {
        "Style/EmptyCaseCondition"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Avoid case without condition"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let empty_case_regex = Regex::new(r#"^\s*case\s*$"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            if empty_case_regex.is_match(line) {
                offenses.push(Offense::new(
                    self.name(),
                    "Avoid `case` without a condition",
                    self.severity(),
                    Location::new(line_number, 1, line.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 23: EmptyElse - empty else clause
// ============================================================================

pub struct EmptyElse;

impl Cop for EmptyElse {
    fn name(&self) -> &str {
        "Style/EmptyElse"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Remove empty else clause"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            let trimmed = line.trim();
            
            if trimmed == "else" {
                if let Some(next_line) = source.line(line_number + 1) {
                    if next_line.trim() == "end" {
                        offenses.push(Offense::new(
                            self.name(),
                            "Remove empty `else` clause",
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

// ============================================================================
// Cop 24: EmptyHeredoc - empty heredoc
// ============================================================================

pub struct EmptyHeredoc;

impl Cop for EmptyHeredoc {
    fn name(&self) -> &str {
        "Style/EmptyHeredoc"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Avoid empty heredocs"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let heredoc_regex = Regex::new(r#"<<[-~]?([A-Z_]+)"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            if let Some(capture) = heredoc_regex.captures(line) {
                if let Some(delimiter) = capture.get(1) {
                    let delimiter_str = delimiter.as_str();
                    
                    if let Some(next_line) = source.line(line_number + 1) {
                        if next_line.trim() == delimiter_str {
                            offenses.push(Offense::new(
                                self.name(),
                                "Avoid empty heredoc",
                                self.severity(),
                                Location::new(line_number, 1, line.len()),
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
// Cop 25: EmptyLambdaParameter - don't use empty lambda params -> () { }
// ============================================================================

pub struct EmptyLambdaParameter;

impl Cop for EmptyLambdaParameter {
    fn name(&self) -> &str {
        "Style/EmptyLambdaParameter"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Remove empty lambda parameters"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let empty_lambda_regex = Regex::new(r#"->\s*\(\s*\)"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for capture in empty_lambda_regex.find_iter(line) {
                let start_col = capture.start() + 1;
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Remove empty lambda parameters",
                    self.severity(),
                    Location::new(line_number, start_col, capture.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 26: EmptyLiteral - use [] not Array.new, {} not Hash.new
// ============================================================================

pub struct EmptyLiteral;

impl Cop for EmptyLiteral {
    fn name(&self) -> &str {
        "Style/EmptyLiteral"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use literal syntax for empty arrays and hashes"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let array_new_regex = Regex::new(r#"\bArray\.new\b"#).unwrap();
        let hash_new_regex = Regex::new(r#"\bHash\.new\b"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for capture in array_new_regex.find_iter(line) {
                let start_col = capture.start() + 1;
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Use `[]` instead of `Array.new`",
                    self.severity(),
                    Location::new(line_number, start_col, capture.len()),
                ));
            }
            
            for capture in hash_new_regex.find_iter(line) {
                let start_col = capture.start() + 1;
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Use `{}` instead of `Hash.new`",
                    self.severity(),
                    Location::new(line_number, start_col, capture.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 27: Encoding - magic encoding comment
// ============================================================================

pub struct Encoding;

impl Cop for Encoding {
    fn name(&self) -> &str {
        "Style/Encoding"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use UTF-8 encoding comment when necessary"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        
        if source.is_empty() {
            return offenses;
        }

        let first_line = source.line(1).unwrap_or("");
        let second_line = source.line(2).unwrap_or("");
        
        let has_encoding = first_line.contains("coding:") || 
                          first_line.contains("encoding:") ||
                          second_line.contains("coding:") ||
                          second_line.contains("encoding:");

        if !has_encoding && !source.content.is_ascii() {
            offenses.push(Offense::new(
                self.name(),
                "Add encoding comment for non-ASCII characters",
                self.severity(),
                Location::new(1, 1, 1),
            ));
        }

        offenses
    }
}

// ============================================================================
// Cop 28: EndBlock - no END blocks
// ============================================================================

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
        "Avoid using END blocks"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let end_block_regex = Regex::new(r#"^\s*END\s*\{"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            if let Some(capture) = end_block_regex.find(line) {
                let start_col = capture.start() + 1;
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Avoid using END blocks",
                    self.severity(),
                    Location::new(line_number, start_col, capture.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 29: EvenOdd - use .even?/.odd? instead of % 2
// ============================================================================

pub struct EvenOdd;

impl Cop for EvenOdd {
    fn name(&self) -> &str {
        "Style/EvenOdd"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use Integer#even? or Integer#odd? instead of % 2"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let modulo_regex = Regex::new(r#"%\s*2\s*==\s*[01]"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for capture in modulo_regex.find_iter(line) {
                let start_col = capture.start() + 1;
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Use `Integer#even?` or `Integer#odd?` instead of `% 2`",
                    self.severity(),
                    Location::new(line_number, start_col, capture.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 30: For - don't use for loop
// ============================================================================

pub struct For;

impl Cop for For {
    fn name(&self) -> &str {
        "Style/For"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use each instead of for"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let for_regex = Regex::new(r#"^\s*for\s+\w+\s+in\s+"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            if let Some(capture) = for_regex.find(line) {
                let start_col = capture.start() + 1;
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Use `each` instead of `for`",
                    self.severity(),
                    Location::new(line_number, start_col, capture.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 31: FormatString - use consistent format string style
// ============================================================================

pub struct FormatString;

impl Cop for FormatString {
    fn name(&self) -> &str {
        "Style/FormatString"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use consistent format string style"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let _format_regex = Regex::new(r#"(sprintf|String#%)"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            if line.contains("sprintf") {
                if let Some(pos) = line.find("sprintf") {
                    let start_col = pos + 1;
                    
                    if source.in_string_or_comment(line_number, start_col) {
                        continue;
                    }

                    offenses.push(Offense::new(
                        self.name(),
                        "Prefer `format` over `sprintf`",
                        self.severity(),
                        Location::new(line_number, start_col, 7),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 32: GlobalStdStream - use $stdout not STDOUT
// ============================================================================

pub struct GlobalStdStream;

impl Cop for GlobalStdStream {
    fn name(&self) -> &str {
        "Style/GlobalStdStream"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use $stdout/$stderr/$stdin instead of STDOUT/STDERR/STDIN"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let std_const_regex = Regex::new(r#"\b(STDOUT|STDERR|STDIN)\b"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for capture in std_const_regex.captures_iter(line) {
                if let Some(matched) = capture.get(1) {
                    let start_col = matched.start() + 1;
                    
                    if source.in_string_or_comment(line_number, start_col) {
                        continue;
                    }

                    let const_name = matched.as_str();
                    let replacement = format!("${}", const_name.to_lowercase());
                    
                    offenses.push(Offense::new(
                        self.name(),
                        format!("Use `{}` instead of `{}`", replacement, const_name),
                        self.severity(),
                        Location::new(line_number, start_col, matched.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 33: GlobalVars - don't use global $ variables
// ============================================================================

pub struct GlobalVars;

impl Cop for GlobalVars {
    fn name(&self) -> &str {
        "Style/GlobalVars"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Avoid using global variables"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let global_var_regex = Regex::new(r#"\$[a-zA-Z_]\w*"#).unwrap();
        let allowed = ["$stdout", "$stderr", "$stdin", "$LOAD_PATH"];

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for capture in global_var_regex.find_iter(line) {
                let start_col = capture.start() + 1;
                let var_name = capture.as_str();
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                if allowed.contains(&var_name) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Avoid using global variables",
                    self.severity(),
                    Location::new(line_number, start_col, capture.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 34: GuardClause - use guard clause instead of wrapping method in if
// ============================================================================

pub struct GuardClause;

impl Cop for GuardClause {
    fn name(&self) -> &str {
        "Style/GuardClause"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use guard clause instead of wrapping method body in conditional"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let def_regex = Regex::new(r#"^\s*def\s+"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            if def_regex.is_match(line) {
                if let Some(next_line) = source.line(line_number + 1) {
                    let next_trimmed = next_line.trim();
                    if next_trimmed.starts_with("if ") || next_trimmed.starts_with("unless ") {
                        let mut depth = 1;
                        let _all_in_conditional = true;

                        for i in (line_number + 2)..=(line_number + 10).min(source.line_count()) {
                            if let Some(l) = source.line(i) {
                                let trimmed = l.trim();
                                if trimmed == "end" {
                                    depth -= 1;
                                    if depth == 0 {
                                        if let Some(next) = source.line(i + 1) {
                                            if next.trim() == "end" {
                                                offenses.push(Offense::new(
                                                    self.name(),
                                                    "Use a guard clause instead of wrapping the entire method body",
                                                    self.severity(),
                                                    Location::new(line_number + 1, 1, next_line.len()),
                                                ));
                                            }
                                        }
                                        break;
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
// Cop 35: HashSyntax - use new hash syntax key: value not :key => value
// ============================================================================

pub struct HashSyntax;

impl Cop for HashSyntax {
    fn name(&self) -> &str {
        "Style/HashSyntax"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use Ruby 1.9 hash syntax"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let old_hash_regex = Regex::new(r#":([a-zA-Z_]\w*)\s*=>"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for capture in old_hash_regex.captures_iter(line) {
                if let Some(matched) = capture.get(0) {
                    let start_col = matched.start() + 1;
                    
                    if source.in_string_or_comment(line_number, start_col) {
                        continue;
                    }

                    offenses.push(Offense::new(
                        self.name(),
                        "Use Ruby 1.9 hash syntax",
                        self.severity(),
                        Location::new(line_number, start_col, matched.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 36: IdenticalConditionalBranches - same code in if/else branches
// ============================================================================

pub struct IdenticalConditionalBranches;

impl Cop for IdenticalConditionalBranches {
    fn name(&self) -> &str {
        "Style/IdenticalConditionalBranches"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Avoid identical code in conditional branches"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            let trimmed = line.trim();
            
            if trimmed.starts_with("if ") {
                if let Some(if_body) = source.line(line_number + 1) {
                    let if_body_trimmed = if_body.trim();
                    
                    if let Some(else_line) = source.line(line_number + 2) {
                        if else_line.trim() == "else" {
                            if let Some(else_body) = source.line(line_number + 3) {
                                if else_body.trim() == if_body_trimmed && !if_body_trimmed.is_empty() {
                                    offenses.push(Offense::new(
                                        self.name(),
                                        "Identical code in both branches of conditional",
                                        self.severity(),
                                        Location::new(line_number, 1, line.len()),
                                    ));
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
// Cop 37: IfInsideElse - nested if inside else
// ============================================================================

pub struct IfInsideElse;

impl Cop for IfInsideElse {
    fn name(&self) -> &str {
        "Style/IfInsideElse"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use elsif instead of if nested in else"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            if line.trim() == "else" {
                if let Some(next_line) = source.line(line_number + 1) {
                    let next_trimmed = next_line.trim();
                    if next_trimmed.starts_with("if ") {
                        offenses.push(Offense::new(
                            self.name(),
                            "Use `elsif` instead of `if` nested in `else`",
                            self.severity(),
                            Location::new(line_number + 1, 1, next_line.len()),
                        ));
                    }
                }
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 38: IfUnlessModifier - use modifier form for single-line if/unless
// ============================================================================

pub struct IfUnlessModifier;

impl Cop for IfUnlessModifier {
    fn name(&self) -> &str {
        "Style/IfUnlessModifier"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use modifier form for single-line conditionals"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let if_unless_regex = Regex::new(r#"^\s*(if|unless)\s+"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            if if_unless_regex.is_match(line) && !line.contains(" then ") {
                if let Some(next_line) = source.line(line_number + 1) {
                    if let Some(third_line) = source.line(line_number + 2) {
                        if next_line.trim() != "" && third_line.trim() == "end" {
                            offenses.push(Offense::new(
                                self.name(),
                                "Use modifier form for single-line conditionals",
                                self.severity(),
                                Location::new(line_number, 1, line.len()),
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
// Cop 39: IfWithBooleanLiteralBranches - if x then true else false -> just x
// ============================================================================

pub struct IfWithBooleanLiteralBranches;

impl Cop for IfWithBooleanLiteralBranches {
    fn name(&self) -> &str {
        "Style/IfWithBooleanLiteralBranches"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Avoid if/else with boolean literal branches"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let bool_branch_regex = Regex::new(r#"(if|unless)\s+.+\s+then\s+(true|false)\s+else\s+(true|false)"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            if bool_branch_regex.is_match(line) {
                offenses.push(Offense::new(
                    self.name(),
                    "This conditional expression returns a boolean literal; use the condition directly",
                    self.severity(),
                    Location::new(line_number, 1, line.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 40: IfWithSemicolon - don't use if foo; bar; end
// ============================================================================

pub struct IfWithSemicolon;

impl Cop for IfWithSemicolon {
    fn name(&self) -> &str {
        "Style/IfWithSemicolon"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Do not use semicolons in if/unless/while/until"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let if_semicolon_regex = Regex::new(r#"(if|unless)\s+[^;]+;"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            if let Some(capture) = if_semicolon_regex.find(line) {
                let start_col = capture.start() + 1;
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Do not use `if/unless` with semicolons",
                    self.severity(),
                    Location::new(line_number, start_col, capture.len()),
                ));
            }
        }

        offenses
    }
}


// ============================================================================
// Cop 41: InfiniteLoop - use loop instead of while true
// ============================================================================

pub struct InfiniteLoop;

impl Cop for InfiniteLoop {
    fn name(&self) -> &str {
        "Style/InfiniteLoop"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use loop instead of while true"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let while_true_regex = Regex::new(r#"\bwhile\s+true\b"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for capture in while_true_regex.find_iter(line) {
                let start_col = capture.start() + 1;
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Use `loop` instead of `while true`",
                    self.severity(),
                    Location::new(line_number, start_col, capture.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 42: InlineComment - no inline comments
// ============================================================================

pub struct InlineComment;

impl Cop for InlineComment {
    fn name(&self) -> &str {
        "Style/InlineComment"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Avoid inline comments"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            if let Some(comment_pos) = line.find('#') {
                let before_comment = line[..comment_pos].trim();
                
                if !before_comment.is_empty() && comment_pos > 0 {
                    offenses.push(Offense::new(
                        self.name(),
                        "Avoid inline comments",
                        self.severity(),
                        Location::new(line_number, comment_pos + 1, line.len() - comment_pos),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 43: Lambda - use -> lambda syntax
// ============================================================================

pub struct Lambda;

impl Cop for Lambda {
    fn name(&self) -> &str {
        "Style/Lambda"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use -> for lambda literals"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let lambda_regex = Regex::new(r#"\blambda\s*\{"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for capture in lambda_regex.find_iter(line) {
                let start_col = capture.start() + 1;
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Use `->` for lambda literals",
                    self.severity(),
                    Location::new(line_number, start_col, capture.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 44: LambdaCall - use call instead of .()
// ============================================================================

pub struct LambdaCall;

impl Cop for LambdaCall {
    fn name(&self) -> &str {
        "Style/LambdaCall"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use call instead of .() for lambda invocation"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let lambda_call_regex = Regex::new(r#"\.\("#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for capture in lambda_call_regex.find_iter(line) {
                let start_col = capture.start() + 1;
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Use `call` instead of `.()` for lambda invocation",
                    self.severity(),
                    Location::new(line_number, start_col, capture.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 45: MagicCommentFormat - magic comment format consistency
// ============================================================================

pub struct MagicCommentFormat;

impl Cop for MagicCommentFormat {
    fn name(&self) -> &str {
        "Style/MagicCommentFormat"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use consistent magic comment format"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        for line_num in 0..3.min(source.line_count()) {
            let line_number = line_num + 1;
            
            if let Some(line) = source.line(line_number) {
                if (line.contains("coding:") || line.contains("encoding:"))
                    && !line.contains("# frozen_string_literal:") && !line.starts_with("#!")
                    && !line.starts_with("# ") {
                    offenses.push(Offense::new(
                        self.name(),
                        "Magic comment should have space after #",
                        self.severity(),
                        Location::new(line_number, 1, line.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 46: MethodCallWithoutArgsParentheses - no parens for method call without args
// ============================================================================

pub struct MethodCallWithoutArgsParentheses;

impl Cop for MethodCallWithoutArgsParentheses {
    fn name(&self) -> &str {
        "Style/MethodCallWithoutArgsParentheses"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Do not use parentheses for method calls with no arguments"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let method_call_regex = Regex::new(r#"\.\w+\(\)"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for capture in method_call_regex.find_iter(line) {
                let start_col = capture.start() + 1;
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Do not use parentheses for method calls with no arguments",
                    self.severity(),
                    Location::new(line_number, start_col, capture.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 47: MethodDefParentheses - use parens for method def
// ============================================================================

pub struct MethodDefParentheses;

impl Cop for MethodDefParentheses {
    fn name(&self) -> &str {
        "Style/MethodDefParentheses"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use parentheses in method definitions"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let def_no_parens_regex = Regex::new(r#"\bdef\s+\w+\s*$"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            if def_no_parens_regex.is_match(line) && !line.contains('(') {
                let trimmed = line.trim();
                if !trimmed.ends_with(')') {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use parentheses in method definitions",
                        self.severity(),
                        Location::new(line_number, 1, line.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 48: MixinUsage - use include/extend/prepend properly
// ============================================================================

pub struct MixinUsage;

impl Cop for MixinUsage {
    fn name(&self) -> &str {
        "Style/MixinUsage"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use include/extend/prepend at the top of the class"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let mixin_regex = Regex::new(r#"^\s+(include|extend|prepend)\s+\w+"#).unwrap();
        let mut in_class = false;
        let mut class_line = 0;
        let mut depth = 0;

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;

            if line.trim().starts_with("class ") {
                if !in_class {
                    in_class = true;
                    class_line = line_number;
                }
                depth += 1;
                continue;
            }

            // Track def/module/etc that increase depth
            if line.trim().starts_with("def ") || line.trim().starts_with("module ") {
                depth += 1;
            }

            if in_class && mixin_regex.is_match(line)
                && line_number - class_line > 10 {
                offenses.push(Offense::new(
                    self.name(),
                    "Use `include`/`extend`/`prepend` at the top of the class",
                    self.severity(),
                    Location::new(line_number, 1, line.len()),
                ));
            }

            if line.trim() == "end" && in_class {
                depth -= 1;
                if depth == 0 {
                    in_class = false;
                }
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 49: MultilineIfThen - no then on multiline if
// ============================================================================

pub struct MultilineIfThen;

impl Cop for MultilineIfThen {
    fn name(&self) -> &str {
        "Style/MultilineIfThen"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Do not use then on multiline if"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let if_then_regex = Regex::new(r#"^\s*if\s+.+\s+then\s*$"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            if if_then_regex.is_match(line) {
                offenses.push(Offense::new(
                    self.name(),
                    "Do not use `then` for multiline `if`",
                    self.severity(),
                    Location::new(line_number, 1, line.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 50: MultilineTernaryOperator - don't use multiline ternary
// ============================================================================

pub struct MultilineTernaryOperator;

impl Cop for MultilineTernaryOperator {
    fn name(&self) -> &str {
        "Style/MultilineTernaryOperator"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Avoid multiline ternary operators"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            if line.contains('?') && !line.contains(':') {
                if let Some(next_line) = source.line(line_number + 1) {
                    if next_line.contains(':') {
                        offenses.push(Offense::new(
                            self.name(),
                            "Avoid multiline ternary operators",
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

// ============================================================================
// Cop 51: MultilineWhenThen - no then on multiline when
// ============================================================================

pub struct MultilineWhenThen;

impl Cop for MultilineWhenThen {
    fn name(&self) -> &str {
        "Style/MultilineWhenThen"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Do not use then on multiline when"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let when_then_regex = Regex::new(r#"^\s*when\s+.+\s+then\s*$"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            if when_then_regex.is_match(line) {
                offenses.push(Offense::new(
                    self.name(),
                    "Do not use `then` for multiline `when`",
                    self.severity(),
                    Location::new(line_number, 1, line.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 52: MutableConstant - freeze mutable constants
// ============================================================================

pub struct MutableConstant;

impl Cop for MutableConstant {
    fn name(&self) -> &str {
        "Style/MutableConstant"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Freeze mutable constants"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let const_regex = Regex::new(r#"^\s*[A-Z][A-Z_]*\s*=\s*(\[|\{|%[wWiI])"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            if const_regex.is_match(line) && !line.contains(".freeze") {
                offenses.push(Offense::new(
                    self.name(),
                    "Freeze mutable objects assigned to constants",
                    self.severity(),
                    Location::new(line_number, 1, line.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 53: NegatedUnless - don't use unless !condition
// ============================================================================

pub struct NegatedUnless;

impl Cop for NegatedUnless {
    fn name(&self) -> &str {
        "Style/NegatedUnless"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use if instead of unless with negation"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let negated_unless_regex = Regex::new(r#"\bunless\s+!"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for capture in negated_unless_regex.find_iter(line) {
                let start_col = capture.start() + 1;
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Use `if` instead of `unless` with negation",
                    self.severity(),
                    Location::new(line_number, start_col, capture.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 54: NegatedWhile - use until instead of while !
// ============================================================================

pub struct NegatedWhile;

impl Cop for NegatedWhile {
    fn name(&self) -> &str {
        "Style/NegatedWhile"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use until instead of while with negation"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let negated_while_regex = Regex::new(r#"\bwhile\s+!"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for capture in negated_while_regex.find_iter(line) {
                let start_col = capture.start() + 1;
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Use `until` instead of `while` with negation",
                    self.severity(),
                    Location::new(line_number, start_col, capture.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 55: NestedModifier - don't nest modifier if/unless
// ============================================================================

pub struct NestedModifier;

impl Cop for NestedModifier {
    fn name(&self) -> &str {
        "Style/NestedModifier"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Avoid nested modifier if/unless"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let nested_modifier_regex = Regex::new(r#"\s+(if|unless)\s+.+\s+(if|unless)\s+"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            if nested_modifier_regex.is_match(line) {
                offenses.push(Offense::new(
                    self.name(),
                    "Avoid nested modifier `if`/`unless`",
                    self.severity(),
                    Location::new(line_number, 1, line.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 56: NestedTernaryOperator - don't nest ternary operators
// ============================================================================

pub struct NestedTernaryOperator;

impl Cop for NestedTernaryOperator {
    fn name(&self) -> &str {
        "Style/NestedTernaryOperator"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Avoid nested ternary operators"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            let question_count = line.matches('?').count();
            let colon_count = line.matches(':').count();
            
            if question_count > 1 && colon_count > 1 {
                offenses.push(Offense::new(
                    self.name(),
                    "Avoid nested ternary operators",
                    self.severity(),
                    Location::new(line_number, 1, line.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 57: Next - use next instead of if in loop
// ============================================================================

pub struct Next;

impl Cop for Next {
    fn name(&self) -> &str {
        "Style/Next"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use next to skip iteration instead of wrapping in conditional"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let mut in_loop = false;

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            let trimmed = line.trim();
            
            if trimmed.contains(".each") || trimmed.starts_with("while ") || trimmed.starts_with("until ") {
                in_loop = true;
            }

            if in_loop && (trimmed.starts_with("if ") || trimmed.starts_with("unless ")) {
                if let Some(next_line) = source.line(line_number + 1) {
                    if let Some(third_line) = source.line(line_number + 2) {
                        if !next_line.trim().is_empty() && third_line.trim() == "end" {
                            offenses.push(Offense::new(
                                self.name(),
                                "Use `next` to skip iteration",
                                self.severity(),
                                Location::new(line_number, 1, line.len()),
                            ));
                        }
                    }
                }
            }

            if trimmed == "end" {
                in_loop = false;
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 58: NilComparison - use .nil? not == nil
// ============================================================================

pub struct NilComparison;

impl Cop for NilComparison {
    fn name(&self) -> &str {
        "Style/NilComparison"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use .nil? instead of == nil"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let nil_comparison_regex = Regex::new(r#"(==|!=)\s*nil\b"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for capture in nil_comparison_regex.find_iter(line) {
                let start_col = capture.start() + 1;
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Use `.nil?` instead of `== nil`",
                    self.severity(),
                    Location::new(line_number, start_col, capture.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 59: NonNilCheck - use !x.nil? properly
// ============================================================================

pub struct NonNilCheck;

impl Cop for NonNilCheck {
    fn name(&self) -> &str {
        "Style/NonNilCheck"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use !x.nil? for non-nil checks"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let _non_nil_regex = Regex::new(r#"!\w+\.nil\?"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            if line.contains("!= nil") {
                if let Some(pos) = line.find("!= nil") {
                    let start_col = pos + 1;
                    
                    if source.in_string_or_comment(line_number, start_col) {
                        continue;
                    }

                    offenses.push(Offense::new(
                        self.name(),
                        "Use `!x.nil?` instead of `x != nil`",
                        self.severity(),
                        Location::new(line_number, start_col, 6),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 60: Not - use ! not not
// ============================================================================

pub struct Not;

impl Cop for Not {
    fn name(&self) -> &str {
        "Style/Not"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use ! instead of not"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let not_regex = Regex::new(r#"\bnot\s+"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for capture in not_regex.find_iter(line) {
                let start_col = capture.start() + 1;
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Use `!` instead of `not`",
                    self.severity(),
                    Location::new(line_number, start_col, 3),
                ));
            }
        }

        offenses
    }
}


// ============================================================================
// Cop 61: NumericLiterals - use underscores in large numbers 1_000_000
// ============================================================================

pub struct NumericLiterals;

impl Cop for NumericLiterals {
    fn name(&self) -> &str {
        "Style/NumericLiterals"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use underscores in large numeric literals"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let large_number_regex = Regex::new(r#"\b\d{6,}\b"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for capture in large_number_regex.find_iter(line) {
                let start_col = capture.start() + 1;
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                let num_str = capture.as_str();
                if !num_str.contains('_') {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use underscores in large numeric literals for readability",
                        self.severity(),
                        Location::new(line_number, start_col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 62: NumericLiteralPrefix - use 0o not 0 for octal
// ============================================================================

pub struct NumericLiteralPrefix;

impl Cop for NumericLiteralPrefix {
    fn name(&self) -> &str {
        "Style/NumericLiteralPrefix"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use 0o for octal, 0x for hex, 0b for binary"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let octal_regex = Regex::new(r#"\b0[0-7]+\b"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for capture in octal_regex.find_iter(line) {
                let start_col = capture.start() + 1;
                let num_str = capture.as_str();
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                if num_str.len() > 1 && !num_str.starts_with("0o") && !num_str.starts_with("0x") {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use `0o` prefix for octal literals",
                        self.severity(),
                        Location::new(line_number, start_col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 63: NumericPredicate - use .zero?/.positive?/.negative?
// ============================================================================

pub struct NumericPredicate;

impl Cop for NumericPredicate {
    fn name(&self) -> &str {
        "Style/NumericPredicate"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use numeric predicate methods"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let zero_comparison_regex = Regex::new(r#"==\s*0\b"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for capture in zero_comparison_regex.find_iter(line) {
                let start_col = capture.start() + 1;
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Use `.zero?` instead of `== 0`",
                    self.severity(),
                    Location::new(line_number, start_col, capture.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 64: OneLineConditional - use ternary for simple if/else
// ============================================================================

pub struct OneLineConditional;

impl Cop for OneLineConditional {
    fn name(&self) -> &str {
        "Style/OneLineConditional"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use ternary operator for simple conditionals"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            let trimmed = line.trim();
            
            if trimmed.starts_with("if ") && !line.contains('?') {
                if let Some(then_line) = source.line(line_number + 1) {
                    if let Some(else_line) = source.line(line_number + 2) {
                        if else_line.trim() == "else" {
                            if let Some(else_body) = source.line(line_number + 3) {
                                if let Some(end_line) = source.line(line_number + 4) {
                                    if end_line.trim() == "end" && 
                                       !then_line.trim().is_empty() && 
                                       !else_body.trim().is_empty() {
                                        offenses.push(Offense::new(
                                            self.name(),
                                            "Use ternary operator for simple conditionals",
                                            self.severity(),
                                            Location::new(line_number, 1, line.len()),
                                        ));
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
// Cop 65: OptionalBooleanParameter - avoid def foo(bar = true)
// ============================================================================

pub struct OptionalBooleanParameter;

impl Cop for OptionalBooleanParameter {
    fn name(&self) -> &str {
        "Style/OptionalBooleanParameter"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Avoid optional boolean parameters"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let bool_param_regex = Regex::new(r#"\bdef\s+\w+\([^)]*=\s*(true|false)"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            if bool_param_regex.is_match(line) {
                offenses.push(Offense::new(
                    self.name(),
                    "Avoid optional boolean parameters",
                    self.severity(),
                    Location::new(line_number, 1, line.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 66: OrAssignment - use ||= for conditional assignment
// ============================================================================

pub struct OrAssignment;

impl Cop for OrAssignment {
    fn name(&self) -> &str {
        "Style/OrAssignment"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use ||= for conditional assignment"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let or_assign_regex = Regex::new(r#"(\w+)\s*=\s*(\w+)\s*\|\|"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;

            for cap in or_assign_regex.captures_iter(line) {
                let left = cap.get(1).unwrap().as_str();
                let right = cap.get(2).unwrap().as_str();

                // Only flag if both sides are identical
                if left != right {
                    continue;
                }

                let full_match = cap.get(0).unwrap();
                let column = full_match.start() + 1;

                if source.in_string_or_comment(line_number, column) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Use `||=` for conditional assignment",
                    self.severity(),
                    Location::new(line_number, column, full_match.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 67: ParenthesesAroundCondition - no parens around if/unless condition
// ============================================================================

pub struct ParenthesesAroundCondition;

impl Cop for ParenthesesAroundCondition {
    fn name(&self) -> &str {
        "Style/ParenthesesAroundCondition"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Do not use parentheses around conditions"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let parens_condition_regex = Regex::new(r#"(if|unless|while|until)\s+\("#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for capture in parens_condition_regex.find_iter(line) {
                let start_col = capture.start() + 1;
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Do not use parentheses around conditions",
                    self.severity(),
                    Location::new(line_number, start_col, capture.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 68: PercentLiteralDelimiters - consistent delimiters for % literals
// ============================================================================

pub struct PercentLiteralDelimiters;

impl Cop for PercentLiteralDelimiters {
    fn name(&self) -> &str {
        "Style/PercentLiteralDelimiters"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use consistent delimiters for percent literals"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let percent_literal_regex = Regex::new(r#"%[wWiIqQrsx][{(\[<]"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for capture in percent_literal_regex.find_iter(line) {
                let start_col = capture.start() + 1;
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                let delimiter = capture.as_str().chars().last().unwrap();
                if delimiter != '(' && delimiter != '[' {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use consistent delimiters for percent literals",
                        self.severity(),
                        Location::new(line_number, start_col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 69: PercentQLiterals - use %q/%Q consistently
// ============================================================================

pub struct PercentQLiterals;

impl Cop for PercentQLiterals {
    fn name(&self) -> &str {
        "Style/PercentQLiterals"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use %q for single-quoted, %Q for double-quoted strings"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let percent_q_regex = Regex::new(r#"%[qQ][{(\[<]"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for capture in percent_q_regex.find_iter(line) {
                let start_col = capture.start() + 1;
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Use `%q` for single-quoted strings, `%Q` for double-quoted strings",
                    self.severity(),
                    Location::new(line_number, start_col, capture.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 70: PerlBackrefs - don't use $1, $2 Perl backrefs
// ============================================================================

pub struct PerlBackrefs;

impl Cop for PerlBackrefs {
    fn name(&self) -> &str {
        "Style/PerlBackrefs"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use named captures instead of numbered backreferences"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let perl_backref_regex = Regex::new(r#"\$\d+"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for capture in perl_backref_regex.find_iter(line) {
                let start_col = capture.start() + 1;
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Use named captures instead of numbered backreferences",
                    self.severity(),
                    Location::new(line_number, start_col, capture.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 71: Proc - use proc not Proc.new
// ============================================================================

pub struct Proc;

impl Cop for Proc {
    fn name(&self) -> &str {
        "Style/Proc"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use proc instead of Proc.new"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let proc_new_regex = Regex::new(r#"\bProc\.new\b"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for capture in proc_new_regex.find_iter(line) {
                let start_col = capture.start() + 1;
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Use `proc` instead of `Proc.new`",
                    self.severity(),
                    Location::new(line_number, start_col, capture.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 72: RaiseArgs - consistent raise argument style
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
        "Use consistent raise argument style"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let raise_with_new_regex = Regex::new(r#"\braise\s+\w+\.new\("#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for capture in raise_with_new_regex.find_iter(line) {
                let start_col = capture.start() + 1;
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Use `raise ErrorClass, message` instead of `raise ErrorClass.new(message)`",
                    self.severity(),
                    Location::new(line_number, start_col, capture.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 73: RedundantBegin - redundant begin in method body
// ============================================================================

pub struct RedundantBegin;

impl Cop for RedundantBegin {
    fn name(&self) -> &str {
        "Style/RedundantBegin"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Remove redundant begin block in method definition"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            if line.trim().starts_with("def ") {
                if let Some(next_line) = source.line(line_number + 1) {
                    if next_line.trim() == "begin" {
                        offenses.push(Offense::new(
                            self.name(),
                            "Redundant `begin` block in method definition",
                            self.severity(),
                            Location::new(line_number + 1, 1, next_line.len()),
                        ));
                    }
                }
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 74: RedundantException - redundant RuntimeError in raise
// ============================================================================

pub struct RedundantException;

impl Cop for RedundantException {
    fn name(&self) -> &str {
        "Style/RedundantException"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Remove redundant RuntimeError from raise"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let runtime_error_regex = Regex::new(r#"\braise\s+RuntimeError,"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for capture in runtime_error_regex.find_iter(line) {
                let start_col = capture.start() + 1;
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Redundant `RuntimeError` in raise statement",
                    self.severity(),
                    Location::new(line_number, start_col, capture.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 75: RedundantFreeze - redundant .freeze on immutable
// ============================================================================

pub struct RedundantFreeze;

impl Cop for RedundantFreeze {
    fn name(&self) -> &str {
        "Style/RedundantFreeze"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Remove redundant .freeze on immutable objects"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let immutable_freeze_regex = Regex::new(r#"(true|false|nil|\d+)\.freeze"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for capture in immutable_freeze_regex.find_iter(line) {
                let start_col = capture.start() + 1;
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Redundant `.freeze` on immutable object",
                    self.severity(),
                    Location::new(line_number, start_col, capture.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 76: RedundantInterpolation - "#{var}" should just be var.to_s
// ============================================================================

pub struct RedundantInterpolation;

impl Cop for RedundantInterpolation {
    fn name(&self) -> &str {
        "Style/RedundantInterpolation"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Remove redundant string interpolation"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let redundant_interpolation_regex = Regex::new(r##""#\{\w+\}""##).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for capture in redundant_interpolation_regex.find_iter(line) {
                let start_col = capture.start() + 1;
                let matched = capture.as_str();
                
                if matched.matches("#{").count() == 1 && 
                   matched.len() < 30 &&
                   !matched.contains(' ') {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use variable or expression directly instead of interpolation",
                        self.severity(),
                        Location::new(line_number, start_col, capture.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 77: RedundantParentheses - unnecessary ()
// ============================================================================

pub struct RedundantParentheses;

impl Cop for RedundantParentheses {
    fn name(&self) -> &str {
        "Style/RedundantParentheses"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Remove redundant parentheses"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let redundant_parens_regex = Regex::new(r#"return\s+\("#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for capture in redundant_parens_regex.find_iter(line) {
                let start_col = capture.start() + 1;
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Remove redundant parentheses",
                    self.severity(),
                    Location::new(line_number, start_col, capture.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 78: RedundantPercentQ - use quotes instead of %q
// ============================================================================

pub struct RedundantPercentQ;

impl Cop for RedundantPercentQ {
    fn name(&self) -> &str {
        "Style/RedundantPercentQ"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use quotes instead of %q when possible"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let percent_q_regex = Regex::new(r#"%q[{(\[<][^})\]>]*[})\]>]"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for capture in percent_q_regex.find_iter(line) {
                let start_col = capture.start() + 1;
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Use quotes instead of `%q` when possible",
                    self.severity(),
                    Location::new(line_number, start_col, capture.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 79: RedundantSelf - unnecessary self.
// ============================================================================

pub struct RedundantSelf;

impl Cop for RedundantSelf {
    fn name(&self) -> &str {
        "Style/RedundantSelf"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Remove redundant self"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let redundant_self_regex = Regex::new(r#"\bself\.\w+\b"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            if line.contains("self.") && !line.contains("def self.") && !line.contains("self.class") {
                for capture in redundant_self_regex.find_iter(line) {
                    let start_col = capture.start() + 1;
                    
                    if source.in_string_or_comment(line_number, start_col) {
                        continue;
                    }

                    if !line.contains('=') || line.find("self.").unwrap() > line.find('=').unwrap_or(usize::MAX) {
                        offenses.push(Offense::new(
                            self.name(),
                            "Remove redundant `self`",
                            self.severity(),
                            Location::new(line_number, start_col, 5),
                        ));
                    }
                }
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 80: RegexpLiteral - use // vs %r{}
// ============================================================================

pub struct RegexpLiteral;

impl Cop for RegexpLiteral {
    fn name(&self) -> &str {
        "Style/RegexpLiteral"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use // for regexps instead of %r{}"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let percent_r_regex = Regex::new(r#"%r[{(\[<]"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for capture in percent_r_regex.find_iter(line) {
                let start_col = capture.start() + 1;
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Use `//` for regexps instead of `%r{}`",
                    self.severity(),
                    Location::new(line_number, start_col, capture.len()),
                ));
            }
        }

        offenses
    }
}


// ============================================================================
// Cop 81: RescueModifier - don't use rescue modifier
// ============================================================================

pub struct RescueModifier;

impl Cop for RescueModifier {
    fn name(&self) -> &str {
        "Style/RescueModifier"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Avoid using rescue modifier"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let rescue_modifier_regex = Regex::new(r#"\s+rescue\s+"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            let trimmed = line.trim();
            
            if !trimmed.starts_with("rescue") && rescue_modifier_regex.is_match(line) {
                if let Some(pos) = line.find(" rescue ") {
                    let start_col = pos + 1;
                    
                    if source.in_string_or_comment(line_number, start_col) {
                        continue;
                    }

                    offenses.push(Offense::new(
                        self.name(),
                        "Avoid using `rescue` modifier",
                        self.severity(),
                        Location::new(line_number, start_col, 7),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 82: RescueStandardError - rescue StandardError not bare rescue
// ============================================================================

pub struct RescueStandardError;

impl Cop for RescueStandardError {
    fn name(&self) -> &str {
        "Style/RescueStandardError"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Rescue StandardError explicitly instead of bare rescue"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            let trimmed = line.trim();
            
            if trimmed == "rescue" {
                offenses.push(Offense::new(
                    self.name(),
                    "Rescue `StandardError` explicitly instead of bare `rescue`",
                    self.severity(),
                    Location::new(line_number, 1, line.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 83: SafeNavigation - use &. safe navigation
// ============================================================================

pub struct SafeNavigation;

impl Cop for SafeNavigation {
    fn name(&self) -> &str {
        "Style/SafeNavigation"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use safe navigation operator &."
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let nil_check_regex = Regex::new(r#"(\w+)\s*&&\s*(\w+)\."#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;

            for cap in nil_check_regex.captures_iter(line) {
                let left = cap.get(1).unwrap().as_str();
                let right = cap.get(2).unwrap().as_str();

                // Only flag if both sides are identical
                if left != right {
                    continue;
                }

                let full_match = cap.get(0).unwrap();
                let column = full_match.start() + 1;

                if source.in_string_or_comment(line_number, column) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Use safe navigation operator `&.` instead of checking for nil",
                    self.severity(),
                    Location::new(line_number, column, full_match.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 84: SelfAssignment - use += instead of x = x +
// ============================================================================

pub struct SelfAssignment;

impl Cop for SelfAssignment {
    fn name(&self) -> &str {
        "Style/SelfAssignment"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use self-assignment shorthand"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let self_assign_regex = Regex::new(r#"(\w+)\s*=\s*(\w+)\s*([+\-*/%])"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;

            for cap in self_assign_regex.captures_iter(line) {
                let left = cap.get(1).unwrap().as_str();
                let right = cap.get(2).unwrap().as_str();

                // Only flag if both sides are identical
                if left != right {
                    continue;
                }

                let full_match = cap.get(0).unwrap();
                let column = full_match.start() + 1;

                if source.in_string_or_comment(line_number, column) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Use self-assignment shorthand",
                    self.severity(),
                    Location::new(line_number, column, full_match.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 85: Semicolon - don't use ; to separate statements
// ============================================================================

pub struct Semicolon;

impl Cop for Semicolon {
    fn name(&self) -> &str {
        "Style/Semicolon"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Do not use semicolons to separate statements"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            if let Some(pos) = line.find(';') {
                let start_col = pos + 1;
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Do not use semicolons to separate statements",
                    self.severity(),
                    Location::new(line_number, start_col, 1),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 86: Send - don't use .send
// ============================================================================

pub struct Send;

impl Cop for Send {
    fn name(&self) -> &str {
        "Style/Send"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Avoid using send method"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let send_regex = Regex::new(r#"\.send\("#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for capture in send_regex.find_iter(line) {
                let start_col = capture.start() + 1;
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Avoid using `send` method",
                    self.severity(),
                    Location::new(line_number, start_col, capture.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 87: SignalException - use raise not fail
// ============================================================================

pub struct SignalException;

impl Cop for SignalException {
    fn name(&self) -> &str {
        "Style/SignalException"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use raise instead of fail"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let fail_regex = Regex::new(r#"\bfail\s+"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for capture in fail_regex.find_iter(line) {
                let start_col = capture.start() + 1;
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Use `raise` instead of `fail`",
                    self.severity(),
                    Location::new(line_number, start_col, 4),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 88: SingleLineMethods - don't define single-line methods
// ============================================================================

pub struct SingleLineMethods;

impl Cop for SingleLineMethods {
    fn name(&self) -> &str {
        "Style/SingleLineMethods"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Avoid single-line method definitions"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let single_line_method_regex = Regex::new(r#"\bdef\s+\w+.*;\s*\w+.*;\s*end"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            if single_line_method_regex.is_match(line) {
                offenses.push(Offense::new(
                    self.name(),
                    "Avoid single-line method definitions",
                    self.severity(),
                    Location::new(line_number, 1, line.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 89: SpecialGlobalVars - use readable names for special $ vars
// ============================================================================

pub struct SpecialGlobalVars;

impl Cop for SpecialGlobalVars {
    fn name(&self) -> &str {
        "Style/SpecialGlobalVars"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use readable names for special global variables"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let special_vars = ["$:", "$;", "$,", "$/", "$\\", "$.", "$_", "$>", "$<"];

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for special_var in &special_vars {
                if let Some(pos) = line.find(special_var) {
                    let start_col = pos + 1;
                    
                    if source.in_string_or_comment(line_number, start_col) {
                        continue;
                    }

                    offenses.push(Offense::new(
                        self.name(),
                        format!("Use readable name instead of `{}`", special_var),
                        self.severity(),
                        Location::new(line_number, start_col, special_var.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 90: StderrPuts - use warn not $stderr.puts
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
        let stderr_puts_regex = Regex::new(r#"\$stderr\.puts"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for capture in stderr_puts_regex.find_iter(line) {
                let start_col = capture.start() + 1;
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Use `warn` instead of `$stderr.puts`",
                    self.severity(),
                    Location::new(line_number, start_col, capture.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 91: StringConcatenation - use interpolation not + for strings
// ============================================================================

pub struct StringConcatenation;

impl Cop for StringConcatenation {
    fn name(&self) -> &str {
        "Style/StringConcatenation"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use string interpolation instead of concatenation"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let string_concat_regex = Regex::new(r#"["'][^"']*["']\s*\+\s*["']"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for capture in string_concat_regex.find_iter(line) {
                let start_col = capture.start() + 1;
                
                offenses.push(Offense::new(
                    self.name(),
                    "Use string interpolation instead of concatenation",
                    self.severity(),
                    Location::new(line_number, start_col, capture.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 92: StructInheritance - use Struct.new not class inheritance
// ============================================================================

pub struct StructInheritance;

impl Cop for StructInheritance {
    fn name(&self) -> &str {
        "Style/StructInheritance"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use Struct.new instead of inheriting from Struct"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let struct_inherit_regex = Regex::new(r#"class\s+\w+\s*<\s*Struct\.new"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            if struct_inherit_regex.is_match(line) {
                offenses.push(Offense::new(
                    self.name(),
                    "Use `Struct.new` instead of inheriting from Struct",
                    self.severity(),
                    Location::new(line_number, 1, line.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 93: SymbolArray - use %i[] for symbol arrays
// ============================================================================

pub struct SymbolArray;

impl Cop for SymbolArray {
    fn name(&self) -> &str {
        "Style/SymbolArray"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Use %i or %I for symbol arrays"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let symbol_array_regex = Regex::new(r#"\[:\w+(,\s*:\w+){2,}\]"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for capture in symbol_array_regex.find_iter(line) {
                let start_col = capture.start() + 1;
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Use `%i` or `%I` for symbol arrays",
                    self.severity(),
                    Location::new(line_number, start_col, capture.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 94: SymbolLiteral - redundant symbol quoting
// ============================================================================

pub struct SymbolLiteral;

impl Cop for SymbolLiteral {
    fn name(&self) -> &str {
        "Style/SymbolLiteral"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Avoid redundant quotes in symbols"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let quoted_symbol_regex = Regex::new(r#":["']([a-zA-Z_]\w*)["']"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for capture in quoted_symbol_regex.find_iter(line) {
                let start_col = capture.start() + 1;
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Avoid redundant quotes in symbols",
                    self.severity(),
                    Location::new(line_number, start_col, capture.len()),
                ));
            }
        }

        offenses
    }
}

// ============================================================================
// Cop 95: TernaryParentheses - no parens in ternary condition
// ============================================================================

pub struct TernaryParentheses;

impl Cop for TernaryParentheses {
    fn name(&self) -> &str {
        "Style/TernaryParentheses"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Do not use parentheses in ternary conditions"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let ternary_parens_regex = Regex::new(r#"\(\s*[^)]+\s*\)\s*\?"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            
            for capture in ternary_parens_regex.find_iter(line) {
                let start_col = capture.start() + 1;
                
                if source.in_string_or_comment(line_number, start_col) {
                    continue;
                }

                offenses.push(Offense::new(
                    self.name(),
                    "Do not use parentheses in ternary conditions",
                    self.severity(),
                    Location::new(line_number, start_col, capture.len()),
                ));
            }
        }

        offenses
    }
}

// Collect all cops from this module
pub fn all_style_extra1_cops() -> Vec<Box<dyn Cop>> {
    vec![
        Box::new(AndOr),
        Box::new(AsciiComments),
        Box::new(Attr),
        Box::new(BarePercentLiterals),
        Box::new(BeginBlock),
        Box::new(BlockComments),
        Box::new(CaseEquality),
        Box::new(CharacterLiteral),
        Box::new(ClassCheck),
        Box::new(ClassVars),
        Box::new(ColonMethodCall),
        Box::new(ColonMethodDefinition),
        Box::new(CommandLiteral),
        Box::new(CommentAnnotation),
        Box::new(CommentedKeyword),
        Box::new(Copyright),
        Box::new(DateTime),
        Box::new(DefWithParentheses),
        Box::new(DoubleNegation),
        Box::new(EachForSimpleLoop),
        Box::new(EmptyBlockParameter),
        Box::new(EmptyCaseCondition),
        Box::new(EmptyElse),
        Box::new(EmptyHeredoc),
        Box::new(EmptyLambdaParameter),
        Box::new(EmptyLiteral),
        Box::new(Encoding),
        Box::new(EndBlock),
        Box::new(EvenOdd),
        Box::new(For),
        Box::new(FormatString),
        Box::new(GlobalStdStream),
        Box::new(GlobalVars),
        Box::new(GuardClause),
        Box::new(HashSyntax),
        Box::new(IdenticalConditionalBranches),
        Box::new(IfInsideElse),
        Box::new(IfUnlessModifier),
        Box::new(IfWithBooleanLiteralBranches),
        Box::new(IfWithSemicolon),
        Box::new(InfiniteLoop),
        Box::new(InlineComment),
        Box::new(Lambda),
        Box::new(LambdaCall),
        Box::new(MagicCommentFormat),
        Box::new(MethodCallWithoutArgsParentheses),
        Box::new(MethodDefParentheses),
        Box::new(MixinUsage),
        Box::new(MultilineIfThen),
        Box::new(MultilineTernaryOperator),
        Box::new(MultilineWhenThen),
        Box::new(MutableConstant),
        Box::new(NegatedUnless),
        Box::new(NegatedWhile),
        Box::new(NestedModifier),
        Box::new(NestedTernaryOperator),
        Box::new(Next),
        Box::new(NilComparison),
        Box::new(NonNilCheck),
        Box::new(Not),
        Box::new(NumericLiterals),
        Box::new(NumericLiteralPrefix),
        Box::new(NumericPredicate),
        Box::new(OneLineConditional),
        Box::new(OptionalBooleanParameter),
        Box::new(OrAssignment),
        Box::new(ParenthesesAroundCondition),
        Box::new(PercentLiteralDelimiters),
        Box::new(PercentQLiterals),
        Box::new(PerlBackrefs),
        Box::new(Proc),
        Box::new(RaiseArgs),
        Box::new(RedundantBegin),
        Box::new(RedundantException),
        Box::new(RedundantFreeze),
        Box::new(RedundantInterpolation),
        Box::new(RedundantParentheses),
        Box::new(RedundantPercentQ),
        Box::new(RedundantSelf),
        Box::new(RegexpLiteral),
        Box::new(RescueModifier),
        Box::new(RescueStandardError),
        Box::new(SafeNavigation),
        Box::new(SelfAssignment),
        Box::new(Semicolon),
        Box::new(Send),
        Box::new(SignalException),
        Box::new(SingleLineMethods),
        Box::new(SpecialGlobalVars),
        Box::new(StderrPuts),
        Box::new(StringConcatenation),
        Box::new(StructInheritance),
        Box::new(SymbolArray),
        Box::new(SymbolLiteral),
        Box::new(TernaryParentheses),
    ]
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

    #[test]
    fn test_and_or_pass() {
        let source = test_source("x = true && false\ny = true || false\n");
        let cop = AndOr;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_and_or_fail() {
        let source = test_source("x = true and false\n");
        let cop = AndOr;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_ascii_comments_pass() {
        let source = test_source("# This is a comment\n");
        let cop = AsciiComments;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_ascii_comments_fail() {
        let source = test_source("# This is a comment with émoji\n");
        let cop = AsciiComments;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_attr_pass() {
        let source = test_source("attr_reader :name\n");
        let cop = Attr;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_attr_fail() {
        let source = test_source("attr :name\n");
        let cop = Attr;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_bare_percent_literals_pass() {
        let source = test_source("%q(hello)\n");
        let cop = BarePercentLiterals;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_bare_percent_literals_fail() {
        let source = test_source("%(hello)\n");
        let cop = BarePercentLiterals;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_begin_block_pass() {
        let source = test_source("begin\n  x = 1\nend\n");
        let cop = BeginBlock;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_begin_block_fail() {
        let source = test_source("BEGIN { puts 'hello' }\n");
        let cop = BeginBlock;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_block_comments_pass() {
        let source = test_source("# comment\n# another\n");
        let cop = BlockComments;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_block_comments_fail() {
        let source = test_source("=begin\ncomment\n=end\n");
        let cop = BlockComments;
        assert_eq!(cop.check(&source).len(), 2);
    }

    #[test]
    fn test_case_equality_pass() {
        let source = test_source("x == y\n");
        let cop = CaseEquality;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_case_equality_fail() {
        let source = test_source("x === y\n");
        let cop = CaseEquality;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_character_literal_pass() {
        let source = test_source("x = 'a'\n");
        let cop = CharacterLiteral;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_character_literal_fail() {
        let source = test_source("x = ?a\n");
        let cop = CharacterLiteral;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_class_check_pass() {
        let source = test_source("x.is_a?(String)\n");
        let cop = ClassCheck;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_class_check_fail() {
        let source = test_source("x.kind_of?(String)\n");
        let cop = ClassCheck;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_class_vars_pass() {
        let source = test_source("@instance_var = 1\n");
        let cop = ClassVars;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_class_vars_fail() {
        let source = test_source("@@class_var = 1\n");
        let cop = ClassVars;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_colon_method_call_pass() {
        let source = test_source("Foo.bar\n");
        let cop = ColonMethodCall;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_colon_method_call_fail() {
        let source = test_source("Foo::bar()\n");
        let cop = ColonMethodCall;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_colon_method_definition_pass() {
        let source = test_source("def self.method\nend\n");
        let cop = ColonMethodDefinition;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_colon_method_definition_fail() {
        let source = test_source("def self::method\nend\n");
        let cop = ColonMethodDefinition;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_command_literal_pass() {
        let source = test_source("`ls`\n");
        let cop = CommandLiteral;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_command_literal_fail() {
        let source = test_source("%x(ls)\n");
        let cop = CommandLiteral;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_comment_annotation_pass() {
        let source = test_source("# TODO: fix this\n");
        let cop = CommentAnnotation;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_comment_annotation_fail() {
        let source = test_source("# TODO fix this\n");
        let cop = CommentAnnotation;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_double_negation_pass() {
        let source = test_source("!x\n");
        let cop = DoubleNegation;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_double_negation_fail() {
        let source = test_source("!!x\n");
        let cop = DoubleNegation;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_datetime_pass() {
        let source = test_source("Time.now\n");
        let cop = DateTime;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_datetime_fail() {
        let source = test_source("DateTime.now\n");
        let cop = DateTime;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_empty_block_parameter_pass() {
        let source = test_source("{ |x| puts x }\n");
        let cop = EmptyBlockParameter;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_empty_block_parameter_fail() {
        let source = test_source("{ || puts 'hi' }\n");
        let cop = EmptyBlockParameter;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_empty_literal_pass() {
        let source = test_source("x = []\ny = {}\n");
        let cop = EmptyLiteral;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_empty_literal_fail() {
        let source = test_source("x = Array.new\n");
        let cop = EmptyLiteral;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_end_block_pass() {
        let source = test_source("at_exit { puts 'bye' }\n");
        let cop = EndBlock;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_end_block_fail() {
        let source = test_source("END { puts 'bye' }\n");
        let cop = EndBlock;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_even_odd_pass() {
        let source = test_source("x.even?\n");
        let cop = EvenOdd;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_even_odd_fail() {
        let source = test_source("x % 2 == 0\n");
        let cop = EvenOdd;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_for_pass() {
        let source = test_source("items.each do |item|\nend\n");
        let cop = For;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_for_fail() {
        let source = test_source("for item in items\nend\n");
        let cop = For;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_global_std_stream_pass() {
        let source = test_source("$stdout.puts 'hi'\n");
        let cop = GlobalStdStream;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_global_std_stream_fail() {
        let source = test_source("STDOUT.puts 'hi'\n");
        let cop = GlobalStdStream;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_hash_syntax_pass() {
        let source = test_source("{ a: 1 }\n");
        let cop = HashSyntax;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_hash_syntax_fail() {
        let source = test_source("{ :a => 1 }\n");
        let cop = HashSyntax;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_infinite_loop_pass() {
        let source = test_source("loop do\nend\n");
        let cop = InfiniteLoop;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_infinite_loop_fail() {
        let source = test_source("while true\nend\n");
        let cop = InfiniteLoop;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_lambda_pass() {
        let source = test_source("-> { puts 'hi' }\n");
        let cop = Lambda;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_lambda_fail() {
        let source = test_source("lambda { puts 'hi' }\n");
        let cop = Lambda;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_lambda_call_pass() {
        let source = test_source("my_lambda.call\n");
        let cop = LambdaCall;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_lambda_call_fail() {
        let source = test_source("my_lambda.()\n");
        let cop = LambdaCall;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_negated_unless_pass() {
        let source = test_source("unless x\nend\n");
        let cop = NegatedUnless;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_negated_unless_fail() {
        let source = test_source("unless !x\nend\n");
        let cop = NegatedUnless;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_negated_while_pass() {
        let source = test_source("until x\nend\n");
        let cop = NegatedWhile;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_negated_while_fail() {
        let source = test_source("while !x\nend\n");
        let cop = NegatedWhile;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_nil_comparison_pass() {
        let source = test_source("x.nil?\n");
        let cop = NilComparison;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_nil_comparison_fail() {
        let source = test_source("x == nil\n");
        let cop = NilComparison;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_not_pass() {
        let source = test_source("!x\n");
        let cop = Not;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_not_fail() {
        let source = test_source("not x\n");
        let cop = Not;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_numeric_literals_pass() {
        let source = test_source("x = 1_000_000\n");
        let cop = NumericLiterals;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_numeric_literals_fail() {
        let source = test_source("x = 1000000\n");
        let cop = NumericLiterals;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_numeric_predicate_pass() {
        let source = test_source("x.zero?\n");
        let cop = NumericPredicate;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_numeric_predicate_fail() {
        let source = test_source("x == 0\n");
        let cop = NumericPredicate;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_proc_pass() {
        let source = test_source("proc { }\n");
        let cop = Proc;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_proc_fail() {
        let source = test_source("Proc.new { }\n");
        let cop = Proc;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_rescue_modifier_pass() {
        let source = test_source("begin\n  foo\nrescue\n  bar\nend\n");
        let cop = RescueModifier;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_rescue_modifier_fail() {
        let source = test_source("foo rescue bar\n");
        let cop = RescueModifier;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_self_assignment_pass() {
        let source = test_source("x += 1\n");
        let cop = SelfAssignment;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_self_assignment_fail() {
        let source = test_source("x = x + 1\n");
        let cop = SelfAssignment;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_semicolon_pass() {
        let source = test_source("x = 1\ny = 2\n");
        let cop = Semicolon;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_semicolon_fail() {
        let source = test_source("x = 1; y = 2\n");
        let cop = Semicolon;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_send_pass() {
        let source = test_source("obj.public_send(:method)\n");
        let cop = Send;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_send_fail() {
        let source = test_source("obj.send(:method)\n");
        let cop = Send;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_signal_exception_pass() {
        let source = test_source("raise 'error'\n");
        let cop = SignalException;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_signal_exception_fail() {
        let source = test_source("fail 'error'\n");
        let cop = SignalException;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_stderr_puts_pass() {
        let source = test_source("warn 'message'\n");
        let cop = StderrPuts;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_stderr_puts_fail() {
        let source = test_source("$stderr.puts 'message'\n");
        let cop = StderrPuts;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_string_concatenation_pass() {
        let source = test_source("\"hello #{name}\"\n");
        let cop = StringConcatenation;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_string_concatenation_fail() {
        let source = test_source("\"hello \" + \"world\"\n");
        let cop = StringConcatenation;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_symbol_literal_pass() {
        let source = test_source(":symbol\n");
        let cop = SymbolLiteral;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_symbol_literal_fail() {
        let source = test_source(":\"symbol\"\n");
        let cop = SymbolLiteral;
        assert_eq!(cop.check(&source).len(), 1);
    }
}

#[cfg(test)]
mod additional_tests {
    use super::*;
    use std::path::PathBuf;

    fn test_source(content: &str) -> SourceFile {
        SourceFile::from_string(PathBuf::from("test.rb"), content.to_string())
    }

    #[test]
    fn test_commented_keyword_pass() {
        let source = test_source("begin\n  x = 1\nend\n");
        let cop = CommentedKeyword;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_commented_keyword_fail() {
        let source = test_source("end # comment\n");
        let cop = CommentedKeyword;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_copyright_pass() {
        let source = test_source("# Copyright 2024\nclass Foo\nend\n");
        let cop = Copyright;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_copyright_fail() {
        let source = test_source("class Foo\nend\n");
        let cop = Copyright;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_def_with_parentheses_pass() {
        let source = test_source("def foo(x)\nend\n");
        let cop = DefWithParentheses;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_def_with_parentheses_fail() {
        let source = test_source("def foo x\nend\n");
        let cop = DefWithParentheses;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_each_for_simple_loop_pass() {
        let source = test_source("10.times do\nend\n");
        let cop = EachForSimpleLoop;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_each_for_simple_loop_fail() {
        let source = test_source("(0...10).each do\nend\n");
        let cop = EachForSimpleLoop;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_empty_case_condition_pass() {
        let source = test_source("case x\nwhen 1\nend\n");
        let cop = EmptyCaseCondition;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_empty_case_condition_fail() {
        let source = test_source("case\nwhen x == 1\nend\n");
        let cop = EmptyCaseCondition;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_empty_else_pass() {
        let source = test_source("if x\n  y\nelse\n  z\nend\n");
        let cop = EmptyElse;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_empty_else_fail() {
        let source = test_source("if x\n  y\nelse\nend\n");
        let cop = EmptyElse;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_empty_heredoc_pass() {
        let source = test_source("<<~TEXT\nHello\nTEXT\n");
        let cop = EmptyHeredoc;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_empty_heredoc_fail() {
        let source = test_source("<<TEXT\nTEXT\n");
        let cop = EmptyHeredoc;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_empty_lambda_parameter_pass() {
        let source = test_source("-> { puts 'hi' }\n");
        let cop = EmptyLambdaParameter;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_empty_lambda_parameter_fail() {
        let source = test_source("-> () { puts 'hi' }\n");
        let cop = EmptyLambdaParameter;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_encoding_pass() {
        let source = test_source("# encoding: utf-8\nclass Foo\nend\n");
        let cop = Encoding;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_encoding_fail() {
        let source = test_source("# café\nclass Foo\nend\n");
        let cop = Encoding;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_format_string_pass() {
        let source = test_source("format('%d', x)\n");
        let cop = FormatString;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_format_string_fail() {
        let source = test_source("sprintf('%d', x)\n");
        let cop = FormatString;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_global_vars_pass() {
        let source = test_source("$stdout.puts 'hi'\n");
        let cop = GlobalVars;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_global_vars_fail() {
        let source = test_source("$my_global = 1\n");
        let cop = GlobalVars;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_guard_clause_pass() {
        let source = test_source("def foo\n  return if x\n  bar\nend\n");
        let cop = GuardClause;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_guard_clause_fail() {
        let source = test_source("def foo\n  if x\n    bar\n  end\nend\n");
        let cop = GuardClause;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_identical_conditional_branches_pass() {
        let source = test_source("if x\n  a\nelse\n  b\nend\n");
        let cop = IdenticalConditionalBranches;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_identical_conditional_branches_fail() {
        let source = test_source("if x\n  foo\nelse\n  foo\nend\n");
        let cop = IdenticalConditionalBranches;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_if_inside_else_pass() {
        let source = test_source("if x\n  a\nelsif y\n  b\nend\n");
        let cop = IfInsideElse;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_if_inside_else_fail() {
        let source = test_source("if x\n  a\nelse\n  if y\n    b\n  end\nend\n");
        let cop = IfInsideElse;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_if_unless_modifier_pass() {
        let source = test_source("foo if x\n");
        let cop = IfUnlessModifier;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_if_unless_modifier_fail() {
        let source = test_source("if x\n  foo\nend\n");
        let cop = IfUnlessModifier;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_if_with_boolean_literal_branches_pass() {
        let source = test_source("x ? foo : bar\n");
        let cop = IfWithBooleanLiteralBranches;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_if_with_boolean_literal_branches_fail() {
        let source = test_source("if x then true else false\n");
        let cop = IfWithBooleanLiteralBranches;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_if_with_semicolon_pass() {
        let source = test_source("if x\n  y\nend\n");
        let cop = IfWithSemicolon;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_if_with_semicolon_fail() {
        let source = test_source("if x; y; end\n");
        let cop = IfWithSemicolon;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_inline_comment_pass() {
        let source = test_source("# comment\nfoo\n");
        let cop = InlineComment;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_inline_comment_fail() {
        let source = test_source("foo # comment\n");
        let cop = InlineComment;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_magic_comment_format_pass() {
        let source = test_source("# frozen_string_literal: true\n");
        let cop = MagicCommentFormat;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_magic_comment_format_fail() {
        let source = test_source("#coding: utf-8\n");
        let cop = MagicCommentFormat;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_method_call_without_args_parentheses_pass() {
        let source = test_source("foo.bar\n");
        let cop = MethodCallWithoutArgsParentheses;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_method_call_without_args_parentheses_fail() {
        let source = test_source("foo.bar()\n");
        let cop = MethodCallWithoutArgsParentheses;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_method_def_parentheses_pass() {
        let source = test_source("def foo()\nend\n");
        let cop = MethodDefParentheses;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_method_def_parentheses_fail() {
        let source = test_source("def foo\nend\n");
        let cop = MethodDefParentheses;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_mixin_usage_pass() {
        let source = test_source("class Foo\n  include Bar\n  def x\n  end\nend\n");
        let cop = MixinUsage;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_mixin_usage_fail() {
        let source = test_source("class Foo\n  def x\n  end\n  def y\n  end\n  def z\n  end\n  def a\n  end\n  def b\n  end\n  def c\n  end\n  include Bar\nend\n");
        let cop = MixinUsage;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_multiline_if_then_pass() {
        let source = test_source("if x\n  y\nend\n");
        let cop = MultilineIfThen;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_multiline_if_then_fail() {
        let source = test_source("if x then\n  y\nend\n");
        let cop = MultilineIfThen;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_multiline_ternary_operator_pass() {
        let source = test_source("x ? y : z\n");
        let cop = MultilineTernaryOperator;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_multiline_ternary_operator_fail() {
        let source = test_source("x ?\n  y : z\n");
        let cop = MultilineTernaryOperator;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_multiline_when_then_pass() {
        let source = test_source("case x\nwhen 1\n  y\nend\n");
        let cop = MultilineWhenThen;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_multiline_when_then_fail() {
        let source = test_source("case x\nwhen 1 then\n  y\nend\n");
        let cop = MultilineWhenThen;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_mutable_constant_pass() {
        let source = test_source("FOO = [1, 2].freeze\n");
        let cop = MutableConstant;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_mutable_constant_fail() {
        let source = test_source("FOO = [1, 2]\n");
        let cop = MutableConstant;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_nested_modifier_pass() {
        let source = test_source("foo if x\n");
        let cop = NestedModifier;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_nested_modifier_fail() {
        let source = test_source("foo if x if y\n");
        let cop = NestedModifier;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_nested_ternary_operator_pass() {
        let source = test_source("x ? y : z\n");
        let cop = NestedTernaryOperator;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_nested_ternary_operator_fail() {
        let source = test_source("x ? y ? a : b : z\n");
        let cop = NestedTernaryOperator;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_next_pass() {
        let source = test_source("[1,2].each do |x|\n  next if x == 1\n  puts x\nend\n");
        let cop = Next;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_next_fail() {
        let source = test_source("[1,2].each do |x|\n  if x == 1\n    puts x\n  end\nend\n");
        let cop = Next;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_non_nil_check_pass() {
        let source = test_source("!x.nil?\n");
        let cop = NonNilCheck;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_non_nil_check_fail() {
        let source = test_source("x != nil\n");
        let cop = NonNilCheck;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_numeric_literal_prefix_pass() {
        let source = test_source("x = 0o755\n");
        let cop = NumericLiteralPrefix;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_numeric_literal_prefix_fail() {
        let source = test_source("x = 0755\n");
        let cop = NumericLiteralPrefix;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_one_line_conditional_pass() {
        let source = test_source("x ? y : z\n");
        let cop = OneLineConditional;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_one_line_conditional_fail() {
        let source = test_source("if x\n  y\nelse\n  z\nend\n");
        let cop = OneLineConditional;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_optional_boolean_parameter_pass() {
        let source = test_source("def foo(bar = nil)\nend\n");
        let cop = OptionalBooleanParameter;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_optional_boolean_parameter_fail() {
        let source = test_source("def foo(bar = true)\nend\n");
        let cop = OptionalBooleanParameter;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_or_assignment_pass() {
        let source = test_source("x ||= 1\n");
        let cop = OrAssignment;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_or_assignment_fail() {
        let source = test_source("x = x || 1\n");
        let cop = OrAssignment;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_parentheses_around_condition_pass() {
        let source = test_source("if x == 1\nend\n");
        let cop = ParenthesesAroundCondition;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_parentheses_around_condition_fail() {
        let source = test_source("if (x == 1)\nend\n");
        let cop = ParenthesesAroundCondition;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_percent_literal_delimiters_pass() {
        let source = test_source("%w(foo bar)\n");
        let cop = PercentLiteralDelimiters;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_percent_literal_delimiters_fail() {
        let source = test_source("%w{foo bar}\n");
        let cop = PercentLiteralDelimiters;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_percent_q_literals_pass() {
        let source = test_source("'hello'\n");
        let cop = PercentQLiterals;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_percent_q_literals_fail() {
        let source = test_source("%q(hello)\n");
        let cop = PercentQLiterals;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_perl_backrefs_pass() {
        let source = test_source("match[:name]\n");
        let cop = PerlBackrefs;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_perl_backrefs_fail() {
        let source = test_source("puts $1\n");
        let cop = PerlBackrefs;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_raise_args_pass() {
        let source = test_source("raise StandardError, 'error'\n");
        let cop = RaiseArgs;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_raise_args_fail() {
        let source = test_source("raise StandardError.new('error')\n");
        let cop = RaiseArgs;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_redundant_begin_pass() {
        let source = test_source("def foo\n  bar\nrescue\n  baz\nend\n");
        let cop = RedundantBegin;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_redundant_begin_fail() {
        let source = test_source("def foo\n  begin\n    bar\n  end\nend\n");
        let cop = RedundantBegin;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_redundant_exception_pass() {
        let source = test_source("raise 'error'\n");
        let cop = RedundantException;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_redundant_exception_fail() {
        let source = test_source("raise RuntimeError, 'error'\n");
        let cop = RedundantException;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_redundant_freeze_pass() {
        let source = test_source("[1, 2].freeze\n");
        let cop = RedundantFreeze;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_redundant_freeze_fail() {
        let source = test_source("42.freeze\n");
        let cop = RedundantFreeze;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_redundant_interpolation_pass() {
        let source = test_source("\"hello #{name} world\"\n");
        let cop = RedundantInterpolation;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_redundant_interpolation_fail() {
        let source = test_source("\"#{name}\"\n");
        let cop = RedundantInterpolation;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_redundant_parentheses_pass() {
        let source = test_source("return x\n");
        let cop = RedundantParentheses;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_redundant_parentheses_fail() {
        let source = test_source("return (x)\n");
        let cop = RedundantParentheses;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_redundant_percent_q_pass() {
        let source = test_source("'hello'\n");
        let cop = RedundantPercentQ;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_redundant_percent_q_fail() {
        let source = test_source("%q(hello)\n");
        let cop = RedundantPercentQ;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_redundant_self_pass() {
        let source = test_source("def self.foo\nend\n");
        let cop = RedundantSelf;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_redundant_self_fail() {
        let source = test_source("self.foo\n");
        let cop = RedundantSelf;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_regexp_literal_pass() {
        let source = test_source("/regex/\n");
        let cop = RegexpLiteral;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_regexp_literal_fail() {
        let source = test_source("%r{regex}\n");
        let cop = RegexpLiteral;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_rescue_standard_error_pass() {
        let source = test_source("rescue StandardError\n");
        let cop = RescueStandardError;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_rescue_standard_error_fail() {
        let source = test_source("rescue\n");
        let cop = RescueStandardError;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_safe_navigation_pass() {
        let source = test_source("obj&.method\n");
        let cop = SafeNavigation;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_safe_navigation_fail() {
        let source = test_source("obj && obj.method\n");
        let cop = SafeNavigation;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_single_line_methods_pass() {
        let source = test_source("def foo\n  bar\nend\n");
        let cop = SingleLineMethods;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_single_line_methods_fail() {
        let source = test_source("def foo; bar; end\n");
        let cop = SingleLineMethods;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_special_global_vars_pass() {
        let source = test_source("$LOAD_PATH << 'lib'\n");
        let cop = SpecialGlobalVars;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_special_global_vars_fail() {
        let source = test_source("puts $:\n");
        let cop = SpecialGlobalVars;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_struct_inheritance_pass() {
        let source = test_source("Foo = Struct.new(:name)\n");
        let cop = StructInheritance;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_struct_inheritance_fail() {
        let source = test_source("class Foo < Struct.new(:name)\nend\n");
        let cop = StructInheritance;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_symbol_array_pass() {
        let source = test_source("%i[foo bar]\n");
        let cop = SymbolArray;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_symbol_array_fail() {
        let source = test_source("[:foo, :bar, :baz]\n");
        let cop = SymbolArray;
        assert_eq!(cop.check(&source).len(), 1);
    }

    #[test]
    fn test_ternary_parentheses_pass() {
        let source = test_source("x ? y : z\n");
        let cop = TernaryParentheses;
        assert_eq!(cop.check(&source).len(), 0);
    }

    #[test]
    fn test_ternary_parentheses_fail() {
        let source = test_source("(x) ? y : z\n");
        let cop = TernaryParentheses;
        assert_eq!(cop.check(&source).len(), 1);
    }
}
