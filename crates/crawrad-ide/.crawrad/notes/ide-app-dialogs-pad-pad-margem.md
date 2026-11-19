---
id: ide-app-dialogs-pad-pad-margem
kind: doc
target_file: src/ide_app/dialogs/pad.rs
line_hint: 10
author: 
created: 
title: "PAD_MARGEM"
tags: []
signature: "pub (super) const PAD_MARGEM : f64 = 8.0"
---

The clearance between the edge of the Inspector panel and the window it opens.

In Rust and not in the DSL because what adds it is `abs_pos`, which is written at
runtime from the panel's rectangle.
