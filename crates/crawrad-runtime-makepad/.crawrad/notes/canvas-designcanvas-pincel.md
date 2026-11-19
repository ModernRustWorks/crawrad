---
id: canvas-designcanvas-pincel
kind: doc
target_file: src/canvas.rs
line_hint: 859
author: 
created: 
title: "DesignCanvas::pincel"
tags: []
signature: "fn pincel < 'a , 'b , 'c > (& 'a mut self , cx : & 'a mut Cx2d < 'b , 'c > , t : Transformacao ,) -> Brush < 'a , 'b , 'c >"
---

Borrows the widget's shaders for the frame's brush.

From here on nothing else reads the model: the borrows last for the frame, which is
why `draw_adornments` receives an [`Adornments`] already read.
