---
id: components-grid-grid
kind: doc
target_file: src/components/grid.rs
line_hint: 4
author: 
created: 
title: "grid"
tags: []
signature: "pub fn grid () -> ComponentDesc"
---

`Grid`: The unified tabular data grid of CrawRAD (ADR 0041).

Capable of working statically as an unbound string grid or reactively
connected to a data engine and table.

# Examples

```rust
use crawrad_registry::grid;

let desc = grid();
assert_eq!(desc.class, "Grid");
```
