---
id: components-comum-makepad-panel-border
kind: doc
target_file: src/components/comum.rs
line_hint: 82
author: 
created: 
title: "MAKEPAD_PANEL_BORDER"
tags: []
signature: "pub const MAKEPAD_PANEL_BORDER : [f32 ; 4] = rgb (0x6E , 0x72 , 0x90)"
---

The border of a container. Mirrors RXP's `PAINEL_BORDA`.

More discreet than [`MAKEPAD_BORDER`] on purpose: a group frame must
not compete with the border of the controls that are INSIDE it.

# Examples

```rust
use crawrad_runtime_makepad::components::comum::MAKEPAD_PANEL_BORDER;

assert_eq!(MAKEPAD_PANEL_BORDER[3], 1.0);
```
