---
id: components-non-visual-icon-box
kind: doc
target_file: src/components/non_visual.rs
line_hint: 48
author: 
created: 
title: "icon_box"
tags: []
signature: "pub fn icon_box (badge : Rect , scale : f64) -> Rect"
---

The square where the class icon is drawn, centred inside the badge.

Sixteen design units — the icon grid — scaled with the zoom, never larger
than the badge, and snapped to whole pixels so the rasterized icon stays sharp.

# Examples

```rust
use makepad_widgets::{dvec2, Rect};
use crawrad_runtime_makepad::components::non_visual::icon_box;

let badge = Rect { pos: dvec2(10.0, 10.0), size: dvec2(28.0, 28.0) };
let icon = icon_box(badge, 1.0);
assert_eq!(icon.size, dvec2(16.0, 16.0));
assert_eq!(icon.pos, dvec2(16.0, 16.0));
```
