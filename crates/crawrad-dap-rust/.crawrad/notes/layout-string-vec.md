---
id: layout-string-vec
kind: doc
target_file: src/layout.rs
line_hint: 22
author: 
created: 
title: "STRING_VEC"
tags: []
signature: "pub const STRING_VEC : & str = \"{}.vec\""
---

A `String` is a `Vec<u8>` called `vec`.

# Examples

```rust
use crawrad_dap_rust::layout::STRING_VEC;

assert_eq!(STRING_VEC, "{}.vec");
```
