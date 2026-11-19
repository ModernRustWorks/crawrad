---
id: canvas-content-clip-of
kind: doc
target_file: src/canvas.rs
line_hint: 552
author: 
created: 
title: "content_clip_of"
tags: []
signature: "pub fn content_clip_of (c : & MakepadCtl) -> Option < RectIr >"
---

The rectangle that clips a control's **own content** — its own box.

# What the owner saw, and it is the issue #129

> *"Did you see that the item label overlapped, right?"*

The `delta` of a `ListBox` was painted **below the bottom edge of the
`ListBox` itself**, on top of the scroll bar of the container beneath it.
The number of lines that fit is measured, but the last one was drawn whole
anyway and spilled over the neighbour. The RXP closed the same hole in
`crawrad-runtime-egui/src/lib.rs:2466` and guards it with
`nenhum_componente_pinta_fora_da_propria_caixa`.

# This is a SECOND clip, and it does not replace the first

[`clip_of`] answers *where the parent lets the child appear* (issue #111);
this one answers *where the control's own ink may fall*. The two nest, and
Makepad intersects them by itself on the turtle stack
(`makepad-draw-1.0.0/src/turtle.rs:749-758`) — so neither has to know about
the other.

# And it does NOT touch what a container paints over its children

The scroll bar of a `ScrollBox` is chrome of the container
([`crate::components::ComponentDrawing::draw_over`], issue #131): it
legitimately covers a child. What may not leave the box is the control's own
**content**. The two laws live in different steps of the frame
([`Step`]), and that is why they do not fight.

`None` means *nothing clips it*, and exactly one thing in the catalog
answers that: the non-visual seal, which writes the instance name below
itself on purpose. The reason is in
[`crate::components::ComponentDrawing::clips_to_own_box`], and the test
that keeps the exception from becoming a hole was written **before** the
clip existed.

# Examples

```rust,no_run
// MakepadCtl instances are managed by MakepadRuntime.
use crawrad_runtime_makepad::canvas::content_clip_of;
use crawrad_runtime_makepad::MakepadCtl;

fn example(c: &MakepadCtl) {
    let _clip = content_clip_of(c);
}
```
