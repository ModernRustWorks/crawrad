---
id: diff-view-diff-row-accent
kind: doc
target_file: src/diff_view.rs
line_hint: 172
author: 
created: 
title: "diff_row_accent"
tags: []
signature: "pub fn diff_row_accent (kind : DiffRowKind) -> Option < & 'static str >"
---

The accent color of a row change kind, or None for unchanged lines.

The colour of a change of `kind`: the gutter mark, the line number and the
legend's swatch. `None` for an unchanged line.

# Examples

```rust
use crawrad_runtime_makepad::diff_view::{diff_row_accent, DiffRowKind};

assert_eq!(diff_row_accent(DiffRowKind::Same), None);
assert!(diff_row_accent(DiffRowKind::Added).is_some());
```
