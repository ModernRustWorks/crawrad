---
id: props-props-col-count
kind: doc
target_file: src/props.rs
line_hint: 1752
author: 
created: 
title: "Props::col_count"
tags: []
signature: "pub fn col_count (default : i64) -> PropDesc"
---

Number of columns in Grid (`ColCount`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::col_count(5);
assert_eq!(p.key, "col_count");
```
