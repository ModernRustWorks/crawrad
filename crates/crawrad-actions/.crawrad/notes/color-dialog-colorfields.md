---
id: color-dialog-colorfields
kind: doc
target_file: src/color_dialog.rs
line_hint: 21
author: 
created: 
title: "ColorFields"
tags: []
signature: "pub struct ColorFields"
---

Text representation of color channels and hex code for input fields.

# Examples

```rust
use crawrad_actions::color_dialog::ColorFields;

let fields = ColorFields {
    hex: "#FFFFFF".to_string(),
    red: "255".to_string(),
    green: "255".to_string(),
    blue: "255".to_string(),
    alpha: "255".to_string(),
};
assert_eq!(fields.hex, "#FFFFFF");
```
