---
id: props-props-auto-size
kind: doc
target_file: src/props.rs
line_hint: 1407
author: 
created: 
title: "Props::auto_size"
tags: []
signature: "pub fn auto_size (default : bool) -> PropDesc"
---

Automatically fits control dimensions to image size (`AutoSize`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::auto_size(true);
assert_eq!(p.key, "auto_size");
```
