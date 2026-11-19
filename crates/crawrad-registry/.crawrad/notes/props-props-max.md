---
id: props-props-max
kind: doc
target_file: src/props.rs
line_hint: 898
author: 
created: 
title: "Props::max"
tags: []
signature: "pub fn max (default : i64) -> PropDesc"
---

Maximum value in numerical bounds (`Max`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::max(100);
assert_eq!(p.key, "max");
```
