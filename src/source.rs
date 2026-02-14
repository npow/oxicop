use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Represents a loaded Ruby source file with precomputed line data.
#[derive(Debug, Clone)]
pub struct SourceFile {
    /// Path to the file (may be synthetic for tests).
    pub path: PathBuf,
    /// Raw file content.
    pub content: String,
    /// Lines split without trailing newline characters.
    pub lines: Vec<String>,
}

impl SourceFile {
    /// Load a source file from disk.
    pub fn from_path(path: &Path) -> io::Result<Self> {
        let content = fs::read_to_string(path)?;
        Ok(Self::from_string(path.to_path_buf(), content))
    }

    /// Create a source file from an in-memory string (useful for testing).
    pub fn from_string(path: PathBuf, content: String) -> Self {
        let lines = content.lines().map(String::from).collect();
        Self {
            path,
            content,
            lines,
        }
    }

    /// Number of lines in the file.
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    /// Whether the file has no content.
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    /// Get a specific line (1-based index). Returns None if out of bounds.
    pub fn line(&self, line_number: usize) -> Option<&str> {
        if line_number == 0 || line_number > self.lines.len() {
            None
        } else {
            Some(&self.lines[line_number - 1])
        }
    }

    /// Check whether a given column on a line is inside a string literal or comment.
    /// This is a heuristic - not 100% accurate for heredocs or complex interpolation,
    /// but good enough for most linting decisions.
    pub fn in_string_or_comment(&self, line_number: usize, column: usize) -> bool {
        if let Some(line) = self.line(line_number) {
            let chars: Vec<char> = line.chars().collect();
            let mut in_single_quote = false;
            let mut in_double_quote = false;
            let mut escaped = false;

            for (i, &ch) in chars.iter().enumerate() {
                if i + 1 >= column {
                    break;
                }
                if escaped {
                    escaped = false;
                    continue;
                }
                if ch == '\\' && in_double_quote {
                    escaped = true;
                    continue;
                }
                if ch == '#' && !in_single_quote && !in_double_quote {
                    return true;
                }
                if ch == '\'' && !in_double_quote {
                    in_single_quote = !in_single_quote;
                }
                if ch == '"' && !in_single_quote {
                    in_double_quote = !in_double_quote;
                }
            }
            in_single_quote || in_double_quote
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_source(content: &str) -> SourceFile {
        SourceFile::from_string(PathBuf::from("test.rb"), content.to_string())
    }

    #[test]
    fn test_from_string_basic() {
        let source = test_source("line 1\nline 2\nline 3\n");
        assert_eq!(source.line_count(), 3);
        assert_eq!(source.line(1), Some("line 1"));
        assert_eq!(source.line(2), Some("line 2"));
        assert_eq!(source.line(3), Some("line 3"));
    }

    #[test]
    fn test_empty_file() {
        let source = test_source("");
        assert!(source.is_empty());
        assert_eq!(source.line_count(), 0);
        assert_eq!(source.line(1), None);
    }

    #[test]
    fn test_line_out_of_bounds() {
        let source = test_source("hello\n");
        assert_eq!(source.line(0), None);
        assert_eq!(source.line(2), None);
    }

    #[test]
    fn test_in_comment() {
        let source = test_source("code # comment\n");
        assert!(!source.in_string_or_comment(1, 1));
        assert!(source.in_string_or_comment(1, 8));
    }

    #[test]
    fn test_in_double_quoted_string() {
        let source = test_source("x = \"hello world\"\n");
        assert!(!source.in_string_or_comment(1, 4));
        assert!(source.in_string_or_comment(1, 7));
    }

    #[test]
    fn test_in_single_quoted_string() {
        let source = test_source("x = 'hello world'\n");
        assert!(!source.in_string_or_comment(1, 4));
        assert!(source.in_string_or_comment(1, 7));
    }

    #[test]
    fn test_no_trailing_newline() {
        let source = test_source("no newline at end");
        assert_eq!(source.line_count(), 1);
        assert_eq!(source.line(1), Some("no newline at end"));
    }

    #[test]
    fn test_escaped_quote_in_string() {
        let source = test_source("x = \"he said \\\"hi\\\"\"\n");
        // The whole string including escaped quotes is inside a string
        assert!(source.in_string_or_comment(1, 10));
    }
}
