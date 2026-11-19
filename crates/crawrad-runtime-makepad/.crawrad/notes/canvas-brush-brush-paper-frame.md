---
id: canvas-brush-brush-paper-frame
kind: doc
target_file: src/canvas/brush.rs
line_hint: 956
author: 
created: 
title: "Brush::paper_frame"
tags: []
signature: "pub (crate) fn paper_frame (& mut self , r : Rect , c : [f32 ; 4] , thickness : f64)"
---

The form PAPER's frame, on the background layer.

It cannot come out of [`Brush::stroke`]: the stroke is the layer a
component uses for the accelerator underline, the `GroupBox` frame and
the tree marker, and in Makepad **a shader's draw call is born on its
first instance** (`draw_list.rs:334-343`). Opening the stroke layer here,
before any component, put all three of those BELOW the components —
measured: the `&OK` underline vanished and the tree's `[-]` became an
empty box.
