---
id: props-props-default-row-height
kind: doc
target_file: src/props.rs
line_hint: 1813
author: 
created: 
title: "Props::default_row_height"
tags: []
signature: "pub fn default_row_height (default : i64) -> PropDesc"
---

Default row height of Grid (`DefaultRowHeight`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::default_row_height(24);
assert_eq!(p.key, "default_row_height");
```
