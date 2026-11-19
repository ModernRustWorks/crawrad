---
id: types-in-rust-spelling
kind: doc
target_file: src/types.rs
line_hint: 227
author: 
created: 
title: "in_rust_spelling"
tags: []
signature: "pub fn in_rust_spelling (arg : & str , container : & str) -> String"
---

`arg`, a generic argument taken from `container`, in the Rust spelling.

# Examples

```rust
use crawrad_dap_rust::types::in_rust_spelling;

assert_eq!(in_rust_spelling("int", "Vec<int>"), "i32");
```
