//! Bundler and Gemspec cops for Gemfile and gemspec file conventions.

use regex::Regex;
use std::collections::HashMap;

use crate::cop::{Category, Cop, Severity};
use crate::offense::{Location, Offense};
use crate::source::SourceFile;

// ==================== BUNDLER COPS ====================

/// Detects duplicate gem declarations in Gemfile.
///
/// This cop ensures that each gem is only declared once in the Gemfile.
pub struct DuplicatedGem;

impl Cop for DuplicatedGem {
    fn name(&self) -> &str {
        "Bundler/DuplicatedGem"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Detects duplicate gem declarations in Gemfile"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let mut seen_gems: HashMap<String, usize> = HashMap::new();

        let gem_regex = Regex::new(r#"^\s*gem\s+['"]([^'"]+)['"]"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;

            if let Some(capture) = gem_regex.captures(line) {
                if let Some(gem_name) = capture.get(1) {
                    let name = gem_name.as_str().to_string();
                    
                    if let Some(&first_line) = seen_gems.get(&name) {
                        let col = capture.get(0).unwrap().start() + 1;
                        offenses.push(Offense::new(
                            self.name(),
                            format!("Gem '{}' is declared multiple times (first seen on line {})", name, first_line),
                            self.severity(),
                            Location::new(line_number, col, line.len()),
                        ));
                    } else {
                        seen_gems.insert(name, line_number);
                    }
                }
            }
        }

        offenses
    }
}

/// Detects duplicate group declarations in Gemfile.
///
/// This cop ensures that each group is only declared once in the Gemfile.
pub struct DuplicatedGroup;

impl Cop for DuplicatedGroup {
    fn name(&self) -> &str {
        "Bundler/DuplicatedGroup"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Detects duplicate group declarations in Gemfile"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let mut seen_groups: HashMap<String, usize> = HashMap::new();

        let group_regex = Regex::new(r#"^\s*group\s+:([a-z_]+)"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;

            if let Some(capture) = group_regex.captures(line) {
                if let Some(group_name) = capture.get(1) {
                    let name = group_name.as_str().to_string();
                    
                    if let Some(&first_line) = seen_groups.get(&name) {
                        let col = capture.get(0).unwrap().start() + 1;
                        offenses.push(Offense::new(
                            self.name(),
                            format!("Group '{}' is declared multiple times (first seen on line {})", name, first_line),
                            self.severity(),
                            Location::new(line_number, col, line.len()),
                        ));
                    } else {
                        seen_groups.insert(name, line_number);
                    }
                }
            }
        }

        offenses
    }
}

/// Checks that gems in Gemfile have explanatory comments.
///
/// This cop encourages documenting why each gem is included.
pub struct GemComment;

impl Cop for GemComment {
    fn name(&self) -> &str {
        "Bundler/GemComment"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks that gems have explanatory comments"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let gem_regex = Regex::new(r#"^\s*gem\s+['"]"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;

            if gem_regex.is_match(line) {
                // Check if previous line is a comment
                let has_comment = if line_number > 1 {
                    if let Some(prev_line) = source.line(line_number - 1) {
                        prev_line.trim().starts_with('#')
                    } else {
                        false
                    }
                } else {
                    false
                };

                // Check if same line has comment
                let same_line_comment = line.contains('#');

                if !has_comment && !same_line_comment {
                    let col = gem_regex.find(line).unwrap().start() + 1;
                    offenses.push(Offense::new(
                        self.name(),
                        "Missing gem description comment",
                        self.severity(),
                        Location::new(line_number, col, 3),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks Gemfile naming convention.
///
/// This cop ensures the file is named "Gemfile" not "gemfile" or other variants.
pub struct GemFilename;

impl Cop for GemFilename {
    fn name(&self) -> &str {
        "Bundler/GemFilename"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks Gemfile naming convention"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        if let Some(filename) = source.path.file_name() {
            if let Some(name_str) = filename.to_str() {
                // Check if this is a Gemfile variant but with wrong casing
                if name_str.to_lowercase().starts_with("gemfile") && name_str != "Gemfile" && !name_str.starts_with("Gemfile.") {
                    offenses.push(Offense::new(
                        self.name(),
                        format!("Gemfile should be named 'Gemfile', not '{}'", name_str),
                        self.severity(),
                        Location::new(1, 1, 1),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks for gem version specifications.
///
/// This cop encourages specifying gem versions for better dependency management.
pub struct GemVersion;

impl Cop for GemVersion {
    fn name(&self) -> &str {
        "Bundler/GemVersion"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for gem version specifications"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        // Match gem declarations: gem 'name' or gem "name" without version
        let gem_no_version_regex = Regex::new(r#"^\s*gem\s+['"]([^'"]+)['"]\s*$"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;

            if let Some(capture) = gem_no_version_regex.captures(line) {
                if let Some(gem_name) = capture.get(1) {
                    let col = capture.get(0).unwrap().start() + 1;
                    offenses.push(Offense::new(
                        self.name(),
                        format!("Gem '{}' should have a version constraint", gem_name.as_str()),
                        self.severity(),
                        Location::new(line_number, col, line.trim().len()),
                    ));
                }
            }
        }

        offenses
    }
}

/// Detects insecure protocol sources in Gemfile.
///
/// This cop flags http:// sources and recommends using https://.
pub struct InsecureProtocolSource;

impl Cop for InsecureProtocolSource {
    fn name(&self) -> &str {
        "Bundler/InsecureProtocolSource"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Detects insecure protocol sources in Gemfile"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        let http_regex = Regex::new(r#"['"]http://[^'"]*['"]"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;

            // Look for source or git declarations with http://
            if (line.contains("source") || line.contains("git")) && line.contains("http://") {
                if let Some(matched) = http_regex.find(line) {
                    let col = matched.start() + 1;
                    offenses.push(Offense::new(
                        self.name(),
                        "Use https:// instead of insecure http:// for gem sources",
                        self.severity(),
                        Location::new(line_number, col, matched.len()),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks that gems are alphabetically ordered within groups.
///
/// This cop enforces alphabetical ordering of gems for better maintainability.
pub struct OrderedGems;

impl Cop for OrderedGems {
    fn name(&self) -> &str {
        "Bundler/OrderedGems"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks that gems are alphabetically ordered"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let gem_regex = Regex::new(r#"^\s*gem\s+['"]([^'"]+)['"]"#).unwrap();

        let mut last_gem: Option<(String, usize)> = None;

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;

            // Reset on group boundaries
            if line.trim().starts_with("group ") || line.trim() == "end" {
                last_gem = None;
                continue;
            }

            if let Some(capture) = gem_regex.captures(line) {
                if let Some(gem_name) = capture.get(1) {
                    let name = gem_name.as_str().to_string();
                    
                    if let Some((last_name, _)) = &last_gem {
                        if name < *last_name {
                            let col = capture.get(0).unwrap().start() + 1;
                            offenses.push(Offense::new(
                                self.name(),
                                format!("Gem '{}' should be sorted before '{}'", name, last_name),
                                self.severity(),
                                Location::new(line_number, col, line.trim().len()),
                            ));
                        }
                    }
                    
                    last_gem = Some((name, line_number));
                }
            }
        }

        offenses
    }
}

// ==================== GEMSPEC COPS ====================

/// Detects use of add_runtime_dependency instead of add_dependency.
///
/// This cop prefers the shorter add_dependency method.
pub struct AddRuntimeDependency;

impl Cop for AddRuntimeDependency {
    fn name(&self) -> &str {
        "Gemspec/AddRuntimeDependency"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Detects use of add_runtime_dependency instead of add_dependency"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        let runtime_dep_regex = Regex::new(r"\.add_runtime_dependency\b").unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;

            if let Some(matched) = runtime_dep_regex.find(line) {
                if !source.in_string_or_comment(line_number, matched.start() + 1) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use `add_dependency` instead of `add_runtime_dependency`",
                        self.severity(),
                        Location::new(line_number, matched.start() + 1, matched.len()),
                    ));
                }
            }
        }

        offenses
    }
}

/// Detects conditional assignment of spec attributes.
///
/// This cop flags attribute assignments inside conditionals in gemspec files.
pub struct AttributeAssignment;

impl Cop for AttributeAssignment {
    fn name(&self) -> &str {
        "Gemspec/AttributeAssignment"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Detects conditional assignment of spec attributes"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let mut in_conditional = false;
        let mut conditional_depth = 0;

        let conditional_regex = Regex::new(r"^\s*(if|unless|case)\b").unwrap();
        let assignment_regex = Regex::new(r"^\s*\w+\.\w+\s*=").unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;
            let trimmed = line.trim();

            // Track conditional blocks
            if conditional_regex.is_match(line) {
                in_conditional = true;
                conditional_depth += 1;
            }

            if trimmed == "end" && conditional_depth > 0 {
                conditional_depth -= 1;
                if conditional_depth == 0 {
                    in_conditional = false;
                }
            }

            // Check for attribute assignments inside conditionals
            if in_conditional && assignment_regex.is_match(line) {
                if let Some(matched) = assignment_regex.find(line) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Don't assign spec attributes conditionally",
                        self.severity(),
                        Location::new(line_number, matched.start() + 1, matched.len()),
                    ));
                }
            }
        }

        offenses
    }
}

/// Checks for dependency version specifications in gemspec.
///
/// This cop ensures dependencies have version constraints.
pub struct DependencyVersion;

impl Cop for DependencyVersion {
    fn name(&self) -> &str {
        "Gemspec/DependencyVersion"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks for dependency version specifications"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        // Match add_dependency or add_development_dependency with only one argument
        // Supports both add_dependency('name') and add_dependency 'name' syntax
        let dep_no_version_regex = Regex::new(r#"\.add_(development_)?dependency\s+['"]([^'"]+)['"]"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;

            if let Some(capture) = dep_no_version_regex.captures(line) {
                // Check if there's a comma or another argument after the gem name (indicates version follows)
                let match_end = capture.get(0).unwrap().end();
                let rest_of_line = &line[match_end..];

                // If there's a comma or more content that looks like a version, skip
                if !rest_of_line.trim_start().starts_with(',') && !rest_of_line.contains("'") && !rest_of_line.contains('"') {
                    if let Some(dep_name) = capture.get(2) {
                        let col = capture.get(0).unwrap().start() + 1;
                        offenses.push(Offense::new(
                            self.name(),
                            format!("Dependency '{}' should have a version constraint", dep_name.as_str()),
                            self.severity(),
                            Location::new(line_number, col, line.trim().len()),
                        ));
                    }
                }
            }
        }

        offenses
    }
}

/// Detects use of deprecated gemspec attributes.
///
/// This cop flags deprecated attribute assignments like rubyforge_project.
pub struct DeprecatedAttributeAssignment;

impl Cop for DeprecatedAttributeAssignment {
    fn name(&self) -> &str {
        "Gemspec/DeprecatedAttributeAssignment"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Detects use of deprecated gemspec attributes"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        let deprecated_attrs = vec!["rubyforge_project", "date", "specification_version"];
        
        for attr in deprecated_attrs {
            let attr_regex = Regex::new(&format!(r"\.{}\s*=", attr)).unwrap();

            for (line_num, line) in source.lines.iter().enumerate() {
                let line_number = line_num + 1;

                if let Some(matched) = attr_regex.find(line) {
                    if !source.in_string_or_comment(line_number, matched.start() + 1) {
                        offenses.push(Offense::new(
                            self.name(),
                            format!("Deprecated attribute '{}' should not be used", attr),
                            self.severity(),
                            Location::new(line_number, matched.start() + 1, matched.len()),
                        ));
                    }
                }
            }
        }

        offenses
    }
}

/// Checks development dependencies style in gemspec.
///
/// This cop ensures consistent use of add_development_dependency.
pub struct DevelopmentDependencies;

impl Cop for DevelopmentDependencies {
    fn name(&self) -> &str {
        "Gemspec/DevelopmentDependencies"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks development dependencies style"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        // Check for Gemfile-style gem declarations in gemspec
        let gem_regex = Regex::new(r#"^\s*gem\s+['"]"#).unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;

            if source.path.to_string_lossy().ends_with(".gemspec")
                && gem_regex.is_match(line) {
                let col = gem_regex.find(line).unwrap().start() + 1;
                offenses.push(Offense::new(
                    self.name(),
                    "Use `add_development_dependency` instead of `gem` in gemspec",
                    self.severity(),
                    Location::new(line_number, col, 3),
                ));
            }
        }

        offenses
    }
}

/// Detects duplicate assignments in gemspec.
///
/// This cop ensures each spec attribute is only assigned once.
pub struct DuplicatedAssignment;

impl Cop for DuplicatedAssignment {
    fn name(&self) -> &str {
        "Gemspec/DuplicatedAssignment"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Detects duplicate assignments in gemspec"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();
        let mut seen_attrs: HashMap<String, usize> = HashMap::new();

        let assignment_regex = Regex::new(r"^\s*\w+\.([a-z_]+)\s*=").unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;

            if let Some(capture) = assignment_regex.captures(line) {
                if let Some(attr_name) = capture.get(1) {
                    let name = attr_name.as_str().to_string();
                    
                    if let Some(&first_line) = seen_attrs.get(&name) {
                        let col = capture.get(0).unwrap().start() + 1;
                        offenses.push(Offense::new(
                            self.name(),
                            format!("Attribute '{}' is assigned multiple times (first seen on line {})", name, first_line),
                            self.severity(),
                            Location::new(line_number, col, line.trim().len()),
                        ));
                    } else {
                        seen_attrs.insert(name, line_number);
                    }
                }
            }
        }

        offenses
    }
}

/// Checks that dependencies are alphabetically ordered in gemspec.
///
/// This cop enforces alphabetical ordering of dependencies.
pub struct OrderedDependencies;

impl Cop for OrderedDependencies {
    fn name(&self) -> &str {
        "Gemspec/OrderedDependencies"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks that dependencies are alphabetically ordered"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        // Match both add_dependency('name') and add_dependency 'name' syntax
        let dep_regex = Regex::new(r#"\.add_(development_)?dependency\s*\(?\s*['"]([^'"]+)['"]"#).unwrap();

        let mut last_runtime_dep: Option<(String, usize)> = None;
        let mut last_dev_dep: Option<(String, usize)> = None;

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;

            if let Some(capture) = dep_regex.captures(line) {
                let is_dev = capture.get(1).is_some();
                if let Some(dep_name) = capture.get(2) {
                    let name = dep_name.as_str().to_string();

                    if is_dev {
                        if let Some((last_name, _)) = &last_dev_dep {
                            if name < *last_name {
                                let col = capture.get(0).unwrap().start() + 1;
                                offenses.push(Offense::new(
                                    self.name(),
                                    format!("Development dependency '{}' should be sorted before '{}'", name, last_name),
                                    self.severity(),
                                    Location::new(line_number, col, line.trim().len()),
                                ));
                            }
                        }
                        last_dev_dep = Some((name, line_number));
                    } else {
                        if let Some((last_name, _)) = &last_runtime_dep {
                            if name < *last_name {
                                let col = capture.get(0).unwrap().start() + 1;
                                offenses.push(Offense::new(
                                    self.name(),
                                    format!("Runtime dependency '{}' should be sorted before '{}'", name, last_name),
                                    self.severity(),
                                    Location::new(line_number, col, line.trim().len()),
                                ));
                            }
                        }
                        last_runtime_dep = Some((name, line_number));
                    }
                }
            }
        }

        offenses
    }
}

/// Checks that gemspec requires MFA for gem push operations.
///
/// This cop ensures metadata includes 'allowed_push_host' or 'mfa_required'.
pub struct RequireMFA;

impl Cop for RequireMFA {
    fn name(&self) -> &str {
        "Gemspec/RequireMFA"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks that gemspec requires MFA for gem push"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        let mfa_regex = Regex::new(r#"['"]mfa_required['"]"#).unwrap();
        let metadata_regex = Regex::new(r"\.metadata\b").unwrap();

        let mut has_mfa = false;
        let mut has_metadata = false;

        for (line_num, line) in source.lines.iter().enumerate() {
            let _line_number = line_num + 1;

            if mfa_regex.is_match(line) {
                has_mfa = true;
            }
            if metadata_regex.is_match(line) {
                has_metadata = true;
            }
        }

        if has_metadata && !has_mfa {
            offenses.push(Offense::new(
                self.name(),
                "Gemspec should require MFA: metadata['mfa_required'] = 'true'",
                self.severity(),
                Location::new(1, 1, 1),
            ));
        }

        offenses
    }
}

/// Checks that gemspec specifies required_ruby_version.
///
/// This cop ensures the gemspec declares a minimum Ruby version.
pub struct RequiredRubyVersion;

impl Cop for RequiredRubyVersion {
    fn name(&self) -> &str {
        "Gemspec/RequiredRubyVersion"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Checks that gemspec specifies required_ruby_version"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        let required_ruby_regex = Regex::new(r"\.required_ruby_version\s*=").unwrap();

        let mut has_required_ruby = false;

        for (line_num, line) in source.lines.iter().enumerate() {
            let _line_number = line_num + 1;

            if required_ruby_regex.is_match(line) {
                has_required_ruby = true;
                break;
            }
        }

        if !has_required_ruby && source.path.to_string_lossy().ends_with(".gemspec") {
            offenses.push(Offense::new(
                self.name(),
                "Gemspec should specify required_ruby_version",
                self.severity(),
                Location::new(1, 1, 1),
            ));
        }

        offenses
    }
}

/// Detects use of RUBY_VERSION instead of Gem::Version.
///
/// This cop flags direct use of RUBY_VERSION constant in gemspec.
pub struct RubyVersionGlobalsUsage;

impl Cop for RubyVersionGlobalsUsage {
    fn name(&self) -> &str {
        "Gemspec/RubyVersionGlobalsUsage"
    }

    fn category(&self) -> Category {
        Category::Style
    }

    fn severity(&self) -> Severity {
        Severity::Convention
    }

    fn description(&self) -> &str {
        "Detects use of RUBY_VERSION instead of Gem::Version"
    }

    fn check(&self, source: &SourceFile) -> Vec<Offense> {
        let mut offenses = Vec::new();

        let ruby_version_regex = Regex::new(r"\bRUBY_VERSION\b").unwrap();

        for (line_num, line) in source.lines.iter().enumerate() {
            let line_number = line_num + 1;

            if let Some(matched) = ruby_version_regex.find(line) {
                if !source.in_string_or_comment(line_number, matched.start() + 1) {
                    offenses.push(Offense::new(
                        self.name(),
                        "Use Gem::Version.new(RUBY_VERSION) instead of RUBY_VERSION",
                        self.severity(),
                        Location::new(line_number, matched.start() + 1, matched.len()),
                    ));
                }
            }
        }

        offenses
    }
}

/// Returns all Bundler and Gemspec cops as trait objects.
pub fn all_bundler_gemspec_cops() -> Vec<Box<dyn Cop>> {
    vec![
        // Bundler cops
        Box::new(DuplicatedGem),
        Box::new(DuplicatedGroup),
        Box::new(GemComment),
        Box::new(GemFilename),
        Box::new(GemVersion),
        Box::new(InsecureProtocolSource),
        Box::new(OrderedGems),
        // Gemspec cops
        Box::new(AddRuntimeDependency),
        Box::new(AttributeAssignment),
        Box::new(DependencyVersion),
        Box::new(DeprecatedAttributeAssignment),
        Box::new(DevelopmentDependencies),
        Box::new(DuplicatedAssignment),
        Box::new(OrderedDependencies),
        Box::new(RequireMFA),
        Box::new(RequiredRubyVersion),
        Box::new(RubyVersionGlobalsUsage),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn test_source(content: &str) -> SourceFile {
        SourceFile::from_string(PathBuf::from("Gemfile"), content.to_string())
    }

    fn test_gemspec(content: &str) -> SourceFile {
        SourceFile::from_string(PathBuf::from("test.gemspec"), content.to_string())
    }

    // ===== DuplicatedGem Tests =====

    #[test]
    fn test_duplicated_gem_no_duplicates() {
        let source = test_source("gem 'rails'\ngem 'rspec'\n");
        let cop = DuplicatedGem;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_duplicated_gem_with_duplicate() {
        let source = test_source("gem 'rails'\ngem 'rspec'\ngem 'rails'\n");
        let cop = DuplicatedGem;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert!(offenses[0].message.contains("rails"));
    }

    // ===== DuplicatedGroup Tests =====

    #[test]
    fn test_duplicated_group_no_duplicates() {
        let source = test_source("group :development do\nend\ngroup :test do\nend\n");
        let cop = DuplicatedGroup;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_duplicated_group_with_duplicate() {
        let source = test_source("group :development do\nend\ngroup :development do\nend\n");
        let cop = DuplicatedGroup;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert!(offenses[0].message.contains("development"));
    }

    // ===== GemComment Tests =====

    #[test]
    fn test_gem_comment_with_comment() {
        let source = test_source("# Web framework\ngem 'rails'\n");
        let cop = GemComment;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_gem_comment_without_comment() {
        let source = test_source("gem 'rails'\n");
        let cop = GemComment;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert!(offenses[0].message.contains("comment"));
    }

    #[test]
    fn test_gem_comment_inline_comment() {
        let source = test_source("gem 'rails' # Web framework\n");
        let cop = GemComment;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    // ===== GemFilename Tests =====

    #[test]
    fn test_gem_filename_correct() {
        let source = SourceFile::from_string(PathBuf::from("Gemfile"), "gem 'rails'\n".to_string());
        let cop = GemFilename;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_gem_filename_incorrect() {
        let source = SourceFile::from_string(PathBuf::from("gemfile"), "gem 'rails'\n".to_string());
        let cop = GemFilename;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert!(offenses[0].message.contains("Gemfile"));
    }

    // ===== GemVersion Tests =====

    #[test]
    fn test_gem_version_with_version() {
        let source = test_source("gem 'rails', '~> 7.0'\n");
        let cop = GemVersion;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_gem_version_without_version() {
        let source = test_source("gem 'rails'\n");
        let cop = GemVersion;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert!(offenses[0].message.contains("version constraint"));
    }

    // ===== InsecureProtocolSource Tests =====

    #[test]
    fn test_insecure_protocol_https() {
        let source = test_source("source 'https://rubygems.org'\n");
        let cop = InsecureProtocolSource;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_insecure_protocol_http() {
        let source = test_source("source 'http://rubygems.org'\n");
        let cop = InsecureProtocolSource;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert!(offenses[0].message.contains("https"));
    }

    // ===== OrderedGems Tests =====

    #[test]
    fn test_ordered_gems_sorted() {
        let source = test_source("gem 'rails'\ngem 'rspec'\ngem 'sqlite3'\n");
        let cop = OrderedGems;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_ordered_gems_unsorted() {
        let source = test_source("gem 'rspec'\ngem 'rails'\n");
        let cop = OrderedGems;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert!(offenses[0].message.contains("sorted"));
    }

    // ===== AddRuntimeDependency Tests =====

    #[test]
    fn test_add_runtime_dependency_correct() {
        let source = test_gemspec("spec.add_dependency 'rails'\n");
        let cop = AddRuntimeDependency;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_add_runtime_dependency_incorrect() {
        let source = test_gemspec("spec.add_runtime_dependency 'rails'\n");
        let cop = AddRuntimeDependency;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert!(offenses[0].message.contains("add_dependency"));
    }

    // ===== AttributeAssignment Tests =====

    #[test]
    fn test_attribute_assignment_unconditional() {
        let source = test_gemspec("spec.name = 'mygem'\n");
        let cop = AttributeAssignment;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_attribute_assignment_conditional() {
        let source = test_gemspec("if true\n  spec.name = 'mygem'\nend\n");
        let cop = AttributeAssignment;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert!(offenses[0].message.contains("conditionally"));
    }

    // ===== DependencyVersion Tests =====

    #[test]
    fn test_dependency_version_with_version() {
        let source = test_gemspec("spec.add_dependency 'rails', '~> 7.0'\n");
        let cop = DependencyVersion;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_dependency_version_without_version() {
        let source = test_gemspec("spec.add_dependency 'rails'\n");
        let cop = DependencyVersion;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert!(offenses[0].message.contains("version constraint"));
    }

    // ===== DeprecatedAttributeAssignment Tests =====

    #[test]
    fn test_deprecated_attribute_clean() {
        let source = test_gemspec("spec.name = 'mygem'\n");
        let cop = DeprecatedAttributeAssignment;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_deprecated_attribute_rubyforge() {
        let source = test_gemspec("spec.rubyforge_project = 'mygem'\n");
        let cop = DeprecatedAttributeAssignment;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert!(offenses[0].message.contains("Deprecated"));
    }

    // ===== DevelopmentDependencies Tests =====

    #[test]
    fn test_development_dependencies_correct() {
        let source = test_gemspec("spec.add_development_dependency 'rspec'\n");
        let cop = DevelopmentDependencies;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_development_dependencies_gem_in_gemspec() {
        let source = test_gemspec("gem 'rspec'\n");
        let cop = DevelopmentDependencies;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert!(offenses[0].message.contains("add_development_dependency"));
    }

    // ===== DuplicatedAssignment Tests =====

    #[test]
    fn test_duplicated_assignment_no_duplicates() {
        let source = test_gemspec("spec.name = 'mygem'\nspec.version = '1.0'\n");
        let cop = DuplicatedAssignment;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_duplicated_assignment_with_duplicate() {
        let source = test_gemspec("spec.name = 'mygem'\nspec.name = 'othergem'\n");
        let cop = DuplicatedAssignment;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert!(offenses[0].message.contains("name"));
    }

    // ===== OrderedDependencies Tests =====

    #[test]
    fn test_ordered_dependencies_sorted() {
        let source = test_gemspec("spec.add_dependency 'rails'\nspec.add_dependency 'rspec'\n");
        let cop = OrderedDependencies;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_ordered_dependencies_unsorted() {
        let source = test_gemspec("spec.add_dependency 'rspec'\nspec.add_dependency 'rails'\n");
        let cop = OrderedDependencies;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert!(offenses[0].message.contains("sorted"));
    }

    // ===== RequireMFA Tests =====

    #[test]
    fn test_require_mfa_present() {
        let source = test_gemspec("spec.metadata['mfa_required'] = 'true'\n");
        let cop = RequireMFA;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_require_mfa_missing() {
        let source = test_gemspec("spec.metadata['homepage'] = 'http://example.com'\n");
        let cop = RequireMFA;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert!(offenses[0].message.contains("MFA"));
    }

    // ===== RequiredRubyVersion Tests =====

    #[test]
    fn test_required_ruby_version_present() {
        let source = test_gemspec("spec.required_ruby_version = '>= 2.7'\n");
        let cop = RequiredRubyVersion;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_required_ruby_version_missing() {
        let source = test_gemspec("spec.name = 'mygem'\n");
        let cop = RequiredRubyVersion;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert!(offenses[0].message.contains("required_ruby_version"));
    }

    // ===== RubyVersionGlobalsUsage Tests =====

    #[test]
    fn test_ruby_version_globals_clean() {
        let source = test_gemspec("spec.required_ruby_version = '>= 2.7'\n");
        let cop = RubyVersionGlobalsUsage;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 0);
    }

    #[test]
    fn test_ruby_version_globals_usage() {
        let source = test_gemspec("if RUBY_VERSION >= '2.7'\n  spec.name = 'mygem'\nend\n");
        let cop = RubyVersionGlobalsUsage;
        let offenses = cop.check(&source);
        assert_eq!(offenses.len(), 1);
        assert!(offenses[0].message.contains("Gem::Version"));
    }
}
