---
id: types-rust-spelling
kind: doc
target_file: src/types.rs
line_hint: 251
author: 
created: 
title: "rust_spelling"
tags: []
signature: "pub fn rust_spelling (name : & str) -> String"
---

Translates C primitive type names in DWARF to their Rust equivalents.

`name` with LLDB's C primitives written the Rust way, as DWARF names them:
`Vec<unsigned int, alloc::alloc::Global>` → `Vec<u32, alloc::alloc::Global>`.

# Examples

```rust
use crawrad_dap_rust::types::rust_spelling;

assert_eq!(rust_spelling("int"), "i32");
```
