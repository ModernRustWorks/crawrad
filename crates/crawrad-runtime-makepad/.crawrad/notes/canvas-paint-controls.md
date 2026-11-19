---
id: canvas-paint-controls
kind: doc
target_file: src/canvas.rs
line_hint: 1024
author: 
created: 
title: "paint_controls"
tags: []
signature: "pub (crate) fn paint_controls (p : & mut Brush , ctls : & std :: collections :: BTreeMap < NodeId , MakepadCtl > , ordem : & [NodeId] , modo : crawrad_runtime :: Mode , forma : RectIr , form_font : Option < & Font > ,)"
---

Paints every control of the form in drawing order.

Paints every control of the form, in drawing order, with the clips and the
chrome the canvas gives them — and then the `MainMenu` bar.

It is the frame's painting of the DOCUMENT, cut out of `draw_walk` so that
the shape probe (#114) paints through the very same path the screen does:
a [`Brush`] on a [`Probe`] runs this, and what it writes down is what the
GPU would have been given.
