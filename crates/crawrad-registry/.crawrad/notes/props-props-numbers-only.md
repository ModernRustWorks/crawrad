---
id: props-props-numbers-only
kind: doc
target_file: src/props.rs
line_hint: 1316
author: 
created: 
title: "Props::numbers_only"
tags: []
signature: "pub fn numbers_only (default : bool) -> PropDesc"
---

Allows only numbers to be typed (`NumbersOnly`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::numbers_only(true);
assert_eq!(p.key, "numbers_only");
```
