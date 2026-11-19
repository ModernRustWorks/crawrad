---
id: props-props-button-height
kind: doc
target_file: src/props.rs
line_hint: 1642
author: 
created: 
title: "Props::button_height"
tags: []
signature: "pub fn button_height (default : i64) -> PropDesc"
---

Toolbar button height (`ButtonHeight`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::button_height(24);
assert_eq!(p.key, "button_height");
```
