---
id: types-short-name
kind: doc
target_file: src/types.rs
line_hint: 109
author: 
created: 
title: "short_name"
tags: []
signature: "pub fn short_name (name : & str) -> & str"
---

Returns the last path segment of a type name without generic parameters.

The last path segment without generics: `core::option::Option<i32>::Some<int>`
→ `Some`; `probe::Shape::Circle` → `Circle`.

# Examples

```rust
use crawrad_dap_rust::types::short_name;

assert_eq!(short_name("core::option::Option<i32>"), "Option");
```
