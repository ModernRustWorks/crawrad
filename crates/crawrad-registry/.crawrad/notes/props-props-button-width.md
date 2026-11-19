---
id: props-props-button-width
kind: doc
target_file: src/props.rs
line_hint: 1631
author: 
created: 
title: "Props::button_width"
tags: []
signature: "pub fn button_width (default : i64) -> PropDesc"
---

Toolbar button width (`ButtonWidth`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::button_width(24);
assert_eq!(p.key, "button_width");
```
