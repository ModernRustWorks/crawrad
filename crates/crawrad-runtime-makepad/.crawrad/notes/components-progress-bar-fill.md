---
id: components-progress-bar-fill
kind: doc
target_file: src/components/progress_bar.rs
line_hint: 13
author: 
created: 
title: "fill"
tags: []
signature: "pub fn fill (r : Rect , props : & BTreeMap < String , Value >) -> Rect"
---

Calculates the filled rectangle of a progress bar.

The filled part of the bar — the SAME fraction the generated app draws
([`crate::progress::fraction`], #582): `Position` between `Min` and `Max`,
along the `Orientation`. A vertical bar fills from the bottom, as the app does.

# Examples

```rust
use std::collections::BTreeMap;
use makepad_widgets::{dvec2, Rect};
use crawrad_runtime_makepad::components::progress_bar::fill;

let r = Rect { pos: dvec2(0.0, 0.0), size: dvec2(100.0, 20.0) };
let props = BTreeMap::new();
let filled = fill(r, &props);
assert_eq!(filled.size.x, 0.0);
```
