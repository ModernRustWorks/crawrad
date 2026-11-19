---
id: canvas-brush-brush-expand-marker
kind: doc
target_file: src/canvas/brush.rs
line_hint: 913
author: 
created: 
title: "Brush::expand_marker"
tags: []
signature: "fn expand_marker (& mut self , r : Rect , c : [f32 ; 4] , expanded : bool)"
---

The `[-]` / `[+]` the Windows `TreeView` puts before whoever has children.

**Drawn, not written.** The RXP uses the `U+2212` / `+` glyphs
(`crawrad-runtime-egui/src/lib.rs:2233`), which it can because its font is
the egui theme's; here the font is `makepad-widgets`'s
`THEME_FONT_REGULAR` and a glyph it does not have comes out as an empty
box — that is how `\u{f0c7}` ended up written out in full on this house's
screen. Two little boxes and one stroke depend on no font at all.
