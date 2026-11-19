---
id: color-dialog-colordialog-fields
kind: doc
target_file: src/color_dialog.rs
line_hint: 133
author: 
created: 
title: "ColorDialog::fields"
tags: []
signature: "pub fn fields (& self) -> ColorFields"
---

Returns formatted field strings for hex and decimal channel values.

# Examples

```rust
use crawrad_actions::color_dialog::ColorDialog;
use crawrad_ir::Color;

let dlg = ColorDialog::new("Pick", Color::rgb(255, 255, 255), vec![]);
let f = dlg.fields();
assert_eq!(f.red, "255");
```
