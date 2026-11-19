---
id: components-drawing-componentdrawing-visual
kind: doc
target_file: src/components/drawing.rs
line_hint: 82
author: 
created: 
title: "ComponentDrawing::visual"
tags: []
signature: "fn visual (& self) -> bool"
---

Returns whether the class has painting of its own in the application.

Does the class have PAINTING of its own in the application, or is it a
non-visual?

It is the second question the catalogue answers, and it has two consumers
that must agree: [`crate::CLASSES`] (which admits no non-visual —
`crawrad-runtime/src/lib.rs:166-168`) and `crate::e_visual_no_canvas`, which
is where `crawrad_runtime::na_superficie`'s answer comes from. While it was
a `matches!` over class names in `lib.rs`, it was a hand-written list
nobody matched against the drawing — and the non-visual ended up drawn
down the wrong path.

The default is *yes*: an unknown class is visual, which is what the RXP
assumes too (`crawrad-runtime-egui/src/lib.rs:3296`).
