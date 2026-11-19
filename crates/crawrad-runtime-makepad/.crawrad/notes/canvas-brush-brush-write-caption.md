---
id: canvas-brush-brush-write-caption
kind: doc
target_file: src/canvas/brush.rs
line_hint: 821
author: 
created: 
title: "Brush::write_caption"
tags: []
signature: "pub fn write_caption (& mut self , area : Rect , text : & str , c : [f32 ; 4] , centered : bool)"
---

Writes a caption where `&` becomes an accelerator underline.

Writes a CAPTION: the classic RAD `&` becomes an underline and does not reach
the screen.

The contract is Win32's, and the owner asked for it after seeing **`&OK`**
written on a button: `&` marks the access key and `&&` is a literal `&`.
It holds for `Button`, `CheckBox`, `RadioButton`, `GroupBox`, `Label`,
`Panel` and the `PageControl` tab — and it does **not** hold for content
(`Edit` text, `Memo` line, `ListBox` item), exactly as in the RXP.

# Examples

```rust
use makepad_widgets::{dvec2, Rect};
use crawrad_runtime_makepad::canvas::{Brush, Probe};
use crawrad_runtime_makepad::Transformacao;

let mut probe = Probe::default();
let mut brush = Brush::probing(&mut probe, Transformacao::new(0.0, 0.0, 1.0));
let r = Rect { pos: dvec2(0.0, 0.0), size: dvec2(100.0, 30.0) };
brush.write_caption(r, "&OK", [0.0, 0.0, 0.0, 1.0], true);
assert!(probe.marks().len() >= 1);
```
