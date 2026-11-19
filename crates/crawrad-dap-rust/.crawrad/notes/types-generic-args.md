---
id: types-generic-args
kind: doc
target_file: src/types.rs
line_hint: 83
author: 
created: 
title: "generic_args"
tags: []
signature: "pub fn generic_args (name : & str) -> Vec < & str >"
---

Extracts top-level generic argument strings from a type name.

The generic arguments of `name`, split at the top level:
`alloc::vec::Vec<int, alloc::alloc::Global>` → `["int", "alloc::alloc::Global"]`.
A tuple or array argument keeps its commas: `Vec<(u32, i32), …>` → `(u32, i32)`.

# Examples

```rust
use crawrad_dap_rust::types::generic_args;

assert_eq!(generic_args("Vec<i32, Global>"), vec!["i32", "Global"]);
```
