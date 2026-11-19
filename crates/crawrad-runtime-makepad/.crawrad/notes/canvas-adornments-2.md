---
id: canvas-adornments-2
kind: doc
target_file: src/canvas.rs
line_hint: 891
author: 
created: 
title: "adornments"
tags: []
signature: "fn adornments (rt : & MakepadRuntime) -> Adornments"
---

Reads from runtime everything the designer requested to adorn.

Two rejections, and both come from the runtime, not from here: only what is
on screen (`desenhavel`) enters, and the handle is from `rect_da_alca` — the same
readout that the cursor uses. When there were two readouts, the arrow appeared over things
that could not be dragged.

A free-standing function, and not a widget method, for a measurement reason: it only reads
[`MakepadRuntime`], and a runtime exists without a window. That is what places the handle
clipping rule — the part of issue #111 highlighted by the owner — under
test instead of under screenshot.
