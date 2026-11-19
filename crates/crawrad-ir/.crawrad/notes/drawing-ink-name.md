---
id: drawing-ink-name
kind: doc
target_file: src/drawing.rs
line_hint: 64
author: 
created: 
title: "Ink::name"
tags: []
signature: "pub fn name (self) -> & 'static str"
---

Its name in the file — the reverse of [`Self::from_name`].

# Examples

```rust
use crawrad_ir::drawing::Ink;

assert_eq!(Ink::Text.name(), "text");
```
