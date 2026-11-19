---
id: props-props-stretch
kind: doc
target_file: src/props.rs
line_hint: 1374
author: 
created: 
title: "Props::stretch"
tags: []
signature: "pub fn stretch (default : bool) -> PropDesc"
---

Resizes the image to fill the control's dimensions (`Stretch`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::stretch(true);
assert_eq!(p.key, "stretch");
```
