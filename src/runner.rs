//! Parallel file processing and linting engine.

use std::path::{Path, PathBuf};

use rayon::prelude::*;

use crate::offense::Offense;
use crate::registry::CopRegistry;
use crate::source::SourceFile;

/// The main linting runner.
pub struct Runner {
    registry: CopRegistry,
}

/// Result of checking a single file.
#[derive(Debug, Clone)]
pub struct FileResult {
    pub path: PathBuf,
    pub offenses: Vec<Offense>,
}

/// Result of a complete linting run.
#[derive(Debug)]
pub struct RunResult {
    pub file_results: Vec<FileResult>,
    pub total_files: usize,
    pub total_offenses: usize,
}

impl Runner {
    /// Creates a new runner with the given cop registry.
    pub fn new(registry: CopRegistry) -> Self {
        Self { registry }
    }

    /// Runs all enabled cops on the given files in parallel.
    pub fn run(&self, paths: &[PathBuf]) -> RunResult {
        let mut file_results: Vec<FileResult> = paths
            .par_iter()
            .filter_map(|path| self.check_file(path))
            .collect();

        // Sort by path for consistent output
        file_results.sort_by(|a, b| a.path.cmp(&b.path));

        let total_offenses = file_results.iter().map(|r| r.offenses.len()).sum();
        let total_files = file_results.len();

        RunResult {
            file_results,
            total_files,
            total_offenses,
        }
    }

    /// Checks a single file with all enabled cops.
    fn check_file(&self, path: &Path) -> Option<FileResult> {
        // Skip if not a file
        if !path.is_file() {
            return None;
        }

        // Load the source file
        let source = match SourceFile::from_path(path) {
            Ok(s) => s,
            Err(_) => return None, // Skip files that can't be read
        };

        // Run all enabled cops
        let enabled_cops = self.registry.enabled_cops();
        let mut offenses: Vec<Offense> = enabled_cops
            .iter()
            .flat_map(|cop| cop.check(&source))
            .collect();

        // Sort offenses by location (line, then column)
        offenses.sort_by(|a, b| {
            a.location
                .line
                .cmp(&b.location.line)
                .then_with(|| a.location.column.cmp(&b.location.column))
        });

        Some(FileResult {
            path: path.to_path_buf(),
            offenses,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_file_with_offenses() {
        use std::fs;
        use std::io::Write;

        // Create a temporary file
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join("test_runner_file.rb");
        {
            let mut file = fs::File::create(&file_path).unwrap();
            writeln!(file, "def foo").unwrap();
            writeln!(file, "  bar").unwrap();
            writeln!(file, "end").unwrap();
        }

        let registry = CopRegistry::new();
        let runner = Runner::new(registry);

        let result = runner.check_file(&file_path);
        assert!(result.is_some());

        let file_result = result.unwrap();
        assert_eq!(file_result.path, file_path);

        // Clean up
        let _ = fs::remove_file(&file_path);
    }

    #[test]
    fn test_run_multiple_files() {
        use std::fs;
        use std::io::Write;

        let temp_dir = std::env::temp_dir();
        let file1 = temp_dir.join("test_runner_1.rb");
        let file2 = temp_dir.join("test_runner_2.rb");

        {
            let mut f = fs::File::create(&file1).unwrap();
            writeln!(f, "puts 'hello'").unwrap();
        }
        {
            let mut f = fs::File::create(&file2).unwrap();
            writeln!(f, "puts 'world'").unwrap();
        }

        let registry = CopRegistry::new();
        let runner = Runner::new(registry);

        let result = runner.run(&[file1.clone(), file2.clone()]);

        assert_eq!(result.total_files, 2);
        assert_eq!(result.file_results.len(), 2);

        // Clean up
        let _ = fs::remove_file(&file1);
        let _ = fs::remove_file(&file2);
    }

    #[test]
    fn test_check_nonexistent_file() {
        let registry = CopRegistry::new();
        let runner = Runner::new(registry);

        let result = runner.check_file(Path::new("/nonexistent/file.rb"));
        assert!(result.is_none());
    }

    #[test]
    fn test_offense_sorting() {
        use std::fs;
        use std::io::Write;

        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join("test_runner_sort.rb");
        {
            let mut file = fs::File::create(&file_path).unwrap();
            writeln!(file, "line 1").unwrap();
            writeln!(file, "line 2").unwrap();
        }

        let registry = CopRegistry::new();
        let runner = Runner::new(registry);

        if let Some(result) = runner.check_file(&file_path) {
            // Verify offenses are sorted by line and column
            for i in 1..result.offenses.len() {
                let prev = &result.offenses[i - 1];
                let curr = &result.offenses[i];
                assert!(
                    prev.location.line < curr.location.line
                        || (prev.location.line == curr.location.line
                            && prev.location.column <= curr.location.column)
                );
            }
        }

        let _ = fs::remove_file(&file_path);
    }
}
