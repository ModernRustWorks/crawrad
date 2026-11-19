---
id: components-drawing-componentdrawing-clips-to-own-box
kind: doc
target_file: src/components/drawing.rs
line_hint: 100
author: 
created: 
title: "ComponentDrawing::clips_to_own_box"
tags: []
signature: "fn clips_to_own_box (& self) -> bool"
---

Returns true if this control's painting is confined to its rectangle.

Is this control's own painting confined to its own rectangle?

# What the owner saw, and it is the issue #129

> *"You saw that the item label overlapped, right?"* — the `delta` of a `ListBox`
> drawn **below the bottom edge of the `ListBox` itself**, over the
> scroll bar of the container underneath.

Every control that paints content answers **yes**, and the canvas holds
it to that: a line of text, an item, a caption, a scroll bar of its own
— none of it may leave the box the document drew. The RXP states the
same law in `crawrad-runtime-egui/src/lib.rs:2466` and guards it with
`nenhum_componente_pinta_fora_da_propria_caixa`.

# The one that answers NO, and why the exception is not a hole

The **non-visual seal** writes the instance name *below* its rectangle
on purpose ([`crate::components::non_visual::name_box`]) — that is
how a classic RAD IDE shows a `TActionList`, icon above and name under it. It is
not content overflowing a control; it is a **designer overlay** that has
no box to respect, and a blind clip would erase the name of every
`ActionList`, `ImageList`, `Timer` and `Action` on the form.

The answer belongs to the control, not to a list in the canvas: adding a
component that paints outside itself means overriding this, in its own
file, next to the drawing that needs it.
