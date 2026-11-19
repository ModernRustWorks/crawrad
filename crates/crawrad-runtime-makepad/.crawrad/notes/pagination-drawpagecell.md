---
id: pagination-drawpagecell
kind: doc
target_file: src/pagination.rs
line_hint: 136
author: 
created: 
title: "DrawPageCell"
tags: []
signature: "pub struct DrawPageCell"
---

The shader of one page cell: fill and outline colours.

# Examples

```rust,no_run
// Makepad shader structures require the Makepad application runtime.
use crawrad_runtime_makepad::pagination::DrawPageCell;

fn check_cell(cell: &DrawPageCell) {
    let _c = cell.color;
}
```
