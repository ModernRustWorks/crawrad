---
id: components-track-bar-look-parts
kind: doc
target_file: src/components/track_bar.rs
line_hint: 49
author: 
created: 
title: "look_parts"
tags: []
signature: "pub fn look_parts (r : Rect , props : & BTreeMap < String , Value >) -> Option < (Rect , Rect , f64) >"
---

Calculates look-specific parts for horizontal TrackBar styling.

`makepad.look` (#491) of a horizontal bar: the box the app's `Slider` or
`SliderRound` draws, and its filled part. `None` for `Minimal` and for a
vertical bar, which keep the line of [`parts`]. Measured on 31/10 on a
300 x 32 slider: `Slider` is an 11 px inset box along the bottom, filled by
a 2 px line through its middle; `SliderRound` is a pill the whole height,
filled by a lighter pill up to the value.

# Examples

```rust
use std::collections::BTreeMap;
use makepad_widgets::{dvec2, Rect};
use crawrad_runtime_makepad::components::track_bar::look_parts;

let r = Rect { pos: dvec2(0.0, 0.0), size: dvec2(100.0, 20.0) };
let props = BTreeMap::new();
assert!(look_parts(r, &props).is_none());
```
