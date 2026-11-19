---
id: canvas-brush-brush-ruler
kind: doc
target_file: src/canvas/brush.rs
line_hint: 687
author: 
created: 
title: "Brush::ruler"
tags: []
signature: "pub (crate) fn ruler (& mut self , r : Rect , c : [f32 ; 4])"
---

A solid rectangle on the RULER layer — band, tick and pointer mark.

The ruler is the canvas's last layer because it **measures** what is
underneath: a component passing over the number would erase the one thing
the ruler is there to say.
