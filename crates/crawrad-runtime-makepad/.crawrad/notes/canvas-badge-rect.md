---
id: canvas-badge-rect
kind: doc
target_file: src/canvas.rs
line_hint: 648
author: 
created: 
title: "badge_rect"
tags: []
signature: "pub fn badge_rect (r : Rect , scale : f64) -> Rect"
---

The tab order badge rectangle, above the component.

The tab order badge, **above** the top-left corner of the
component — never on top of it, otherwise it hides what it numbers.

`scale` is the canvas scale (device factor times zoom). The badge is a
15x15 DESIGN-unit circle (18x13 read as an oval), like the number inside it, whose `font_scale` follows
the same scale: a badge fixed in screen pixels kept its size while its
number grew with the zoom, and at 2x the number overflowed it (#112).

# Examples

```rust
use makepad_widgets::{dvec2, Rect};
use crawrad_runtime_makepad::canvas::badge_rect;

let r = Rect { pos: dvec2(10.0, 20.0), size: dvec2(50.0, 30.0) };
let badge = badge_rect(r, 1.0);
assert_eq!(badge.pos, dvec2(10.0, 5.0));
assert_eq!(badge.size.x, badge.size.y);
```
