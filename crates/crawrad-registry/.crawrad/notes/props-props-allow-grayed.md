---
id: props-props-allow-grayed
kind: doc
target_file: src/props.rs
line_hint: 335
author: 
created: 
title: "Props::allow_grayed"
tags: []
signature: "pub fn allow_grayed (default : bool) -> PropDesc"
---

Whether a click can reach `cbGrayed` (`AllowGrayed`, #445).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::allow_grayed(true);
assert_eq!(p.key, "allow_grayed");
```
