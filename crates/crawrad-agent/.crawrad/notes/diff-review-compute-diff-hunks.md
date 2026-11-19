---
id: diff-review-compute-diff-hunks
kind: doc
target_file: src/diff_review.rs
line_hint: 289
author: 
created: 
title: "compute_diff_hunks"
tags: []
signature: "pub fn compute_diff_hunks (original : & str , proposed : & str) -> Vec < DiffHunk >"
---

Splits text into per-line difference blocks.

Splits the text into per-line difference blocks (a simple, deterministic algorithm).

# Examples

```rust
use crawrad_agent::compute_diff_hunks;

let hunks = compute_diff_hunks("old\n", "new\n");
assert_eq!(hunks.len(), 1);
```
