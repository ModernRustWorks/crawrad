---
id: canvas-brush-lines-that-fit
kind: doc
target_file: src/canvas/brush.rs
line_hint: 85
author: 
created: 
title: "lines_that_fit"
tags: []
signature: "pub fn lines_that_fit (h : f32 , line_height : f32) -> usize"
---

How many lines of `line_height` fit inside a box of height `h`.

The line count comes from the BOX, not from a constant. A fixed `take(12)`
was wrong on both sides — and the excess spilled over the neighbour, which is
the defect the RXP recorded in `crawrad-runtime-egui/src/lib.rs:2595-2600`.

# Examples

```rust
use crawrad_runtime_makepad::canvas::lines_that_fit;

assert_eq!(lines_that_fit(36.0, 18.0), 2);
assert_eq!(lines_that_fit(10.0, 18.0), 0);
```
