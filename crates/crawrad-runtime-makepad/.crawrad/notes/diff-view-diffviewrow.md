---
id: diff-view-diffviewrow
kind: doc
target_file: src/diff_view.rs
line_hint: 182
author: 
created: 
title: "DiffViewRow"
tags: []
signature: "pub struct DiffViewRow < 'a >"
---

One row of the side-by-side diff view.

# Examples

```rust
use crawrad_runtime_makepad::diff_view::{DiffRowKind, DiffViewRow};

let row = DiffViewRow {
    kind: DiffRowKind::Same,
    left: Some((1, "hello")),
    right: Some((1, "hello")),
};
assert_eq!(row.kind, DiffRowKind::Same);
```
