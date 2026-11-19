---
id: scrolling-content-extent
kind: doc
target_file: src/scrolling.rs
line_hint: 79
author: 
created: 
title: "content_extent"
tags: []
signature: "pub fn content_extent (box_origin : (f32 , f32) , children : impl Iterator < Item = (f32 , f32 , f32 , f32) > ,) -> (f32 , f32)"
---

**How far the content reaches**, from the box's origin corner.

It takes the children's rectangles **in form coordinates**, which is how
both runtimes keep them (`crawrad-runtime/src/base.rs:100`,
`rect_in_form`), and returns the extent relative to the box. The
conversion lives here and not in each adapter for the same reason the rule
does: two similar conversions are two conversions that drift apart.

A child **above or to the left** of the origin extends nothing: in a classic RAD IDE a
`ScrollBox` scrolls forward, and the bar does not run back past where the
content starts. Hence the floor at zero.

# Examples

```rust
use crawrad_ir::scrolling::content_extent;

let children = vec![(10.0, 10.0, 50.0, 50.0)].into_iter();
let extent = content_extent((0.0, 0.0), children);
assert_eq!(extent, (60.0, 60.0));
```
