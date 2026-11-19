---
id: types-vec-element
kind: doc
target_file: src/types.rs
line_hint: 139
author: 
created: 
title: "vec_element"
tags: []
signature: "pub fn vec_element (name : & str) -> Option < & str >"
---

`Some(element)` for `Vec<element, …>`.

# Examples

```rust
use crawrad_dap_rust::types::vec_element;

assert_eq!(vec_element("Vec<i32, Global>"), Some("i32"));
```
