---
id: props-props-corner-radius
kind: doc
target_file: src/props.rs
line_hint: 1970
author: 
created: 
title: "Props::corner_radius"
tags: []
signature: "pub fn corner_radius (default : i64) -> PropDesc"
---

Rounding of card corners (`CornerRadius`, #500).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::corner_radius(8);
assert_eq!(p.key, "corner_radius");
```
