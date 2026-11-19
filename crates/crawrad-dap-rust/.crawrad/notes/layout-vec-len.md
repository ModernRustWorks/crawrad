---
id: layout-vec-len
kind: doc
target_file: src/layout.rs
line_hint: 10
author: 
created: 
title: "VEC_LEN"
tags: []
signature: "pub const VEC_LEN : & [& str] = & [\"{}.len\"]"
---

From a `Vec` to its length.

# Examples

```rust
use crawrad_dap_rust::layout::VEC_LEN;

assert_eq!(VEC_LEN[0], "{}.len");
```
