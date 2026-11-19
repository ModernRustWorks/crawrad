---
id: handler-name-is-handler-ident
kind: doc
target_file: src/handler_name.rs
line_hint: 32
author: 
created: 
title: "is_handler_ident"
tags: []
signature: "pub fn is_handler_ident (name : & str) -> bool"
---

Returns whether `name` can be a Rust function name.

Whether `name` can be a Rust function name: ASCII letter or `_` first, then letters, digits or `_`.

# Examples

```rust
use crawrad_ir::is_handler_ident;

assert!(is_handler_ident("btn_ok_click"));
assert!(!is_handler_ident("123_click"));
assert!(!is_handler_ident(""));
```
