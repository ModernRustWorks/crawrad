---
id: components-check-box-marca-com-rotulo
kind: doc
target_file: src/components/check_box.rs
line_hint: 33
author: 
created: 
title: "marca_com_rotulo"
tags: []
signature: "pub (crate) fn marca_com_rotulo (p : & mut Brush , c : & MakepadCtl , r : Rect , redonda : bool)"
---

The square (or round) mark with the label beside it.

It lives here and `RadioButton` calls it: both are the same figure, and the only
difference is the radius. Composition instead of inheritance — and instead of a `match`
with two identical arms, which is where it started.

# The mark size, and why it came out wrong before

The side is `LADO_DA_MARCA` of DESIGN times the scale, **without clamping by the component
height** — this is the RXP semantics, which starts from the vertical center and uses
a fixed half-side (`crawrad-runtime-egui/src/lib.rs:2644-2648`). The old version
had `.min(r.size.y).max(4.0)`, which looks harmless but was not: what
dropped the dot to 4 px was not this clamp, but rather the doubled radius of
`Sdf2d::box` (see [`Brush::rounded_box`]), yet the clamp concealed the cause and
tied the mark to the rectangle, which is not the contract.
