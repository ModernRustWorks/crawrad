---
id: props-props-elevation
kind: doc
target_file: src/props.rs
line_hint: 1981
author: 
created: 
title: "Props::elevation"
tags: []
signature: "pub fn elevation (default : i64) -> PropDesc"
---

Shadow depth level (`Elevation`, #500).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::elevation(1);
assert_eq!(p.key, "elevation");
```
