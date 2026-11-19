---
id: props-props-min
kind: doc
target_file: src/props.rs
line_hint: 884
author: 
created: 
title: "Props::min"
tags: []
signature: "pub fn min (default : i64) -> PropDesc"
---

Minimum value in numerical bounds (`Min`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::min(0);
assert_eq!(p.key, "min");
```
