---
id: color-dialog-colordialog-accept
kind: doc
target_file: src/color_dialog.rs
line_hint: 144
author: 
created: 
title: "ColorDialog::accept"
tags: []
signature: "pub fn accept (self) -> (Color , Vec < Color >)"
---

Accepts the dialog selection, returning chosen color and recent colors.

Accepts the dialog selection, returning the chosen color and an updated recent colors list
with the chosen color moved to the front (deduplicated and capped at [`RECENT_MAX`]).

# Examples

```rust
use crawrad_actions::color_dialog::ColorDialog;
use crawrad_ir::Color;

let dlg = ColorDialog::new("Pick", Color::rgb(255, 0, 0), vec![]);
let (chosen, recent) = dlg.accept();
assert_eq!(chosen, Color::rgb(255, 0, 0));
assert_eq!(recent, vec![Color::rgb(255, 0, 0)]);
```
