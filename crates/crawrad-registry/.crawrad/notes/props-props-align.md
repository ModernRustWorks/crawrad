---
id: props-props-align
kind: doc
target_file: src/props.rs
line_hint: 346
author: 
created: 
title: "Props::align"
tags: []
signature: "pub fn align () -> PropDesc"
---

Automatic alignment within parent (`Align`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::align();
assert_eq!(p.key, "align");
```
