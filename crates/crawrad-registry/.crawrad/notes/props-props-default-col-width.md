---
id: props-props-default-col-width
kind: doc
target_file: src/props.rs
line_hint: 1799
author: 
created: 
title: "Props::default_col_width"
tags: []
signature: "pub fn default_col_width (default : i64) -> PropDesc"
---

Default column width of Grid (`DefaultColWidth`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::default_col_width(64);
assert_eq!(p.key, "default_col_width");
```
