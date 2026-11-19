---
id: drawing-ink
kind: doc
target_file: src/drawing.rs
line_hint: 36
author: 
created: 
title: "Ink"
tags: []
signature: "pub enum Ink"
---

The ROLE of a color in drawing — never its value.

# Examples

```rust
use crawrad_ir::drawing::Ink;

assert_eq!(Ink::Surface.name(), "surface");
assert_eq!(Ink::from_name("border"), Some(Ink::Border));
```
