---
id: components-generic-desenho
kind: doc
target_file: src/components/generic.rs
line_hint: 20
author: 
created: 
title: "Desenho"
tags: []
signature: "pub struct Desenho"
---

The fallback drawing, for a class with no file of its own.

# Examples

```rust
use crawrad_runtime_makepad::components::ComponentDrawing;
use crawrad_runtime_makepad::components::generic::Desenho;

assert_eq!(Desenho.class_name(), "");
```
