---
id: diff-review-diffhunk
kind: doc
target_file: src/diff_review.rs
line_hint: 65
author: 
created: 
title: "DiffHunk"
tags: []
signature: "pub struct DiffHunk"
---

A contiguous block of changed lines between the original and the proposal.

# Examples

```rust
use crawrad_agent::{DiffHunk, HunkStatus};

let hunk = DiffHunk {
    id: 1,
    start_line_old: 1,
    end_line_old: 1,
    start_line_new: 1,
    end_line_new: 1,
    old_lines: vec!["old".to_string()],
    new_lines: vec!["new".to_string()],
    status: HunkStatus::Pending,
};
assert_eq!(hunk.added_count(), 1);
assert_eq!(hunk.removed_count(), 1);
```
