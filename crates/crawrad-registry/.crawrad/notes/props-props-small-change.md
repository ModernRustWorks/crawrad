---
id: props-props-small-change
kind: doc
target_file: src/props.rs
line_hint: 926
author: 
created: 
title: "Props::small_change"
tags: []
signature: "pub fn small_change (default : i64) -> PropDesc"
---

Small increment on arrow clicks (`SmallChange`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::small_change(1);
assert_eq!(p.key, "small_change");
```
