---
id: components-comum-makepad-widget-input
kind: doc
target_file: src/components/comum.rs
line_hint: 73
author: 
created: 
title: "MAKEPAD_WIDGET_INPUT"
tags: []
signature: "pub const MAKEPAD_WIDGET_INPUT : [f32 ; 4] = rgb (0x18 , 0x18 , 0x25)"
---

The background of a field that sinks into the paper.

The background of a field that SINKS: `Edit`, `Memo`, `ListBox`, `TreeView`,
`ComboBox`, and the track of `ScrollBar`. Mirrors RXP's `CTL_FUNDO`.

# Examples

```rust
use crawrad_runtime_makepad::components::comum::MAKEPAD_WIDGET_INPUT;

assert_eq!(MAKEPAD_WIDGET_INPUT[3], 1.0);
```
