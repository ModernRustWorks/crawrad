---
id: diff-view-diff-row-tint
kind: doc
target_file: src/diff_view.rs
line_hint: 162
author: 
created: 
title: "diff_row_tint"
tags: []
signature: "pub fn diff_row_tint (kind : DiffRowKind) -> & 'static str"
---

The light tint color behind the changed side of a row.

The light tint behind the changed side of a row of `kind`: the IDE's
ground for an unchanged line.

# Examples

```rust
use crawrad_runtime_makepad::diff_view::{diff_row_tint, DiffRowKind};

assert_eq!(diff_row_tint(DiffRowKind::Same), "#1d1f20");
```
