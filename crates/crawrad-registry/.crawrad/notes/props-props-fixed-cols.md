---
id: props-props-fixed-cols
kind: doc
target_file: src/props.rs
line_hint: 1777
author: 
created: 
title: "Props::fixed_cols"
tags: []
signature: "pub fn fixed_cols (default : i64) -> PropDesc"
---

Number of fixed side header columns in Grid (`FixedCols`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::fixed_cols(1);
assert_eq!(p.key, "fixed_cols");
```
