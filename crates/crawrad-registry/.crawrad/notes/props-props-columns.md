---
id: props-props-columns
kind: doc
target_file: src/props.rs
line_hint: 781
author: 
created: 
title: "Props::columns"
tags: []
signature: "pub fn columns (default : i64) -> PropDesc"
---

How many columns a `RadioGroup` lays its items out in (`Columns`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::columns(2);
assert_eq!(p.key, "columns");
```
