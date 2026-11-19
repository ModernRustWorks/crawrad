---
id: components-comum-makepad-selection
kind: doc
target_file: src/components/comum.rs
line_hint: 85
author: 
created: 
title: "MAKEPAD_SELECTION"
tags: []
signature: "pub const MAKEPAD_SELECTION : [f32 ; 4] = rgb (0x3A , 0x5A , 0x8C)"
---

The selected row of a list or tree.

The selected row of a `ListBox` or a `TreeView`. Mirrors RXP's
`CTL_SELECAO`.

# Examples

```rust
use crawrad_runtime_makepad::components::comum::MAKEPAD_SELECTION;

assert_eq!(MAKEPAD_SELECTION[3], 1.0);
```
