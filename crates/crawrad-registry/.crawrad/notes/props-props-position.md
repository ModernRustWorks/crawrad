---
id: props-props-position
kind: doc
target_file: src/props.rs
line_hint: 912
author: 
created: 
title: "Props::position"
tags: []
signature: "pub fn position (default : i64) -> PropDesc"
---

Current scroll or slider position (`Position`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::position(50);
assert_eq!(p.key, "position");
```
