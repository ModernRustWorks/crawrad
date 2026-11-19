---
id: frameworks-of
kind: doc
target_file: src/frameworks.rs
line_hint: 56
author: 
created: 
title: "of"
tags: []
signature: "pub fn of (target : AlvoGerador) -> & 'static Framework"
---

The entry of a generator target.

# Panics

Panics if the generator target is not found in [`CATALOGUE`].

# Examples

```rust
use crawrad_codegen::AlvoGerador;
use crawrad_project::frameworks::of;

let fw = of(AlvoGerador::Makepad);
assert_eq!(fw.name, "Makepad");
```
