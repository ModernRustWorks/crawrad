---
id: drawing-verb-all
kind: doc
target_file: src/drawing.rs
line_hint: 170
author: 
created: 
title: "Verb::all"
tags: []
signature: "pub fn all (stroke : Stroke) -> Self"
---

The verb that paints the entire rectangle — the common case.

# Examples

```rust
use crawrad_ir::drawing::{Area, Ink, Stroke, Verb};

let verb = Verb::all(Stroke::Check { color: Ink::Accent });
assert_eq!(verb.area, Area::All);
```
