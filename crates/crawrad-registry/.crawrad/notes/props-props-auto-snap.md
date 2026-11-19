---
id: props-props-auto-snap
kind: doc
target_file: src/props.rs
line_hint: 1880
author: 
created: 
title: "Props::auto_snap"
tags: []
signature: "pub fn auto_snap (default : bool) -> PropDesc"
---

Magnetic snapping to container edges (`AutoSnap`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::auto_snap(true);
assert_eq!(p.key, "auto_snap");
```
