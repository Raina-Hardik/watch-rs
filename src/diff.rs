//! Difference highlighting for watch-rs

use std::collections::HashSet;

/// Represents a character position that has changed
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChangedPosition {
    pub line: usize,
    pub col: usize,
}

/// Diff engine for highlighting differences between outputs
#[derive(Debug, Default)]
pub struct DiffEngine {
    /// Previous output lines
    previous: Vec<String>,
    /// First output (for permanent mode)
    first: Option<Vec<String>>,
    /// All positions that have ever changed (for permanent mode)
    all_changes: HashSet<ChangedPosition>,
    /// Whether this is the first run
    is_first_run: bool,
}

#[allow(dead_code)]
impl DiffEngine {
    /// Create a new diff engine
    pub fn new() -> Self {
        DiffEngine {
            previous: Vec::new(),
            first: None,
            all_changes: HashSet::new(),
            is_first_run: true,
        }
    }

    /// Reset the diff engine (used when terminal resizes)
    pub fn reset(&mut self) {
        self.previous.clear();
        self.first = None;
        self.all_changes.clear();
        self.is_first_run = true;
    }

    /// Calculate differences between current and previous output
    /// Returns a set of positions that should be highlighted
    pub fn calculate_diff(&mut self, current: &str, permanent: bool) -> HashSet<ChangedPosition> {
        let current_lines: Vec<String> = current.lines().map(|s| s.to_string()).collect();

        if self.is_first_run {
            self.is_first_run = false;
            self.previous = current_lines.clone();
            if permanent {
                self.first = Some(current_lines);
            }
            return HashSet::new();
        }

        let compare_to = if permanent {
            self.first.as_ref().unwrap_or(&self.previous)
        } else {
            &self.previous
        };

        let mut changes = HashSet::new();

        // Compare each line
        let max_lines = current_lines.len().max(compare_to.len());

        for line_idx in 0..max_lines {
            let current_line = current_lines
                .get(line_idx)
                .map(|s| s.as_str())
                .unwrap_or("");
            let prev_line = compare_to.get(line_idx).map(|s| s.as_str()).unwrap_or("");

            if current_line != prev_line {
                // Find character-level differences
                let current_chars: Vec<char> = current_line.chars().collect();
                let prev_chars: Vec<char> = prev_line.chars().collect();
                let max_cols = current_chars.len().max(prev_chars.len());

                for col_idx in 0..max_cols {
                    let current_char = current_chars.get(col_idx);
                    let prev_char = prev_chars.get(col_idx);

                    if current_char != prev_char {
                        // Only mark if there's a character in current output
                        if current_char.is_some() {
                            changes.insert(ChangedPosition {
                                line: line_idx,
                                col: col_idx,
                            });
                        }
                    }
                }
            }
        }

        // For permanent mode, accumulate all changes
        if permanent {
            self.all_changes.extend(changes.iter().cloned());
            // Update previous for next comparison
            self.previous = current_lines;
            self.all_changes.clone()
        } else {
            self.previous = current_lines;
            changes
        }
    }

    /// Check if this is the first run (no previous output to compare)
    pub fn is_first(&self) -> bool {
        self.is_first_run
    }
}

/// Simple line-based diff for checking if output has changed
#[allow(dead_code)]
pub fn output_changed(current: &str, previous: &str) -> bool {
    current != previous
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_run_no_changes() {
        let mut engine = DiffEngine::new();
        let changes = engine.calculate_diff("Hello World", false);
        assert!(changes.is_empty());
    }

    #[test]
    fn test_detect_character_change() {
        let mut engine = DiffEngine::new();
        engine.calculate_diff("Hello", false);
        let changes = engine.calculate_diff("Hallo", false);

        assert!(changes.contains(&ChangedPosition { line: 0, col: 1 }));
    }

    #[test]
    fn test_detect_line_addition() {
        let mut engine = DiffEngine::new();
        engine.calculate_diff("Line1", false);
        let changes = engine.calculate_diff("Line1\nLine2", false);

        // Should have changes for the new line
        assert!(!changes.is_empty());
    }

    #[test]
    fn test_permanent_mode() {
        let mut engine = DiffEngine::new();

        // First run
        engine.calculate_diff("AAA", true);

        // Second run - change to BBB
        let changes1 = engine.calculate_diff("BBB", true);
        assert_eq!(changes1.len(), 3); // All 3 characters changed

        // Third run - change to CCC (should still show all changes from original)
        let changes2 = engine.calculate_diff("CCC", true);
        assert_eq!(changes2.len(), 3); // Still all 3 positions
    }

    #[test]
    fn test_output_changed() {
        assert!(output_changed("hello", "world"));
        assert!(!output_changed("same", "same"));
    }
}
