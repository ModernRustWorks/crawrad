---
id: props-props-dataset
kind: doc
target_file: src/props.rs
line_hint: 434
author: 
created: 
title: "Props::dataset"
tags: []
signature: "pub fn dataset () -> PropDesc"
---

Reference to a dataset bound to an action (`Dataset`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::dataset();
assert_eq!(p.key, "dataset");
```
