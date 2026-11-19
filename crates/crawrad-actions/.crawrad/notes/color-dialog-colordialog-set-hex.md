---
id: color-dialog-colordialog-set-hex
kind: doc
target_file: src/color_dialog.rs
line_hint: 100
author: 
created: 
title: "ColorDialog::set_hex"
tags: []
signature: "pub fn set_hex (& mut self , text : & str) -> Result < () , String >"
---

Sets the chosen color from a hexadecimal string.

Sets the chosen color from a hex string (with or without `#`, 6 or 8 hex digits).

# Errors

Returns an error string if the text is not a valid hex color.

# Examples

```rust
use crawrad_actions::color_dialog::ColorDialog;
use crawrad_ir::Color;

let mut dlg = ColorDialog::new("Pick", Color::rgb(0, 0, 0), vec![]);
assert!(dlg.set_hex("#FF0000").is_ok());
assert_eq!(dlg.chosen, Color::rgb(255, 0, 0));
```
