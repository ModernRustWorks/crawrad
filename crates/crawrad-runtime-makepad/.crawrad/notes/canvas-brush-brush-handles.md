---
id: canvas-brush-brush-handles
kind: doc
target_file: src/canvas/brush.rs
line_hint: 1006
author: 
created: 
title: "Brush::handles"
tags: []
signature: "pub (crate) fn handles (& mut self , points : & [(f32 , f32)] , side : f64)"
---

The selection handles, centred on the points `crawrad-runtime` handed over.

The geometry is NOT computed here: it arrives ready from
`crawrad_runtime::alcas` (ADR 0019). What this method does is centre the
little square on the point.
