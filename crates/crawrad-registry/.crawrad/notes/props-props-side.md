---
id: props-props-side
kind: doc
target_file: src/props.rs
line_hint: 2060
author: 
created: 
title: "Props::side"
tags: []
signature: "pub fn side (default : & str) -> PropDesc"
---

The edge from which a `Drawer` slides out (`Side`, #500).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::side("dsLeft");
assert_eq!(p.key, "side");
```
