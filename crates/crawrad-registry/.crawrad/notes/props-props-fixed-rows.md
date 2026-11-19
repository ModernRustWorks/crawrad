---
id: props-props-fixed-rows
kind: doc
target_file: src/props.rs
line_hint: 1788
author: 
created: 
title: "Props::fixed_rows"
tags: []
signature: "pub fn fixed_rows (default : i64) -> PropDesc"
---

Number of fixed header rows in Grid (`FixedRows`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::fixed_rows(1);
assert_eq!(p.key, "fixed_rows");
```
