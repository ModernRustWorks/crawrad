---
id: components-track-bar-parts
kind: doc
target_file: src/components/track_bar.rs
line_hint: 13
author: 
created: 
title: "parts"
tags: []
signature: "pub fn parts (r : Rect , props : & BTreeMap < String , Value >) -> (Rect , Rect , i64)"
---

Calculates the track, filled part and value of a TrackBar.

The track, its filled part and the value (#582 family): drawn as the app's
`SliderMinimal` shows it — measured on 16/10, a thin line along the bottom
filled up to `Position` (between `Min` and `Max`), and the value on the right
above it — and not a thumb at a fixed 40 %. Vertical: the line runs up the
middle and fills from the bottom.

# Examples

```rust
use std::collections::BTreeMap;
use makepad_widgets::{dvec2, Rect};
use crawrad_runtime_makepad::components::track_bar::parts;

let r = Rect { pos: dvec2(0.0, 0.0), size: dvec2(100.0, 20.0) };
let props = BTreeMap::new();
let (track, fill, pos) = parts(r, &props);
assert_eq!(pos, 0);
```
