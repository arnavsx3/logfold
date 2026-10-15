use std::collections::HashMap;

use crate::mask::mask_line;

/// One folded pattern and how many lines matched it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pattern {
    pub text: String,
    pub count: usize,
}

/// Collects log lines one by one and counts the masked patterns.
#[derive(Debug, Default)]
pub struct Folder {
    counts: HashMap<String, usize>,
}

impl Folder {
    /// Creates an empty folder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds one log line, ignoring empty or whitespace-only lines.
    pub fn add(&mut self, line: &str) {
        let pattern = mask_line(line);
        if !pattern.is_empty() {
            *self.counts.entry(pattern).or_default() += 1;
        }
    }

    /// Consumes the folder and returns patterns, most frequent first.
    /// Ties are sorted lexicographically by pattern text for repeatable output.
    pub fn finish(self) -> Vec<Pattern> {
        let mut patterns: Vec<Pattern> = self
            .counts
            .into_iter()
            .map(|(text, count)| Pattern { text, count })
            .collect();

        patterns.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.text.cmp(&b.text)));
        patterns
    }
}
