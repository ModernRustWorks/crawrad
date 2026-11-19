---
id: numero
kind: doc
target_file: src/lib.rs
line_hint: 887
author: 
created: 
title: "numero"
tags: []
signature: "pub fn numero (v : f32) -> String"
---

Formats an `f32` as a Rust literal, ensuring the presence of a decimal point.

`f32` as a literal that always has a decimal point — `24` becomes `24.0`, otherwise Rust reads an
integer and does not compile where `f32` is expected.

# Examples

```rust
use crawrad_codegen::numero;

assert_eq!(numero(24.0), "24.0");
assert_eq!(numero(24.5), "24.5");
```
