---
id: components-drawing-componentdrawing
kind: doc
target_file: src/components/drawing.rs
line_hint: 77
author: 
created: 
title: "ComponentDrawing"
tags: []
signature: "pub trait ComponentDrawing : Sync"
---

Everything a component class settles on its own inside this crate.

Adding a component to the canvas means **creating a file** with a type that
honours this contract and naming it once in the catalogue — and nothing else:
there is no `match` to extend, no class list to update ([`crate::CLASSES`] is
derived from the catalogue) and no function in the canvas to touch.

# Examples

```rust
use crawrad_runtime_makepad::components::ComponentDrawing;
use crawrad_runtime_makepad::components::button::Desenho;

let drawing: &dyn ComponentDrawing = &Desenho;
assert_eq!(drawing.class_name(), "Button");
```
