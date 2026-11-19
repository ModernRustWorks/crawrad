---
id: props-props-bevel-style
kind: doc
target_file: src/props.rs
line_hint: 1125
author: 
created: 
title: "Props::bevel_style"
tags: []
signature: "pub fn bevel_style (default : & 'static str) -> PropDesc"
---

Whether a `Bevel` looks sunk into the form or raised from it (#500).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::bevel_style("Lowered");
assert_eq!(p.key, "bevel_style");
```
