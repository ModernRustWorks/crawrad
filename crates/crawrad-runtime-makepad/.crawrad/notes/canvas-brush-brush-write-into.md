---
id: canvas-brush-brush-write-into
kind: doc
target_file: src/canvas/brush.rs
line_hint: 849
author: 
created: 
title: "Brush::write_into"
tags: []
signature: "fn write_into (& mut self , area : Rect , text : & str , c : [f32 ; 4] , centered : bool) -> Option < Rect >"
---

Writes text and returns the rectangle where the ink landed.

Writes and returns the rectangle where the ink landed — `None` when
nothing was written.
