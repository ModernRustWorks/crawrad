---
id: types-primitive-decode
kind: doc
target_file: src/types.rs
line_hint: 56
author: 
created: 
title: "Primitive::decode"
tags: []
signature: "pub fn decode (self , b : & [u8]) -> String"
---

One element from its little-endian bytes, as `{:?}` prints it.

# Examples

```rust
use crawrad_dap_rust::types::Primitive;

assert_eq!(Primitive::I32.decode(&[42, 0, 0, 0]), "42");
```
