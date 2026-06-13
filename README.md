# Blame Tracker

**A Rust library for line-level provenance tracking** — maintains a mapping from text lines to their origin (commit, author, timestamp), providing `git blame`-style attribution as edits are applied.

## Why It Matters

Code provenance — knowing *who changed each line and when* — is essential for:

- **Accountability** — identifying who introduced a bug or security vulnerability
- **Code archaeology** — understanding why code exists by tracing its history
- **Compliance** — audit trails for regulated industries (SOX, HIPAA)
- **Review routing** — automatically CC'ing the last person who touched a function

Git's `blame` command computes this retroactively by diffing commit history. This library provides a **forward-tracking** alternative: you maintain blame metadata as edits happen, giving O(1) blame lookups without historical diffing.

## How It Works

**Initialization**: `BlameMap::from_single_origin()` creates a fresh blame map where every line is attributed to a single commit (e.g., the initial file creation).

**Line tracking**: Each line stores a `LineOrigin` containing: line number, commit hash, author name, Unix timestamp, and line content. Lines are stored in a `Vec<LineOrigin>` — a flat array indexed by (line number − 1).

**Edit application**: `apply_edit(start, end, new_content, ...)` replaces lines `start..=end` (1-based inclusive) with new content. The new lines are attributed to the new commit/author/timestamp. Lines after the edit are renumbered automatically. The implementation uses `Vec::splice` for efficient replacement.

**Queries**: `blame_line(n)` returns O(1) provenance for line n. `blame_all()` returns the full blame map for bulk export.

## Quick Start

```rust
use blame_tracker::BlameMap;

// Start with an initial commit by Alice
let mut map = BlameMap::from_single_origin("a\nb\nc", "abc123", "alice", 1000);

// Bob edits line 2
map.apply_edit(2, 2, "B", "def456", "bob", 2000);

// Check blame
assert_eq!(map.blame_line(1).unwrap().author, "alice");
assert_eq!(map.blame_line(2).unwrap().author, "bob");
assert_eq!(map.blame_line(2).unwrap().content, "B");
assert_eq!(map.blame_line(3).unwrap().author, "alice"); // untouched
```

## API

- **`LineOrigin`** — Provenance: line_number, commit, author, timestamp, content
- **`BlameMap`** — The blame data store
  - `from_single_origin(content, commit, author, ts)` — Initialize blame from one commit
  - `blame_line(n)` → `Option<&LineOrigin>` — O(1) blame for line n
  - `blame_all()` → `&[LineOrigin]` — Full blame data
  - `apply_edit(start, end, content, commit, author, ts)` — Apply an edit with new attribution

## Architecture Notes

Provides the provenance tracking primitive for SuperInstance code analysis tools. The forward-tracking design complements git's backward-looking blame — useful for collaborative editing, operational transforms, and real-time attribution. See the [architecture overview](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md).

## License

MIT
