//! Metrics and Security cops for Ruby code.

use regex::Regex;
use once_cell::sync::Lazy;

use crate::cop::{Category, Cop, Severity};
use crate::offense::{Location, Offense};
use crate::source::SourceFile;

// ============================================================================
// REGEX PATTERNS
// ============================================================================

static METHOD_DEF_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"^\s*def\s+"#).unwrap()
});

static CLASS_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"^\s*class\s+"#).unwrap()
});

static MODULE_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"^\s*module\s+"#).unwrap()
});

static END_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"^\s*end\s*$"#).unwrap()
});

static BLOCK_START_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(\bdo\b|\{)"#).unwrap()
});

static BLOCK_END_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(\bend\b|\})"#).unwrap()
});

static EVAL_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"\beval\s*\("#).unwrap()
});

static OPEN_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"\bopen\s*\("#).unwrap()
});

static YAML_LOAD_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"\bYAML\.load\b"#).unwrap()
});

static JSON_LOAD_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"\bJSON\.load\b"#).unwrap()
});

static MARSHAL_LOAD_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"\bMarshal\.load\b"#).unwrap()
});

// ============================================================================
// METRICS COPS
// ============================================================================

/// Checks method length
pub struct MethodLength {
    max_lines: usize,
}

impl MethodLength {
    pub fn new() -> Self {
        Self { max_lines: 10 }
    }

    pub fn with_max_lines(max_lines: usize) -> Self {
        Self { max_lines }
    }
}

impl Default for MethodLength {
    fn default() -> Self {
        Self::new()
    }
}

impl Cop for MethodLength {
    fn name(&self) -> &str { "Metrics/MethodLength" }
    fn category(&self) -> Category { Category::Metrics }
    fn severity(&self) -> Severity { Severity::Convention }
    fn description(&self) -> &str {
        "Methods should not be too long"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let mut method_start: Option<usize> = None;
        let mut depth = 0;

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;

            if METHOD_DEF_PATTERN.is_match(line) && !source.in_string_or_comment(line_number, 1) {
                if depth == 0 {
                    method_start = Some(line_number);
                }
                depth += 1;
            }

            if END_PATTERN.is_match(line) && !source.in_string_or_comment(line_number, 1) {
                depth -= 1;
                if depth == 0 {
                    if let Some(start) = method_start {
                        let length = line_number - start + 1;
                        if length > self.max_lines {
                            offenses.push(Offense::new(
                                self.name(),
                                format!("Method has {} lines (max {})", length, self.max_lines),
                                self.severity(),
                                Location::new(start, 1, 3),
                            ));
                        }
                        method_start = None;
                    }
                }
            }
        }

        offenses
    }
}

/// Checks class length
pub struct ClassLength {
    max_lines: usize,
}

impl ClassLength {
    pub fn new() -> Self {
        Self { max_lines: 100 }
    }

    pub fn with_max_lines(max_lines: usize) -> Self {
        Self { max_lines }
    }
}

impl Default for ClassLength {
    fn default() -> Self {
        Self::new()
    }
}

impl Cop for ClassLength {
    fn name(&self) -> &str { "Metrics/ClassLength" }
    fn category(&self) -> Category { Category::Metrics }
    fn severity(&self) -> Severity { Severity::Convention }
    fn description(&self) -> &str {
        "Classes should not be too long"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let mut class_start: Option<usize> = None;
        let mut depth = 0;

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;

            if CLASS_PATTERN.is_match(line) && !source.in_string_or_comment(line_number, 1) {
                if depth == 0 {
                    class_start = Some(line_number);
                }
                depth += 1;
            }

            if END_PATTERN.is_match(line) && !source.in_string_or_comment(line_number, 1) {
                depth -= 1;
                if depth == 0 {
                    if let Some(start) = class_start {
                        let length = line_number - start + 1;
                        if length > self.max_lines {
                            offenses.push(Offense::new(
                                self.name(),
                                format!("Class has {} lines (max {})", length, self.max_lines),
                                self.severity(),
                                Location::new(start, 1, 5),
                            ));
                        }
                        class_start = None;
                    }
                }
            }
        }

        offenses
    }
}

/// Checks module length
pub struct ModuleLength {
    max_lines: usize,
}

impl ModuleLength {
    pub fn new() -> Self {
        Self { max_lines: 100 }
    }

    pub fn with_max_lines(max_lines: usize) -> Self {
        Self { max_lines }
    }
}

impl Default for ModuleLength {
    fn default() -> Self {
        Self::new()
    }
}

impl Cop for ModuleLength {
    fn name(&self) -> &str { "Metrics/ModuleLength" }
    fn category(&self) -> Category { Category::Metrics }
    fn severity(&self) -> Severity { Severity::Convention }
    fn description(&self) -> &str {
        "Modules should not be too long"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let mut module_start: Option<usize> = None;
        let mut depth = 0;

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;

            if MODULE_PATTERN.is_match(line) && !source.in_string_or_comment(line_number, 1) {
                if depth == 0 {
                    module_start = Some(line_number);
                }
                depth += 1;
            }

            if END_PATTERN.is_match(line) && !source.in_string_or_comment(line_number, 1) {
                depth -= 1;
                if depth == 0 {
                    if let Some(start) = module_start {
                        let length = line_number - start + 1;
                        if length > self.max_lines {
                            offenses.push(Offense::new(
                                self.name(),
                                format!("Module has {} lines (max {})", length, self.max_lines),
                                self.severity(),
                                Location::new(start, 1, 6),
                            ));
                        }
                        module_start = None;
                    }
                }
            }
        }

        offenses
    }
}

/// Checks block length
pub struct BlockLength {
    max_lines: usize,
}

impl BlockLength {
    pub fn new() -> Self {
        Self { max_lines: 25 }
    }

    pub fn with_max_lines(max_lines: usize) -> Self {
        Self { max_lines }
    }
}

impl Default for BlockLength {
    fn default() -> Self {
        Self::new()
    }
}

impl Cop for BlockLength {
    fn name(&self) -> &str { "Metrics/BlockLength" }
    fn category(&self) -> Category { Category::Metrics }
    fn severity(&self) -> Severity { Severity::Convention }
    fn description(&self) -> &str {
        "Blocks should not be too long"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let mut block_starts: Vec<usize> = Vec::new();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;

            if BLOCK_START_PATTERN.is_match(line) && !source.in_string_or_comment(line_number, 1) {
                block_starts.push(line_number);
            }

            if BLOCK_END_PATTERN.is_match(line) && !source.in_string_or_comment(line_number, 1) {
                if let Some(start) = block_starts.pop() {
                    let length = line_number - start + 1;
                    if length > self.max_lines {
                        offenses.push(Offense::new(
                            self.name(),
                            format!("Block has {} lines (max {})", length, self.max_lines),
                            self.severity(),
                            Location::new(start, 1, 2),
                        ));
                    }
                }
            }
        }

        offenses
    }
}

/// Checks block nesting depth
pub struct BlockNesting {
    max_depth: usize,
}

impl BlockNesting {
    pub fn new() -> Self {
        Self { max_depth: 3 }
    }

    pub fn with_max_depth(max_depth: usize) -> Self {
        Self { max_depth }
    }
}

impl Default for BlockNesting {
    fn default() -> Self {
        Self::new()
    }
}

impl Cop for BlockNesting {
    fn name(&self) -> &str { "Metrics/BlockNesting" }
    fn category(&self) -> Category { Category::Metrics }
    fn severity(&self) -> Severity { Severity::Convention }
    fn description(&self) -> &str {
        "Blocks should not be nested too deeply"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let mut depth = 0;

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;

            if BLOCK_START_PATTERN.is_match(line) && !source.in_string_or_comment(line_number, 1) {
                depth += 1;
                if depth > self.max_depth {
                    offenses.push(Offense::new(
                        self.name(),
                        format!("Block nesting depth is {} (max {})", depth, self.max_depth),
                        self.severity(),
                        Location::new(line_number, 1, line.len()),
                    ));
                }
            }

            if BLOCK_END_PATTERN.is_match(line) && !source.in_string_or_comment(line_number, 1) {
                if depth > 0 {
                    depth -= 1;
                }
            }
        }

        offenses
    }
}

/// Checks parameter list length
pub struct ParameterLists {
    max_params: usize,
}

impl ParameterLists {
    pub fn new() -> Self {
        Self { max_params: 5 }
    }

    pub fn with_max_params(max_params: usize) -> Self {
        Self { max_params }
    }
}

impl Default for ParameterLists {
    fn default() -> Self {
        Self::new()
    }
}

impl Cop for ParameterLists {
    fn name(&self) -> &str { "Metrics/ParameterLists" }
    fn category(&self) -> Category { Category::Metrics }
    fn severity(&self) -> Severity { Severity::Convention }
    fn description(&self) -> &str {
        "Methods should not have too many parameters"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let pattern = Regex::new(r#"def\s+\w+\(([^)]+)\)"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;

            if let Some(cap) = pattern.captures(line) {
                if source.in_string_or_comment(line_number, 1) {
                    continue;
                }
                let params = cap.get(1).unwrap().as_str();
                let param_count = params.split(',').count();
                
                if param_count > self.max_params {
                    offenses.push(Offense::new(
                        self.name(),
                        format!("Method has {} parameters (max {})", param_count, self.max_params),
                        self.severity(),
                        Location::new(line_number, 1, 3),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks cyclomatic complexity (simplified)
pub struct CyclomaticComplexity {
    max_complexity: usize,
}

impl CyclomaticComplexity {
    pub fn new() -> Self {
        Self { max_complexity: 10 }
    }

    pub fn with_max_complexity(max_complexity: usize) -> Self {
        Self { max_complexity }
    }
}

impl Default for CyclomaticComplexity {
    fn default() -> Self {
        Self::new()
    }
}

impl Cop for CyclomaticComplexity {
    fn name(&self) -> &str { "Metrics/CyclomaticComplexity" }
    fn category(&self) -> Category { Category::Metrics }
    fn severity(&self) -> Severity { Severity::Convention }
    fn description(&self) -> &str {
        "Methods should not be too complex"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let keywords = ["if", "unless", "while", "until", "for", "rescue", "when", "&&", "||"];
        let pattern = Regex::new(&format!(r"\b({})\b", keywords.join("|"))).unwrap();

        let mut method_start: Option<usize> = None;
        let mut complexity = 1;
        let mut depth = 0;

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;

            if METHOD_DEF_PATTERN.is_match(line) && !source.in_string_or_comment(line_number, 1) {
                if depth == 0 {
                    method_start = Some(line_number);
                    complexity = 1;
                }
                depth += 1;
            }

            if depth > 0 {
                complexity += pattern.find_iter(line).count();
            }

            if END_PATTERN.is_match(line) && !source.in_string_or_comment(line_number, 1) {
                depth -= 1;
                if depth == 0 {
                    if let Some(start) = method_start {
                        if complexity > self.max_complexity {
                            offenses.push(Offense::new(
                                self.name(),
                                format!("Cyclomatic complexity is {} (max {})", complexity, self.max_complexity),
                                self.severity(),
                                Location::new(start, 1, 3),
                            ));
                        }
                        method_start = None;
                    }
                }
            }
        }

        offenses
    }
}

/// Checks ABC size (simplified)
pub struct AbcSize {
    max_size: usize,
}

impl AbcSize {
    pub fn new() -> Self {
        Self { max_size: 20 }
    }

    pub fn with_max_size(max_size: usize) -> Self {
        Self { max_size }
    }
}

impl Default for AbcSize {
    fn default() -> Self {
        Self::new()
    }
}

impl Cop for AbcSize {
    fn name(&self) -> &str { "Metrics/AbcSize" }
    fn category(&self) -> Category { Category::Metrics }
    fn severity(&self) -> Severity { Severity::Convention }
    fn description(&self) -> &str {
        "Methods should not have high ABC size"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let assignment_pattern = Regex::new(r#"[=]"#).unwrap();
        let branch_pattern = Regex::new(r#"\b(if|unless|case|while|until)\b"#).unwrap();
        let condition_pattern = Regex::new(r#"(&&|\|\|)"#).unwrap();

        let mut method_start: Option<usize> = None;
        let mut abc_score = 0;
        let mut depth = 0;

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;

            if METHOD_DEF_PATTERN.is_match(line) && !source.in_string_or_comment(line_number, 1) {
                if depth == 0 {
                    method_start = Some(line_number);
                    abc_score = 0;
                }
                depth += 1;
            }

            if depth > 0 {
                abc_score += assignment_pattern.find_iter(line).count();
                abc_score += branch_pattern.find_iter(line).count();
                abc_score += condition_pattern.find_iter(line).count();
            }

            if END_PATTERN.is_match(line) && !source.in_string_or_comment(line_number, 1) {
                depth -= 1;
                if depth == 0 {
                    if let Some(start) = method_start {
                        if abc_score > self.max_size {
                            offenses.push(Offense::new(
                                self.name(),
                                format!("ABC size is {} (max {})", abc_score, self.max_size),
                                self.severity(),
                                Location::new(start, 1, 3),
                            ));
                        }
                        method_start = None;
                    }
                }
            }
        }

        offenses
    }
}

/// Checks collection literal length
pub struct CollectionLiteralLength {
    max_length: usize,
}

impl CollectionLiteralLength {
    pub fn new() -> Self {
        Self { max_length: 20 }
    }

    pub fn with_max_length(max_length: usize) -> Self {
        Self { max_length }
    }
}

impl Default for CollectionLiteralLength {
    fn default() -> Self {
        Self::new()
    }
}

impl Cop for CollectionLiteralLength {
    fn name(&self) -> &str { "Metrics/CollectionLiteralLength" }
    fn category(&self) -> Category { Category::Metrics }
    fn severity(&self) -> Severity { Severity::Convention }
    fn description(&self) -> &str {
        "Collection literals should not be too long"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let pattern = Regex::new(r#"[\[\{]\s*([^[\]{}]+)\s*[\]\}]"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;

            for cap in pattern.captures_iter(line) {
                if source.in_string_or_comment(line_number, 1) {
                    continue;
                }
                let content = cap.get(1).unwrap().as_str();
                let count = content.split(',').count();
                
                if count > self.max_length {
                    let full_match = cap.get(0).unwrap();
                    offenses.push(Offense::new(
                        self.name(),
                        format!("Collection literal has {} elements (max {})", count, self.max_length),
                        self.severity(),
                        Location::new(line_number, full_match.start() + 1, full_match.len()),
                    ));
                }
            }
        }

        offenses
    }
}

// ============================================================================
// SECURITY COPS (using Lint category as Security doesn't exist in enum)
// ============================================================================

/// Detects eval usage
pub struct Eval;

impl Cop for Eval {
    fn name(&self) -> &str { "Security/Eval" }
    fn category(&self) -> Category { Category::Lint }
    fn severity(&self) -> Severity { Severity::Warning }
    fn description(&self) -> &str {
        "Do not use eval - it's a security risk"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in EVAL_PATTERN.find_iter(line) {
                let column = mat.start() + 1;
                if source.in_string_or_comment(line_number, column) {
                    continue;
                }
                offenses.push(Offense::new(
                    self.name(),
                    "Avoid using `eval` - it's a security risk.",
                    self.severity(),
                    Location::new(line_number, column, mat.len()),
                ));
            }
        }
        offenses
    }
}

/// Detects open usage (command injection risk)
pub struct Open;

impl Cop for Open {
    fn name(&self) -> &str { "Security/Open" }
    fn category(&self) -> Category { Category::Lint }
    fn severity(&self) -> Severity { Severity::Warning }
    fn description(&self) -> &str {
        "Using open with user input can lead to command injection"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in OPEN_PATTERN.find_iter(line) {
                let column = mat.start() + 1;
                if source.in_string_or_comment(line_number, column) {
                    continue;
                }
                offenses.push(Offense::new(
                    self.name(),
                    "Using `open` can lead to command injection. Use `File.open` or `URI.open`.",
                    self.severity(),
                    Location::new(line_number, column, mat.len()),
                ));
            }
        }
        offenses
    }
}

/// Detects YAML.load usage
pub struct YAMLLoad;

impl Cop for YAMLLoad {
    fn name(&self) -> &str { "Security/YAMLLoad" }
    fn category(&self) -> Category { Category::Lint }
    fn severity(&self) -> Severity { Severity::Warning }
    fn description(&self) -> &str {
        "Use YAML.safe_load instead of YAML.load"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in YAML_LOAD_PATTERN.find_iter(line) {
                let column = mat.start() + 1;
                if source.in_string_or_comment(line_number, column) {
                    continue;
                }
                if !line.contains("safe_load") {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use `YAML.safe_load` instead of `YAML.load`.",
                        self.severity(),
                        Location::new(line_number, column, mat.len()),
                    ));
                }
            }
        }
        offenses
    }
}

/// Detects JSON.load usage
pub struct JSONLoad;

impl Cop for JSONLoad {
    fn name(&self) -> &str { "Security/JSONLoad" }
    fn category(&self) -> Category { Category::Lint }
    fn severity(&self) -> Severity { Severity::Warning }
    fn description(&self) -> &str {
        "Use JSON.parse instead of JSON.load"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in JSON_LOAD_PATTERN.find_iter(line) {
                let column = mat.start() + 1;
                if source.in_string_or_comment(line_number, column) {
                    continue;
                }
                offenses.push(Offense::new(
                    self.name(),
                    "Use `JSON.parse` instead of `JSON.load`.",
                    self.severity(),
                    Location::new(line_number, column, mat.len()),
                ));
            }
        }
        offenses
    }
}

/// Detects Marshal.load usage
pub struct MarshalLoad;

impl Cop for MarshalLoad {
    fn name(&self) -> &str { "Security/MarshalLoad" }
    fn category(&self) -> Category { Category::Lint }
    fn severity(&self) -> Severity { Severity::Warning }
    fn description(&self) -> &str {
        "Marshal.load is unsafe with untrusted data"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for mat in MARSHAL_LOAD_PATTERN.find_iter(line) {
                let column = mat.start() + 1;
                if source.in_string_or_comment(line_number, column) {
                    continue;
                }
                offenses.push(Offense::new(
                    self.name(),
                    "`Marshal.load` is unsafe with untrusted data.",
                    self.severity(),
                    Location::new(line_number, column, mat.len()),
                ));
            }
        }
        offenses
    }
}

/// Detects IO security methods (simplified)
pub struct IoMethods;

impl Cop for IoMethods {
    fn name(&self) -> &str { "Security/IoMethods" }
    fn category(&self) -> Category { Category::Lint }
    fn severity(&self) -> Severity { Severity::Warning }
    fn description(&self) -> &str {
        "Be careful with IO methods that execute shell commands"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let dangerous_methods = ["system", "exec", "spawn", "`"];
        
        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            for method in &dangerous_methods {
                let pattern = Regex::new(&format!(r"\b{}\b", regex::escape(method))).unwrap();
                for mat in pattern.find_iter(line) {
                    let column = mat.start() + 1;
                    if source.in_string_or_comment(line_number, column) {
                        continue;
                    }
                    offenses.push(Offense::new(
                        self.name(),
                        format!("Be careful with `{}` - validate inputs to prevent command injection.", method),
                        self.severity(),
                        Location::new(line_number, column, mat.len()),
                    ));
                }
            }
        }
        offenses
    }
}

/// Detects compound hash issues (simplified)
pub struct CompoundHash;

impl Cop for CompoundHash {
    fn name(&self) -> &str { "Security/CompoundHash" }
    fn category(&self) -> Category { Category::Lint }
    fn severity(&self) -> Severity { Severity::Warning }
    fn description(&self) -> &str {
        "Be careful with compound hash usage"
    }

    fn check(&self, _source: &SourceFile) -> Vec<Offense> {
        // Simplified - would need deep analysis
        Vec::new()
    }
}

// ============================================================================
// TESTS
// ============================================================================


// Collect all cops from this module
pub fn all_metrics_cops() -> Vec<Box<dyn Cop>> {
    vec![
        Box::new(MethodLength::default()),
        Box::new(ClassLength::default()),
        Box::new(ModuleLength::default()),
        Box::new(BlockLength::default()),
        Box::new(BlockNesting::default()),
        Box::new(ParameterLists::default()),
        Box::new(CyclomaticComplexity::default()),
        Box::new(AbcSize::default()),
        Box::new(CollectionLiteralLength::default()),
        Box::new(Eval),
        Box::new(Open),
        Box::new(YAMLLoad),
        Box::new(JSONLoad),
        Box::new(MarshalLoad),
        Box::new(IoMethods),
        Box::new(CompoundHash),
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
    fn test_method_length() {
        let cop = MethodLength::new();
        let source = test_source("def foo\n  a = 1\n  b = 2\n  c = 3\n  d = 4\n  e = 5\n  f = 6\n  g = 7\n  h = 8\n  i = 9\n  j = 10\n  k = 11\nend\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_method_length_ok() {
        let cop = MethodLength::new();
        let source = test_source("def foo\n  1 + 1\nend\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_class_length() {
        let cop = ClassLength::with_max_lines(5);
        let mut code = String::from("class Foo\n");
        for i in 0..10 {
            code.push_str(&format!("  def method{}\n  end\n", i));
        }
        code.push_str("end\n");
        let source = test_source(&code);
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_parameter_lists() {
        let cop = ParameterLists::new();
        let source = test_source("def foo(a, b, c, d, e, f)\nend\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }

    #[test]
    fn test_parameter_lists_ok() {
        let cop = ParameterLists::new();
        let source = test_source("def foo(a, b, c)\nend\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_eval() {
        let cop = Eval;
        let source = test_source("eval(user_input)\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert!(offenses[0].message.contains("security"));
    }

    #[test]
    fn test_open() {
        let cop = Open;
        let source = test_source("open(filename)\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert!(offenses[0].message.contains("injection"));
    }

    #[test]
    fn test_yaml_load() {
        let cop = YAMLLoad;
        let source = test_source("data = YAML.load(file)\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert!(offenses[0].message.contains("safe_load"));
    }

    #[test]
    fn test_json_load() {
        let cop = JSONLoad;
        let source = test_source("data = JSON.load(file)\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert!(offenses[0].message.contains("parse"));
    }

    #[test]
    fn test_marshal_load() {
        let cop = MarshalLoad;
        let source = test_source("data = Marshal.load(file)\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert!(offenses[0].message.contains("unsafe"));
    }

    #[test]
    fn test_block_nesting() {
        let cop = BlockNesting::new();
        let source = test_source("items.each do |i|\n  i.map do |j|\n    j.select do |k|\n      k.each do |l|\n        puts l\n      end\n    end\n  end\nend\n");
        let offenses = cop.check(&source);
        assert!(offenses.len() >= 1);
    }

    #[test]
    fn test_cyclomatic_complexity() {
        let cop = CyclomaticComplexity::with_max_complexity(3);
        let source = test_source("def foo\n  if a\n    if b\n      if c\n        if d\n          x\n        end\n      end\n    end\n  end\nend\n");
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
    }
}
