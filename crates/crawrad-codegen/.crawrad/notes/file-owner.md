---
id: file-owner
kind: doc
target_file: src/lib.rs
line_hint: 137
author: 
created: 
title: "Dono"
tags: []
signature: "pub enum Dono"
---

Who owns the file — and this is what decides whether it can be overwritten.

# Examples

```rust
use crawrad_codegen::Dono;

assert_eq!(Dono::Gerador, Dono::Gerador);
```
