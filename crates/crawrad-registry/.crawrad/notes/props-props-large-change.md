---
id: props-props-large-change
kind: doc
target_file: src/props.rs
line_hint: 940
author: 
created: 
title: "Props::large_change"
tags: []
signature: "pub fn large_change (default : i64) -> PropDesc"
---

Large increment on trough clicks (`LargeChange`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::large_change(10);
assert_eq!(p.key, "large_change");
```
