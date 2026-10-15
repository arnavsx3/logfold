use std::collections::HashMap;

use crate::mask::mask_line;

/// One folded pattern and how many lines matched it.
#[derive(Debug, PartialEq, Eq)]
pub struct Pattern {
    pub text: String,
    pub count: usize,
}

/// Collects log lines one by one and counts the masked patterns.
#[derive(Default)]
pub struct Folder {
    counts: HashMap<String, usize>,
}

impl Folder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, line: &str) {
        *self.counts.entry(mask_line(line)).or_insert(0) += 1;
    }

    /// Consumes the folder and returns patterns, most frequent first.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_folder_gives_no_patterns() {
        assert!(Folder::new().finish().is_empty());
    }
}
