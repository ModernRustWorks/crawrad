---
id: grid-columns-title-from-field
kind: doc
target_file: src/grid_columns.rs
line_hint: 42
author: 
created: 
title: "title_from_field"
tags: []
signature: "pub fn title_from_field (field : & str) -> String"
---

The header a field name reads as: `customer_name` becomes `Customer name`.

# Examples

```rust
use crawrad_ir::grid_columns::title_from_field;

assert_eq!(title_from_field("customer_name"), "Customer name");
assert_eq!(title_from_field("id"), "Id");
```
