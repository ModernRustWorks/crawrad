---
id: canvas-draw-ruler
kind: doc
target_file: src/canvas.rs
line_hint: 1089
author: 
created: 
title: "draw_ruler"
tags: []
signature: "fn draw_ruler (p : & mut Brush , area : Rect , ponteiro : Option < DVec2 > , landing : Option < RectIr >)"
---

Paints the two ruler strips: background, ticks, numbers, and the pointer mark.

**No new coordinate calculation** (ADR 0019): where each tick falls comes from
[`crate::designer::ticks`], which queries the frame's [`Transformacao`].
This function only chooses stroke thickness and where to place the number.
