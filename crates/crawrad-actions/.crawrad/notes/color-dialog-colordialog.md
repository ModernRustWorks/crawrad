---
id: color-dialog-colordialog
kind: doc
target_file: src/color_dialog.rs
line_hint: 31
author: 
created: 
title: "ColorDialog"
tags: []
signature: "pub struct ColorDialog"
---

The state of the Color dialog.

# Examples

```rust
use crawrad_actions::color_dialog::ColorDialog;
use crawrad_ir::Color;

let dlg = ColorDialog::new("Pick", Color::rgb(0, 0, 0), Vec::new());
assert_eq!(dlg.title, "Pick");
```
