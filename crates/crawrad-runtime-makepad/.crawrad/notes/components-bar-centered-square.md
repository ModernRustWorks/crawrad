---
id: components-bar-centered-square
kind: doc
target_file: src/components/bar.rs
line_hint: 198
author: 
created: 
title: "centered_square"
tags: []
signature: "pub fn centered_square (r : Rect) -> Rect"
---

The largest square centred in `r`.

The arrow draws itself in a square, and the button of a thick bar is not
square (`16 x height`): without this the triangle came out stretched.

# Examples

```rust
use makepad_widgets::{dvec2, Rect};
use crawrad_runtime_makepad::components::bar::centered_square;

let r = Rect { pos: dvec2(0.0, 0.0), size: dvec2(20.0, 10.0) };
let sq = centered_square(r);
assert_eq!(sq.size, dvec2(10.0, 10.0));
assert_eq!(sq.pos, dvec2(5.0, 0.0));
```
