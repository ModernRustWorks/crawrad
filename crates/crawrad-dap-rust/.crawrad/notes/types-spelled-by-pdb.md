---
id: types-spelled-by-pdb
kind: doc
target_file: src/types.rs
line_hint: 222
author: 
created: 
title: "spelled_by_pdb"
tags: []
signature: "pub fn spelled_by_pdb (name : & str) -> bool"
---

Returns whether a type name follows MSVC PDB formatting conventions.

Whether LLDB took `name` verbatim from a PDB (`-msvc`) instead of printing
it from DWARF (`-gnu`). The two spell the same type differently, measured
on 30/09/2026: `Vec<i32,alloc::alloc::Global>`, `enum2$<…>`, `tuple$<…>`
from a PDB — already in Rust names, a bare `char` being Rust's `char` —
and `Vec<int, alloc::alloc::Global>` from DWARF, in C names, where a bare
`char` is an `i8`.

# Examples

```rust
use crawrad_dap_rust::types::spelled_by_pdb;

assert!(spelled_by_pdb("tuple$<i32,i32>"));
assert!(!spelled_by_pdb("Vec<int, alloc::alloc::Global>"));
```
