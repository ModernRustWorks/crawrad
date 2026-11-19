---
id: line-edits-duplicate-lines
kind: doc
target_file: src/line_edits.rs
line_hint: 69
author: 
created: 
title: "duplicate_lines"
tags: []
signature: "pub fn duplicate_lines (lines : & [String] , start : usize , end : usize , up : bool ,) -> (Vec < String > , usize , usize)"
---

Duplicates the line block above or below its current position.

A copy of the block `start..=end` (clamped) above or below it. Returns the
lines and where the caret's block is now — the copy when it went below.

# Examples

```rust
use crawrad_host::line_edits::duplicate_lines;

let lines = vec!["line 1".into()];
let (dup, start, end) = duplicate_lines(&lines, 0, 0, false);
assert_eq!(dup.len(), 2);
assert_eq!((start, end), (1, 1));
```
