---
id: props-props-center
kind: doc
target_file: src/props.rs
line_hint: 1396
author: 
created: 
title: "Props::center"
tags: []
signature: "pub fn center (default : bool) -> PropDesc"
---

Centers the image in the control's rectangle (`Center`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::center(true);
assert_eq!(p.key, "center");
```
