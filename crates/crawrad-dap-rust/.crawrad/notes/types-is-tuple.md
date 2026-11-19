---
id: types-is-tuple
kind: doc
target_file: src/types.rs
line_hint: 236
author: 
created: 
title: "is_tuple"
tags: []
signature: "pub fn is_tuple (name : & str) -> bool"
---

A tuple: `(u32, i32)` from DWARF, `tuple$<u32,i32>` from a PDB.

# Examples

```rust
use crawrad_dap_rust::types::is_tuple;

assert!(is_tuple("(i32, i32)"));
assert!(is_tuple("tuple$<i32,i32>"));
```
