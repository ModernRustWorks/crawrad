---
id: grid-itemlines
kind: doc
target_file: src/grid.rs
line_hint: 495
author: 
created: 
title: "ItemLines"
tags: []
signature: "pub struct ItemLines < 'a >"
---

Selected lines to render for a single portal list item slot.

# Examples

```rust
use crawrad_runtime::grid::ItemLines;

let lines = ItemLines {
    headers: Vec::new(),
    row: None,
    is_stale: false,
};
assert!(!lines.is_stale);
```
