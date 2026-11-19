---
id: color-dialog-colordialog-hue
kind: doc
target_file: src/color_dialog.rs
line_hint: 61
author: 
created: 
title: "ColorDialog::hue"
tags: []
signature: "pub fn hue (& self) -> f32"
---

The active hue in degrees `0.0..360.0`.

# Examples

```rust
use crawrad_actions::color_dialog::ColorDialog;
use crawrad_ir::Color;

let dlg = ColorDialog::new("Pick", Color::rgb(255, 0, 0), vec![]);
assert_eq!(dlg.hue(), 0.0);
```
