---
id: grid-marks
kind: doc
target_file: src/grid.rs
line_hint: 52
author: 
created: 
title: "Marks"
tags: []
signature: "pub struct Marks"
---

The marks a grid writes into its text, chosen by the runtime that paints it.

A mark the font has no glyph for comes out as an empty box, and fonts
differ: Makepad's theme font draws `▲`/`▼`, egui's default fonts only
`⏶`/`⏷`. So the runtime says which marks its font can draw.

# Examples

```rust
use crawrad_runtime::grid::Marks;

let marks = Marks::default();
assert_eq!(marks.ascending, " ▲");
```
