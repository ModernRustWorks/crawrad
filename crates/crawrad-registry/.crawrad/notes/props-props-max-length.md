---
id: props-props-max-length
kind: doc
target_file: src/props.rs
line_hint: 1281
author: 
created: 
title: "Props::max_length"
tags: []
signature: "pub fn max_length (default : i64) -> PropDesc"
---

The most characters the text takes; `0` is no limit (`MaxLength`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::max_length(50);
assert_eq!(p.key, "max_length");
```
