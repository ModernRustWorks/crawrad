---
id: props-props-step
kind: doc
target_file: src/props.rs
line_hint: 1445
author: 
created: 
title: "Props::step"
tags: []
signature: "pub fn step (default : i64) -> PropDesc"
---

Step increment (`Step` of ProgressBar).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::step(10);
assert_eq!(p.key, "step");
```
