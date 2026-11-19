---
id: components-non-visual-name-box
kind: doc
target_file: src/components/non_visual.rs
line_hint: 53
author: 
created: 
title: "name_box"
tags: []
signature: "pub fn name_box (badge : Rect , scale : f64) -> Rect"
---

The box where the instance name is written right below the badge.

The box where the instance NAME is written: right below the badge and
centered on it, like the RXP's `Align2::CENTER_TOP`
(`crawrad-runtime-egui/src/lib.rs:1793-1800`).

`scale` is the same one as the frame's [`crate::Transformacao`] — the gap
and the line height are DESIGN measures, and they shrink with the zoom along
with the badge. (The FONT size does not scale yet: that is a tracked debt, and it is not fixed
here.)

The width is generous on purpose: `VeryLongName1` does not fit in 28 points,
and what one wants is the name centered on the badge, not clipped by it.

# Examples

```rust
use makepad_widgets::{dvec2, Rect};
use crawrad_runtime_makepad::components::non_visual::name_box;

let badge = Rect { pos: dvec2(10.0, 10.0), size: dvec2(28.0, 28.0) };
let nb = name_box(badge, 1.0);
assert!(nb.pos.y > badge.pos.y + badge.size.y);
```
