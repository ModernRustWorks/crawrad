---
id: props-props-orientation
kind: doc
target_file: src/props.rs
line_hint: 1459
author: 
created: 
title: "Props::orientation"
tags: []
signature: "pub fn orientation (default : & 'static str) -> PropDesc"
---

Slider or bar orientation (`Orientation`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::orientation("Horizontal");
assert_eq!(p.key, "orientation");
```
