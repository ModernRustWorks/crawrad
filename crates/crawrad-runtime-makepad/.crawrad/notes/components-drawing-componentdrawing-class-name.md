---
id: components-drawing-componentdrawing-class-name
kind: doc
target_file: src/components/drawing.rs
line_hint: 79
author: 
created: 
title: "ComponentDrawing::class_name"
tags: []
signature: "fn class_name (& self) -> & 'static str"
---

The class name in the `crawrad-registry` catalogue (`"Button"`, `"Edit"`).

It is the ONLY time the class appears written out in the crate: the
dispatch compares against this, and nobody else compares a literal
`&str`.

# Examples

```rust
use crawrad_runtime_makepad::components::ComponentDrawing;
use crawrad_runtime_makepad::components::button::Desenho;

assert_eq!(Desenho.class_name(), "Button");
```
