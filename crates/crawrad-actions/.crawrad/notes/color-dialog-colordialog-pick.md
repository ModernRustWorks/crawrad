---
id: color-dialog-colordialog-pick
kind: doc
target_file: src/color_dialog.rs
line_hint: 85
author: 
created: 
title: "ColorDialog::pick"
tags: []
signature: "pub fn pick (& mut self , c : Color)"
---

Picks a basic or recent color preset, preserving current alpha.

Updates hue unless the picked color is grey.

# Examples

```rust
use crawrad_actions::color_dialog::ColorDialog;
use crawrad_ir::Color;

let mut dlg = ColorDialog::new("Pick", Color::rgb(0, 0, 0), vec![]);
dlg.pick(Color::rgb(0, 255, 0));
assert_eq!(dlg.chosen, Color::rgb(0, 255, 0));
```
