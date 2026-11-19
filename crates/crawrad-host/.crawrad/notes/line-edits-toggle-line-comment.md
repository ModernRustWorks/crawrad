---
id: line-edits-toggle-line-comment
kind: doc
target_file: src/line_edits.rs
line_hint: 4
author: 
created: 
title: "toggle_line_comment"
tags: []
signature: "pub fn toggle_line_comment (lines : & [String] , start_line : usize , end_line : usize) -> Vec < String >"
---

Toggles `//` line comments across the specified line range.

Toggles `//` comments on lines `start_line..=end_line` (clamped to the
text): when every non-empty line is already commented, uncomments them;
otherwise comments them all at the smallest indentation among them, so the
block stays aligned.

# Examples

```rust
use crawrad_host::line_edits::toggle_line_comment;

let lines = vec!["let a = 1;".to_string()];
let commented = toggle_line_comment(&lines, 0, 0);
assert_eq!(commented, vec!["// let a = 1;".to_string()]);
let uncommented = toggle_line_comment(&commented, 0, 0);
assert_eq!(uncommented, lines);
```
