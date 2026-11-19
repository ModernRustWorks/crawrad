---
id: canvas-paint-clipped
kind: doc
target_file: src/canvas.rs
line_hint: 1081
author: 
created: 
title: "paint_clipped"
tags: []
signature: "fn paint_clipped (p : & mut Brush , clip : Option < Rect > , paint : impl FnOnce (& mut Brush))"
---

Paints with the clip when there is one, and with none when there is not.

It exists so that the question *"is there a clip?"* is answered in a single
place. [`Brush::with_clip`] demands a rectangle, because a clip is always a
rectangle; what carries the `Option` is the model, and this is where the two
meet.
