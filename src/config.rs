//! Configuration parsing from .rubocop.yml files.

use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};

use serde::Deserialize;

/// Main configuration structure matching RuboCop's format.
#[derive(Debug, Default, Deserialize)]
pub struct Config {
    #[serde(rename = "AllCops")]
    pub all_cops: Option<AllCopsConfig>,
    #[serde(flatten)]
    pub cops: HashMap<String, CopConfig>,
}

/// Global configuration under the AllCops key.
#[derive(Debug, Deserialize)]
pub struct AllCopsConfig {
    #[serde(rename = "Exclude")]
    pub exclude: Option<Vec<String>>,
    #[serde(rename = "TargetRubyVersion")]
    pub target_ruby_version: Option<f64>,
}

/// Per-cop configuration.
#[derive(Debug, Deserialize)]
pub struct CopConfig {
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,
    #[serde(rename = "Severity")]
    pub severity: Option<String>,
}

impl Config {
    /// Loads configuration from a specific file.
    pub fn from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    /// Searches up the directory tree for .rubocop.yml and loads it.
    /// Starts from the current working directory.
    pub fn find_and_load() -> Option<Self> {
        let current_dir = env::current_dir().ok()?;
        Self::find_config_file(&current_dir)
            .and_then(|path| Self::from_file(&path).ok())
    }

    /// Searches for .rubocop.yml starting from the given directory and going up.
    fn find_config_file(start_dir: &Path) -> Option<PathBuf> {
        let mut current = start_dir.to_path_buf();
        
        loop {
            let config_path = current.join(".rubocop.yml");
            if config_path.exists() {
                return Some(config_path);
            }
            
            // Try parent directory
            if !current.pop() {
                break;
            }
        }
        
        None
    }

    /// Checks if a cop is explicitly enabled in the config.
    pub fn is_cop_enabled(&self, cop_name: &str) -> Option<bool> {
        self.cops.get(cop_name)?.enabled
    }

    /// Gets the severity override for a cop, if any.
    pub fn cop_severity(&self, cop_name: &str) -> Option<&str> {
        self.cops.get(cop_name)?.severity.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty_config() {
        let yaml = "";
        let config: Config = serde_yaml::from_str(yaml).unwrap();
        assert!(config.all_cops.is_none());
        assert!(config.cops.is_empty());
    }

    #[test]
    fn test_parse_all_cops_config() {
        let yaml = r#"
AllCops:
  Exclude:
    - 'vendor/**/*'
    - 'db/schema.rb'
  TargetRubyVersion: 3.0
"#;
        let config: Config = serde_yaml::from_str(yaml).unwrap();
        
        let all_cops = config.all_cops.as_ref().unwrap();
        assert_eq!(all_cops.target_ruby_version, Some(3.0));
        
        let exclude = all_cops.exclude.as_ref().unwrap();
        assert_eq!(exclude.len(), 2);
        assert!(exclude.contains(&"vendor/**/*".to_string()));
    }

    #[test]
    fn test_parse_cop_config() {
        let yaml = r#"
Layout/TrailingWhitespace:
  Enabled: false
  
Style/StringLiterals:
  Enabled: true
  Severity: warning
"#;
        let config: Config = serde_yaml::from_str(yaml).unwrap();
        
        assert_eq!(config.is_cop_enabled("Layout/TrailingWhitespace"), Some(false));
        assert_eq!(config.is_cop_enabled("Style/StringLiterals"), Some(true));
        assert_eq!(config.cop_severity("Style/StringLiterals"), Some("warning"));
    }

    #[test]
    fn test_parse_mixed_config() {
        let yaml = r#"
AllCops:
  Exclude:
    - 'tmp/**/*'
  TargetRubyVersion: 2.7

Layout/TrailingWhitespace:
  Enabled: false

Lint/Debugger:
  Severity: error
"#;
        let config: Config = serde_yaml::from_str(yaml).unwrap();
        
        assert!(config.all_cops.is_some());
        assert_eq!(config.cops.len(), 2);
        assert_eq!(config.is_cop_enabled("Layout/TrailingWhitespace"), Some(false));
        assert_eq!(config.cop_severity("Lint/Debugger"), Some("error"));
    }

    #[test]
    fn test_cop_not_in_config() {
        let yaml = r#"
Layout/TrailingWhitespace:
  Enabled: false
"#;
        let config: Config = serde_yaml::from_str(yaml).unwrap();
        
        assert_eq!(config.is_cop_enabled("Style/StringLiterals"), None);
        assert_eq!(config.cop_severity("Style/StringLiterals"), None);
    }

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(config.all_cops.is_none());
        assert!(config.cops.is_empty());
    }
}
