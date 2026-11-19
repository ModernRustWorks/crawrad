---
id: props-props-smooth
kind: doc
target_file: src/props.rs
line_hint: 1730
author: 
created: 
title: "Props::smooth"
tags: []
signature: "pub fn smooth (default : bool) -> PropDesc"
---

Progress bar with smooth continuous advance (`Smooth`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::smooth(true);
assert_eq!(p.key, "smooth");
```
