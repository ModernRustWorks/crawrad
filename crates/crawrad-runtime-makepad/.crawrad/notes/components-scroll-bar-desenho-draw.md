---
id: components-scroll-bar-desenho-draw
kind: doc
target_file: src/components/scroll_bar.rs
line_hint: 22
author: 
created: 
title: "Desenho::draw"
tags: []
signature: "fn draw (& self , p : & mut Brush , c : & MakepadCtl , r : Rect , _scene : & Scene)"
---

Draws the entire bar with track, buttons, and thumb.

The entire bar: track, the TWO buttons with arrows, and the thumb at the
document's `Position`.

**This is issue #119.** The previous version had the signature
`desenhar(&self, p, _c: &MakepadCtl, r, _scene)` — and the `_` confessed the
defect: the RXP reads `posicao_scroll` in four places and this runtime read it
in zero, so the thumb remained stuck at the beginning regardless of the
`Position`. There were no arrows at the ends, and `(r.size.x * 0.3).max(8.0)`
painted OUTSIDE the component in a narrow bar, because `max` has
no ceiling.

The calculation is now [`crate::components::bar::scroll_bar_geometry`], which is
the port of `barra_de_rolagem` from RXP (`crawrad-runtime-egui/src/lib.rs:2950`)
with its full rule — *the floor is a wish, the ceiling is a fact*.
A disabled `ScrollBar` comes out **dimmed**, and it is the same rule as #122:
gray arrow, empty track, and no thumb. A bar that shows the
thumb promises that it can be dragged; on a component with `Enabled: False`
that is the interface lying, and it is the same reason why the text of a
disabled control already comes out faint
([`crate::components::text_color`]).
