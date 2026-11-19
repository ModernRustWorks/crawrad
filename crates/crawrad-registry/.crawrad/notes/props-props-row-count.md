---
id: props-props-row-count
kind: doc
target_file: src/props.rs
line_hint: 1763
author: 
created: 
title: "Props::row_count"
tags: []
signature: "pub fn row_count (default : i64) -> PropDesc"
---

Number of rows in Grid (`RowCount`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::row_count(10);
assert_eq!(p.key, "row_count");
```
