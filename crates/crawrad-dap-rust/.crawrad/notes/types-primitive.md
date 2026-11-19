---
id: types-primitive
kind: doc
target_file: src/types.rs
line_hint: 8
author: 
created: 
title: "Primitive"
tags: []
signature: "pub enum Primitive"
---

A primitive element of a slice or `Vec`, as it sits in memory.

# Examples

```rust
use crawrad_dap_rust::types::Primitive;

assert_eq!(Primitive::of("i32"), Some(Primitive::I32));
```
