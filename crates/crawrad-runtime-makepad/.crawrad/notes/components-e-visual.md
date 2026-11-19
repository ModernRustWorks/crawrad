---
id: components-e-visual
kind: doc
target_file: src/components/mod.rs
line_hint: 175
author: 
created: 
title: "e_visual"
tags: []
signature: "pub fn e_visual (classe : & str) -> bool"
---

Returns whether the class has its own painting in the application.

Does the class have painting in the application, or is it a non-visual?

The answer comes from the class file ([`ComponentDrawing::visual`]), and not
from a list: it was a `matches!("ActionList" | "Action" | …)` in `lib.rs`, and
a hand-written list next to a catalog is exactly the pair that gets out of
step — the non-visual was in the list and fell into the `generico` drawing.

An unknown class falls back to [`generic::Desenho`], whose `visual()` is the default
*yes*: that is what the RXP also assumes (`crawrad-runtime-egui/src/lib.rs:3296`).

# Examples

```rust
use crawrad_runtime_makepad::components::e_visual;

assert!(e_visual("Button"));
assert!(!e_visual("Timer"));
```
