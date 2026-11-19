---
id: line-edits-selection-line-range
kind: doc
target_file: src/line_edits.rs
line_hint: 153
author: 
created: 
title: "selection_line_range"
tags: []
signature: "pub fn selection_line_range (start_line : usize , end_line : usize , end_byte : usize) -> (usize , usize)"
---

Computes the zero-based inclusive line range covered by a selection.

Calculates the zero-based inclusive line range `(start_line, end_line)` from
a selection. When the selection ends at column 0 of a following line, that
last line is excluded.

# Examples

```rust
use crawrad_host::line_edits::selection_line_range;

assert_eq!(selection_line_range(1, 3, 0), (1, 2));
assert_eq!(selection_line_range(1, 3, 5), (1, 3));
```
