---
id: components-drawing-componentdrawing-draws-over-children
kind: doc
target_file: src/components/drawing.rs
line_hint: 105
author: 
created: 
title: "ComponentDrawing::draws_over_children"
tags: []
signature: "fn draws_over_children (& self) -> bool"
---

Returns true if this control paints on top of its children.

Does this control paint anything **on top of its own children**?

Answering *yes* costs one more step in the frame ([`crate::canvas::Step`])
and buys the only thing that puts a scroll bar back where the eye
expects it. Answering *no* — the default — costs nothing.
