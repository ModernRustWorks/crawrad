---
id: props-props-text
kind: doc
target_file: src/props.rs
line_hint: 140
author: 
created: 
title: "Props::text"
tags: []
signature: "pub fn text (default : & str) -> PropDesc"
---

Text content of edit controls (`Text`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::text("Hello");
assert_eq!(p.key, "text");
```
