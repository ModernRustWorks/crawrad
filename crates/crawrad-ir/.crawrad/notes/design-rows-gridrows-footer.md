---
id: design-rows-gridrows-footer
kind: doc
target_file: src/design_rows.rs
line_hint: 61
author: 
created: 
title: "GridRows::footer"
tags: []
signature: "pub fn footer(&self) -> String"
---

The words of the badge at the grid's foot.

# Examples

```rust
use crawrad_ir::design_rows::GridRows;

let r = GridRows::Reading { columns: Vec::new(), limit: 8, table: "t".into() };
assert_eq!(r.footer(), "reading 8 rows\u{2026}");
```
