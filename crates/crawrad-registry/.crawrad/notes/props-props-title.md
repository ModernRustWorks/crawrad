---
id: props-props-title
kind: doc
target_file: src/props.rs
line_hint: 1653
author: 
created: 
title: "Props::title"
tags: []
signature: "pub fn title (default : impl Into < String >) -> PropDesc"
---

File dialog title (`Title`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::title("Open File");
assert_eq!(p.key, "title");
```
