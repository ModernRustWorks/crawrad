---
id: line-edits-move-lines
kind: doc
target_file: src/line_edits.rs
line_hint: 43
author: 
created: 
title: "move_lines"
tags: []
signature: "pub fn move_lines (lines : & [String] , start : usize , end : usize , up : bool ,) -> (Vec < String > , usize , usize)"
---

Moves the line block up or down by one line.

The block `start..=end` (clamped) one line up or down, past the line next
to it. Returns the lines and where the block landed; at the edge nothing
moves.

# Examples

```rust
use crawrad_host::line_edits::move_lines;

let lines = vec!["a".into(), "b".into()];
let (moved, start, end) = move_lines(&lines, 1, 1, true);
assert_eq!(moved, vec!["b".to_string(), "a".to_string()]);
assert_eq!((start, end), (0, 0));
```
