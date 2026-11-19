---
id: components-comum-makepad-widget
kind: doc
target_file: src/components/comum.rs
line_hint: 70
author: 
created: 
title: "MAKEPAD_WIDGET"
tags: []
signature: "pub const MAKEPAD_WIDGET : [f32 ; 4] = crate :: shared_theme :: ROLE_SURFACE"
---

The face of a control that rises above the paper.

The face of a control that RISES: `Button`, the scrollbar button, the
arrow button of `ComboBox`. Mirrors RXP's `CTL_FACE`.

# Examples

```rust
use crawrad_runtime_makepad::components::comum::MAKEPAD_WIDGET;

assert_eq!(MAKEPAD_WIDGET[3], 1.0);
```
