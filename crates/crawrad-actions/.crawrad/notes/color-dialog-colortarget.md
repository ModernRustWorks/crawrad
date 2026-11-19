---
id: color-dialog-colortarget
kind: doc
target_file: src/color_dialog.rs
line_hint: 165
author: 
created: 
title: "ColorTarget"
tags: []
signature: "pub enum ColorTarget"
---

Destination receiving the chosen color when OK is pressed.

# Examples

```rust
use crawrad_actions::color_dialog::ColorTarget;

let target = ColorTarget::SchemeField(0);
assert!(matches!(target, ColorTarget::SchemeField(0)));
```
