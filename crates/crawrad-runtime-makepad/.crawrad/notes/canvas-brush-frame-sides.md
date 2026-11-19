---
id: canvas-brush-frame-sides
kind: doc
target_file: src/canvas/brush.rs
line_hint: 119
author: 
created: 
title: "frame_sides"
tags: []
signature: "fn frame_sides (r : Rect , thickness : f64) -> [Rect ; 4]"
---

The four sides of a frame `thickness` thick, drawn INSIDE `r`.

A free function because two different LAYERS draw it: a component's frame (on
the stroke layer) and the form paper's (on the background layer, so as not to
open the stroke draw call too early — see the ordering note in the canvas
DSL).
