---
id: design-rows-gridrows-kinds
kind: doc
target_file: src/design_rows.rs
line_hint: 63
author: 
created: 
title: "GridRows::kinds"
tags: []
signature: "pub fn kinds (& self) -> & [String]"
---

The column types of the last read (`INTEGER`, `REAL`, `TEXT`...).

Empty until rows arrive.

# Examples

```rust
use crawrad_ir::design_rows::GridRows;

let r = GridRows::Offline { columns: vec!["id".into()], reason: "x".into(), table: "t".into() };
assert!(r.kinds().is_empty());
```
