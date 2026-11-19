---
id: line-edits-delete-lines
kind: doc
target_file: src/line_edits.rs
line_hint: 93
author: 
created: 
title: "delete_lines"
tags: []
signature: "pub fn delete_lines (lines : & [String] , start : usize , end : usize) -> (Vec < String > , usize)"
---

Deletes the lines in the given range, keeping at least one empty line.

The text without the block `start..=end` (clamped), and the line the caret
goes to. The text never becomes empty: one empty line stays.

# Examples

```rust
use crawrad_host::line_edits::delete_lines;

let lines = vec!["a".into(), "b".into()];
let (remaining, caret) = delete_lines(&lines, 0, 0);
assert_eq!(remaining, vec!["b".to_string()]);
assert_eq!(caret, 0);
```
