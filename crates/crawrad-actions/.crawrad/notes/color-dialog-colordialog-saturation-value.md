---
id: color-dialog-colordialog-saturation-value
kind: doc
target_file: src/color_dialog.rs
line_hint: 66
author: 
created: 
title: "ColorDialog::saturation_value"
tags: []
signature: "pub fn saturation_value (& self) -> (f32 , f32)"
---

Returns the saturation and value of the chosen color in `0.0..=1.0`.

# Examples

```rust
use crawrad_actions::color_dialog::ColorDialog;
use crawrad_ir::Color;

let dlg = ColorDialog::new("Pick", Color::rgb(255, 0, 0), vec![]);
let (s, v) = dlg.saturation_value();
assert!((s - 1.0).abs() < f32::EPSILON);
assert!((v - 1.0).abs() < f32::EPSILON);
```
