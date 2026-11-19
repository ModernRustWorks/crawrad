---
id: types-is-pointer
kind: doc
target_file: src/types.rs
line_hint: 160
author: 
created: 
title: "is_pointer"
tags: []
signature: "pub fn is_pointer (name : & str) -> bool"
---

A pointer or reference, whose one child is the pointee (`*boxed`).

# Examples

```rust
use crawrad_dap_rust::types::is_pointer;

assert!(is_pointer("&i32"));
assert!(is_pointer("i32*"));
```
