//! Oxicop - A blazing-fast Ruby linter in Rust.

use std::path::{Path, PathBuf};
use std::process;

use clap::Parser;
use ignore::WalkBuilder;

use oxicop::config::Config;
use oxicop::formatter::{create_formatter, Format};
use oxicop::registry::CopRegistry;
use oxicop::runner::Runner;

#[derive(Parser)]
#[command(name = "oxicop", about = "A blazing-fast Ruby linter", version)]
struct Cli {
    /// Files or directories to lint
    #[arg(default_value = ".")]
    paths: Vec<PathBuf>,

    /// Output format (simple, compact, json)
    #[arg(short, long, default_value = "simple")]
    format: String,

    /// Only run specific cops (comma-separated)
    #[arg(long)]
    only: Option<String>,

    /// Exclude specific cops (comma-separated)
    #[arg(long)]
    except: Option<String>,

    /// Config file path
    #[arg(short, long)]
    config: Option<PathBuf>,

    /// List all available cops
    #[arg(long)]
    list: bool,
}

fn main() {
    let cli = Cli::parse();

    // Build the cop registry
    let mut registry = CopRegistry::new();

    // Handle --list flag
    if cli.list {
        list_cops(&registry);
        return;
    }

    // Load configuration
    let config = if let Some(config_path) = cli.config {
        match Config::from_file(&config_path) {
            Ok(c) => Some(c),
            Err(e) => {
                eprintln!("Error loading config file: {}", e);
                process::exit(1);
            }
        }
    } else {
        Config::find_and_load()
    };

    // Apply configuration to registry
    if let Some(ref cfg) = config {
        apply_config_to_registry(&mut registry, cfg);
    }

    // Apply CLI filters
    if let Some(ref only) = cli.only {
        apply_only_filter(&mut registry, only);
    }

    if let Some(ref except) = cli.except {
        apply_except_filter(&mut registry, except);
    }

    // Discover Ruby files
    let ruby_files = discover_ruby_files(&cli.paths);

    if ruby_files.is_empty() {
        println!("No Ruby files found.");
        return;
    }

    // Run the linter
    let runner = Runner::new(registry);
    let result = runner.run(&ruby_files);

    // Format and print output
    let format = Format::from_str(&cli.format).unwrap_or(Format::Simple);
    let formatter = create_formatter(format);
    let output = formatter.format(&result);
    print!("{}", output);

    // Exit with appropriate code
    if result.total_offenses > 0 {
        process::exit(1);
    }
}

/// Lists all available cops with their categories and descriptions.
fn list_cops(registry: &CopRegistry) {
    println!("Available cops:\n");

    let mut cops: Vec<_> = registry
        .cop_names()
        .iter()
        .map(|&name| name)
        .collect();
    cops.sort();

    for cop_name in cops {
        println!("  {}", cop_name);
    }

    println!("\nTotal: {} cops", registry.total_count());
}

/// Applies configuration settings to the registry.
fn apply_config_to_registry(registry: &mut CopRegistry, config: &Config) {
    let cop_names: Vec<String> = registry.cop_names().iter().map(|&s| s.to_string()).collect();
    for cop_name in &cop_names {
        if let Some(enabled) = config.is_cop_enabled(cop_name) {
            if !enabled {
                registry.disable(cop_name);
            } else {
                registry.enable(cop_name);
            }
        }
    }
}

/// Applies the --only filter to enable only specified cops.
fn apply_only_filter(registry: &mut CopRegistry, only: &str) {
    let allowed: Vec<&str> = only.split(',').map(|s| s.trim()).collect();
    let cop_names: Vec<String> = registry.cop_names().iter().map(|&s| s.to_string()).collect();

    for cop_name in &cop_names {
        if !allowed.contains(&cop_name.as_str()) {
            registry.disable(cop_name);
        }
    }
}

/// Applies the --except filter to disable specified cops.
fn apply_except_filter(registry: &mut CopRegistry, except: &str) {
    let excluded: Vec<&str> = except.split(',').map(|s| s.trim()).collect();

    for cop_name in excluded {
        registry.disable(cop_name);
    }
}

/// Discovers all Ruby files in the given paths.
fn discover_ruby_files(paths: &[PathBuf]) -> Vec<PathBuf> {
    let mut ruby_files = Vec::new();

    for path in paths {
        if path.is_file() {
            if is_ruby_file(path) {
                ruby_files.push(path.clone());
            }
        } else if path.is_dir() {
            // Use WalkBuilder for efficient directory traversal
            for entry in WalkBuilder::new(path).build() {
                if let Ok(entry) = entry {
                    let entry_path = entry.path();
                    if entry_path.is_file() && is_ruby_file(entry_path) {
                        ruby_files.push(entry_path.to_path_buf());
                    }
                }
            }
        }
    }

    ruby_files
}

/// Checks if a file is a Ruby file based on extension.
fn is_ruby_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext == "rb")
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_ruby_file() {
        assert!(is_ruby_file(&PathBuf::from("test.rb")));
        assert!(is_ruby_file(&PathBuf::from("path/to/file.rb")));
        assert!(!is_ruby_file(&PathBuf::from("test.py")));
        assert!(!is_ruby_file(&PathBuf::from("test.txt")));
        assert!(!is_ruby_file(&PathBuf::from("no_extension")));
    }

    #[test]
    fn test_apply_only_filter() {
        let mut registry = CopRegistry::new();
        let initial_count = registry.enabled_count();

        apply_only_filter(&mut registry, "Layout/TrailingWhitespace,Style/StringLiterals");

        let enabled = registry.enabled_cops();
        assert!(enabled.len() <= 2);

        for cop in enabled {
            assert!(
                cop.name() == "Layout/TrailingWhitespace"
                    || cop.name() == "Style/StringLiterals"
            );
        }
    }

    #[test]
    fn test_apply_except_filter() {
        let mut registry = CopRegistry::new();
        let initial_count = registry.enabled_count();

        apply_except_filter(&mut registry, "Layout/TrailingWhitespace");

        assert_eq!(registry.enabled_count(), initial_count - 1);
        assert!(!registry.is_enabled("Layout/TrailingWhitespace"));
    }

    #[test]
    fn test_apply_config_to_registry() {
        use std::collections::HashMap;

        let mut registry = CopRegistry::new();

        let mut cops = HashMap::new();
        cops.insert(
            "Layout/TrailingWhitespace".to_string(),
            oxicop::config::CopConfig {
                enabled: Some(false),
                severity: None,
            },
        );

        let config = Config {
            all_cops: None,
            cops,
        };

        apply_config_to_registry(&mut registry, &config);

        assert!(!registry.is_enabled("Layout/TrailingWhitespace"));
    }
}
