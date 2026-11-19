---
id: color-dialog-colordialog-pick-square
kind: doc
target_file: src/color_dialog.rs
line_hint: 72
author: 
created: 
title: "ColorDialog::pick_square"
tags: []
signature: "pub fn pick_square (& mut self , s : f32 , v : f32)"
---

Updates chosen color from saturation and value across the square picker.

Updates chosen color from saturation and value across the square picker, preserving hue and alpha.

# Examples

```rust
use crawrad_actions::color_dialog::ColorDialog;
use crawrad_ir::Color;

let mut dlg = ColorDialog::new("Pick", Color::rgb(255, 0, 0), vec![]);
dlg.pick_square(0.5, 0.5);
let (s, v) = dlg.saturation_value();
assert!((s - 0.5).abs() < 0.02);
```
