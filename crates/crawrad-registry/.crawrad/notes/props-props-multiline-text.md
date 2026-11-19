---
id: props-props-multiline-text
kind: doc
target_file: src/props.rs
line_hint: 1327
author: 
created: 
title: "Props::multiline_text"
tags: []
signature: "pub fn multiline_text (key : & 'static str , display : & 'static str , default : & str) -> PropDesc"
---

Multiline text with custom key and display label.

# Examples

```rust
use crawrad_registry::Props;

let p = Props::multiline_text("custom", "Custom", "hello");
assert_eq!(p.key, "custom");
```
