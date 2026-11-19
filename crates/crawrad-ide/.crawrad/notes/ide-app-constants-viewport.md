---
id: ide-app-constants-viewport
kind: doc
target_file: src/ide_app/constants.rs
line_hint: 100
author: 
created: 
title: "VIEWPORT"
tags: []
signature: "pub (super) const VIEWPORT : (i32 , i32) = (1200 , 800)"
---

The viewport size that the shell declares to the runtime, in logical pixels.

It does not position anything: what knows where the form landed on screen is
`DesignCanvas`, which builds the [`crawrad_runtime_makepad::Transformacao`] with the
origin of the frame's turtle (ADR 0019 — a single transformation). This number
exists because the `Runtime` contract requires one, and it is the same as the
`design_canvas` example.
