---
id: design-rows-gridrows-columns
kind: doc
target_file: src/design_rows.rs
line_hint: 53
author: 
created: 
title: "GridRows::columns"
tags: []
signature: "pub fn columns(&self) -> &[String]"
---

The header the grid draws: the columns it knows.

# Examples

```rust
use crawrad_ir::design_rows::GridRows;

let r = GridRows::Offline { columns: vec!["id".into()], reason: "x".into(), table: "t".into() };
assert_eq!(r.columns(), ["id".to_string()]);
```
