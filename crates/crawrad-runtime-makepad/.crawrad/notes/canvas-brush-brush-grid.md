---
id: canvas-brush-brush-grid
kind: doc
target_file: src/canvas/brush.rs
line_hint: 968
author: 
created: 
title: "Brush::grid"
tags: []
signature: "pub (crate) fn grid (& mut self , points : & [(f32 , f32)] , side : f64 , strong : bool)"
---

The dot grid, in ONE draw call (`draw_quad.rs:147-157`).

It lives here, and not in the canvas, because `begin_many_instances`
belongs to the brush: whoever instances the same shader N times is always
this layer.
