---
id: color-dialog-colordialog-basic
kind: doc
target_file: src/color_dialog.rs
line_hint: 159
author: 
created: 
title: "ColorDialog::basic"
tags: []
signature: "pub fn basic () -> [(& 'static str , Color) ; 16]"
---

Returns the 16 standard palette colors with their classic display names.

# Examples

```rust
use crawrad_actions::color_dialog::ColorDialog;

let basic = ColorDialog::basic();
assert_eq!(basic.len(), 16);
```
