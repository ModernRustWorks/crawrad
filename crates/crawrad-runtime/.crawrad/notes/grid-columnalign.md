---
id: grid-columnalign
kind: doc
target_file: src/grid.rs
line_hint: 10
author: 
created: 
title: "ColumnAlign"
tags: []
signature: "pub enum ColumnAlign"
---

Where a column's text sits in its cell (#936).

# Examples

```rust
use crawrad_runtime::grid::ColumnAlign;

assert_ne!(ColumnAlign::Left, ColumnAlign::Right);
```
