---
id: components-page-control-aba-i
kind: doc
target_file: src/components/page_control.rs
line_hint: 27
author: 
created: 
title: "aba_i"
tags: []
signature: "pub fn aba_i (r : RectIr , i : usize) -> Option < RectIr >"
---

Returns the rectangle of tab slot `i` in design units.

The rectangle of tab slot `i`, in design units, or `None` when it no longer
fits in the control.

The designer resolves clicks by slot `((x - r.x) / LARGURA_DA_ABA).floor()`.
This function is that calculation in reverse, taking the slot index.

# Examples

```rust
use crawrad_ir::Rect;
use crawrad_runtime_makepad::components::page_control::aba_i;

let r = Rect::new(0.0, 0.0, 200.0, 100.0);
let tab0 = aba_i(r, 0);
assert!(tab0.is_some());
```
