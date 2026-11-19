---
id: props-props-max-buttons
kind: doc
target_file: src/props.rs
line_hint: 301
author: 
created: 
title: "Props::max_buttons"
tags: []
signature: "pub fn max_buttons (default : i64) -> PropDesc"
---

How many page buttons at most a `Pagination` shows (`MaxButtons`, #500).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::max_buttons(5);
assert_eq!(p.key, "max_buttons");
```
