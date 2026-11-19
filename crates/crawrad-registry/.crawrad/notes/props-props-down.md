---
id: props-props-down
kind: doc
target_file: src/props.rs
line_hint: 1552
author: 
created: 
title: "Props::down"
tags: []
signature: "pub fn down (default : bool) -> PropDesc"
---

Pressed / active toggle button state (`Down`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::down(true);
assert_eq!(p.key, "down");
```
