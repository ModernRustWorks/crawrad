---
id: canvas-brush-brush-open-layers
kind: doc
target_file: src/canvas/brush.rs
line_hint: 435
author: 
created: 
title: "Brush::open_layers"
tags: []
signature: "pub (crate) fn open_layers (& mut self , at : DVec2)"
---

Opens draw calls for every layer in `LAYERS` order.

Opens the draw calls of EVERY layer, in [`LAYERS`] order, before any of
the frame's painting.

Each layer is opened with an instance that **paints nothing**: a
zero-sided quad (the rasterizer produces no fragment) and text with zero
alpha. What is left over is the draw call, already in the right slot of
the queue.

The `match` is exhaustive on purpose: adding a primitive to
[`Primitives`] without giving it a layer **does not compile**.
