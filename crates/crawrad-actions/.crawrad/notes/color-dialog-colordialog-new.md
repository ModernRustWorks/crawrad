---
id: color-dialog-colordialog-new
kind: doc
target_file: src/color_dialog.rs
line_hint: 47
author: 
created: 
title: "ColorDialog::new"
tags: []
signature: "pub fn new (title : impl Into < String > , current : Color , recent : Vec < Color >) -> Self"
---

Creates a new color dialog state.

Creates a new dialog state initialized from the given current color and recent colors list.

# Examples

```rust
use crawrad_actions::color_dialog::ColorDialog;
use crawrad_ir::Color;

let dlg = ColorDialog::new("Pick Color", Color::rgb(255, 0, 0), vec![]);
assert_eq!(dlg.current, Color::rgb(255, 0, 0));
```
