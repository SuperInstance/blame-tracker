//! blame-tracker — Line-level blame/provenance tracking for text documents.
//!
/// Maintains a mapping from lines to their origin (commit, author, timestamp)
/// and supports blame queries.

/// Provenance metadata for a single line.
#[derive(Debug, Clone)]
pub struct LineOrigin {
    pub line_number: usize,
    pub commit: String,
    pub author: String,
    pub timestamp: u64,
    pub content: String,
}

/// A blame map for a document.
#[derive(Debug, Clone, Default)]
pub struct BlameMap {
    lines: Vec<LineOrigin>,
}

impl BlameMap {
    /// Create a new empty blame map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Build a blame map from content and a single origin (e.g., initial commit).
    pub fn from_single_origin(content: &str, commit: &str, author: &str, timestamp: u64) -> Self {
        let lines = content
            .lines()
            .enumerate()
            .map(|(i, line)| LineOrigin {
                line_number: i + 1,
                commit: commit.to_string(),
                author: author.to_string(),
                timestamp,
                content: line.to_string(),
            })
            .collect();
        Self { lines }
    }

    /// Get blame for a specific line (1-based).
    pub fn blame_line(&self, line: usize) -> Option<&LineOrigin> {
        if line == 0 { return None; }
        self.lines.get(line - 1)
    }

    /// Get the full blame for all lines.
    pub fn blame_all(&self) -> &[LineOrigin] {
        &self.lines
    }

    /// Apply an edit: replace lines `start..=end` (1-based inclusive) with new content.
    /// The new lines get the given origin metadata.
    pub fn apply_edit(
        &mut self,
        start: usize,
        end: usize,
        new_content: &str,
        commit: &str,
        author: &str,
        timestamp: u64,
    ) {
        if start == 0 || start > self.lines.len() + 1 { return; }
        let new_lines: Vec<LineOrigin> = new_content
            .lines()
            .enumerate()
            .map(|(i, line)| LineOrigin {
                line_number: start + i,
                commit: commit.to_string(),
                author: author.to_string(),
                timestamp,
                content: line.to_string(),
            })
            .collect();

        let replace_end = (end.min(self.lines.len())).max(start - 1);
        self.lines.splice(start - 1..replace_end, new_lines);

        // Re-number
        for (i, line) in self.lines.iter_mut().enumerate() {
            line.line_number = i + 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_origin_blame() {
        let map = BlameMap::from_single_origin("a\nb\nc", "abc123", "alice", 1000);
        let line = map.blame_line(2).unwrap();
        assert_eq!(line.content, "b");
        assert_eq!(line.author, "alice");
    }

    #[test]
    fn edit_blame() {
        let mut map = BlameMap::from_single_origin("a\nb\nc", "abc", "alice", 1000);
        map.apply_edit(2, 2, "x\ny", "def", "bob", 2000);
        assert_eq!(map.blame_all().len(), 4);
        assert_eq!(map.blame_line(2).unwrap().content, "x");
        assert_eq!(map.blame_line(2).unwrap().author, "bob");
        assert_eq!(map.blame_line(4).unwrap().content, "c");
    }
}

/// FNV-1a 64 — the digest every substrate in the SuperInstance fleet agrees on.
pub const FNV_OFFSET: u64 = 0xcbf29ce484222325;
pub const FNV_PRIME: u64 = 0x100000001b3;

#[inline]
pub fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut h = FNV_OFFSET;
    for &b in bytes {
        h = (h ^ b as u64).wrapping_mul(FNV_PRIME);
    }
    h
}

/// True if this crate's FNV-1a still agrees with the rest of the fleet.
pub fn canary_holds() -> bool {
    fnv1a64("café Δ 日本語".as_bytes()) == 0x024a555471370b18d
}
