---
id: dialogs-dialogkind-icon-color-rgb
kind: doc
target_file: src/dialogs.rs
line_hint: 53
author: 
created: 
title: "DialogKind::icon_color_rgb"
tags: []
signature: "pub const fn icon_color_rgb (& self) -> (u8 , u8 , u8)"
---

Standard RGB color for the icon.

# Examples

```rust
use crawrad_runtime::dialogs::DialogKind;

assert_eq!(DialogKind::Information.icon_color_rgb(), (121, 192, 255));
```
