---
id: color-dialog-colordialog-pick-hue
kind: doc
target_file: src/color_dialog.rs
line_hint: 77
author: 
created: 
title: "ColorDialog::pick_hue"
tags: []
signature: "pub fn pick_hue (& mut self , fraction : f32)"
---

Updates chosen color and hue from a fraction down the hue strip.

Updates chosen color and hue from a fraction down the hue strip, preserving saturation, value, and alpha.

# Examples

```rust
use crawrad_actions::color_dialog::ColorDialog;
use crawrad_ir::Color;

let mut dlg = ColorDialog::new("Pick", Color::rgb(255, 0, 0), vec![]);
dlg.pick_hue(0.5);
assert!((dlg.hue() - 180.0).abs() < 0.01);
```
