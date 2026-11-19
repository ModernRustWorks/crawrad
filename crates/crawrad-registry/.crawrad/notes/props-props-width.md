---
id: props-props-width
kind: doc
target_file: src/props.rs
line_hint: 2244
author: 
created: 
title: "Props::width"
tags: []
signature: "pub fn width (default : i64) -> PropDesc"
---

A column's width in pixels; `0` shares the space left.

# Examples

```rust
use crawrad_registry::Props;

assert_eq!(Props::width(0).key, "width");
```
