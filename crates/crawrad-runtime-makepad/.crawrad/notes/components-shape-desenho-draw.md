---
id: components-shape-desenho-draw
kind: doc
target_file: src/components/shape.rs
line_hint: 20
author: 
created: 
title: "Desenho::draw"
tags: []
signature: "fn draw (& self , p : & mut Brush , c : & MakepadCtl , r : Rect , _scene : & Scene)"
---

Draws the shape with the configured brush and pen.

#460: the kind, the brush and the pen the app draws, so the designer
shows the same shape. The ellipse is a box rounded by half its short
side here: the canvas has no ellipse primitive, and the kind still
reads at a glance.
