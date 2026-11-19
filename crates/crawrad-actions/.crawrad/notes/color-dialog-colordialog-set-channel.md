---
id: color-dialog-colordialog-set-channel
kind: doc
target_file: src/color_dialog.rs
line_hint: 112
author: 
created: 
title: "ColorDialog::set_channel"
tags: []
signature: "pub fn set_channel (& mut self , ch : Channel , text : & str) -> Result < () , String >"
---

Sets one color channel from decimal text `0..=255`.

# Errors

Returns an error string if the text is not a valid decimal number between 0 and 255.

# Examples

```rust
use crawrad_actions::color_dialog::Channel;
use crawrad_actions::color_dialog::ColorDialog;
use crawrad_ir::Color;

let mut dlg = ColorDialog::new("Pick", Color::rgb(0, 0, 0), vec![]);
assert!(dlg.set_channel(Channel::Red, "255").is_ok());
assert_eq!(dlg.chosen.r, 255);
```
