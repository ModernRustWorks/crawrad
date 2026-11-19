---
id: design-rows-gridrows
kind: doc
target_file: src/design_rows.rs
line_hint: 29
author: 
created: 
title: "GridRows"
tags: []
signature: "pub enum GridRows"
---

Where a bound `Grid`'s design-time rows stand.

# Examples

```rust
use crawrad_ir::design_rows::GridRows;

let r = GridRows::Reading { columns: Vec::new(), limit: 8, table: "t".into() };
assert!(r.columns().is_empty());
```
