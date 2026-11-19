---
id: targets-egui-layout-e-visual
kind: doc
target_file: src/targets/egui/layout.rs
line_hint: 7
author: 
created: 
title: "e_visual"
tags: []
signature: "pub fn e_visual (class : & str) -> bool"
---

The class draws something on the form (has a rectangle).

# Examples

```rust
use crawrad_codegen::targets::egui::layout::e_visual;

assert!(e_visual("Button"));
assert!(!e_visual("Timer"));
```
