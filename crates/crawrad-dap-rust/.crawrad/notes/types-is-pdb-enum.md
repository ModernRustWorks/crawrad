---
id: types-is-pdb-enum
kind: doc
target_file: src/types.rs
line_hint: 241
author: 
created: 
title: "is_pdb_enum"
tags: []
signature: "pub fn is_pdb_enum (name : & str) -> bool"
---

A Rust `enum` as a PDB describes it (`-msvc`).

# Examples

```rust
use crawrad_dap_rust::types::is_pdb_enum;

assert!(is_pdb_enum("enum2$<Option<i32>>"));
```
