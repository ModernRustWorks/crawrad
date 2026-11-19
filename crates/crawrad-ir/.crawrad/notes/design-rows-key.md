---
id: design-rows-key
kind: doc
target_file: src/design_rows.rs
line_hint: 9
author: 
created: 
title: "DESIGN_ROWS_KEY"
tags: []
signature: "pub const DESIGN_ROWS_KEY: &str = \"design_rows\""
---

The property of a `Grid` that says how many real rows the Designer shows.

# Examples

```rust
use crawrad_ir::design_rows::DESIGN_ROWS_KEY;

assert_eq!(DESIGN_ROWS_KEY, "design_rows");
```
