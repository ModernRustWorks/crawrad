---
id: props-props-simple-text
kind: doc
target_file: src/props.rs
line_hint: 1585
author: 
created: 
title: "Props::simple_text"
tags: []
signature: "pub fn simple_text (default : impl Into < String >) -> PropDesc"
---

Text displayed in status bar simple panel (`SimpleText`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::simple_text("Ready");
assert_eq!(p.key, "simple_text");
```
