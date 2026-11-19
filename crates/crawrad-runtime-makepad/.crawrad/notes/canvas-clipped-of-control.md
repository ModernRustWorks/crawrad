---
id: canvas-clipped-of-control
kind: doc
target_file: src/canvas.rs
line_hint: 538
author: 
created: 
title: "Clipped::of_control"
tags: []
signature: "pub fn of_control (c : & MakepadCtl , form : RectIr) -> Self"
---

Returns the control rectangle together with its parent's clipping area.

What the canvas asks for a control: **the whole rectangle the document
gave it**, and the parent's clip on top of it.

The two halves are built here, together, and that is the point: this is
the single place where the law *"the clip never shrinks the drawing
rectangle"* can be broken, and it is the single place a test has to
watch. Handing the intersection to the control instead — so that it
"fits" — is the mutation the RXP test
`encolher_o_formulario_recorta_o_componente_e_nao_o_encolhe` exists to
kill, and the reason is the classic RAD: a `TListBox` cut in half is still a
whole `TListBox`, and dragging it back out brings the whole thing back.

# Examples

```rust,no_run
// MakepadCtl instances are managed by MakepadRuntime.
use crawrad_ir::Rect as RectIr;
use crawrad_runtime_makepad::canvas::Clipped;
use crawrad_runtime_makepad::MakepadCtl;

fn example(c: &MakepadCtl) {
    let clipped = Clipped::of_control(c, RectIr::new(0.0, 0.0, 100.0, 100.0));
    assert_eq!(clipped.item, c.rect);
}
```
