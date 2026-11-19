---
id: layout-vec-ptr
kind: doc
target_file: src/layout.rs
line_hint: 13
author: 
created: 
title: "VEC_PTR"
tags: []
signature: "pub const VEC_PTR : & [& str] = & [\"{}.buf.inner.ptr.pointer.pointer\" , \"{}.buf.ptr.pointer.pointer\" , \"{}.buf.ptr.pointer\" ,]"
---

From a `Vec` to the address of its first element.

# Examples

```rust
use crawrad_dap_rust::layout::VEC_PTR;

assert!(!VEC_PTR.is_empty());
```
