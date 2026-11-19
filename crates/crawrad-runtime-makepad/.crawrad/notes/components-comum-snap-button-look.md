---
id: components-comum-snap-button-look
kind: doc
target_file: src/components/comum.rs
line_hint: 55
author: 
created: 
title: "snap_button_look"
tags: []
signature: "pub const fn snap_button_look (on : bool) -> [[f32 ; 4] ; 3]"
---

Colors for the designer bar's Snap button.

The designer bar's Snap button (#564): background, border and text, lit in
the accent while Snap is on, the IDE's plain button while it is off.

# Examples

```rust
use crawrad_runtime_makepad::components::comum::snap_button_look;

let colors = snap_button_look(true);
assert_eq!(colors.len(), 3);
```
