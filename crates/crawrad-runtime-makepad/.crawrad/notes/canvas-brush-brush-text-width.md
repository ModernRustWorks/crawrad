---
id: canvas-brush-brush-text-width
kind: doc
target_file: src/canvas/brush.rs
line_hint: 790
author: 
created: 
title: "Brush::text_width"
tags: []
signature: "fn text_width (& mut self , text : & str) -> f64"
---

The width `text` takes up, in screen pixels.

Measured with `DrawText::layout`
(`makepad-draw-1.0.0/src/shader/draw_text.rs:323`), which returns
`size_in_lpxs` **without** `font_scale` applied — the drawing is what
multiplies it (`:179`), and that is why we multiply here too.
