---
id: props-props-anchors
kind: doc
target_file: src/props.rs
line_hint: 358
author: 
created: 
title: "Props::anchors"
tags: []
signature: "pub fn anchors () -> PropDesc"
---

Anchors tracking parent resizing (`Anchors`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::anchors();
assert_eq!(p.key, "anchors");
```
