---
id: events-pointerrouter-route-at
kind: doc
target_file: src/events.rs
line_hint: 101
author: 
created: 
title: "PointerRouter::route_at"
tags: []
signature: "pub fn route_at (& mut self , raw : Raw , at : DVec2 , time : f64 , rects : & [(usize , Rect)] ,) -> Vec < (usize , Pointer , DVec2) >"
---

Determines which controls receive an event for a raw input at `at`.

The pure core: which controls get what for a `raw` event at `at`, as
`(control, kind, local position)`.

# Examples

```rust
use makepad_widgets::{dvec2, Rect};
use crawrad_runtime_makepad::events::{Pointer, PointerRouter, Raw};

let mut router = PointerRouter::default();
let rects = vec![(0, Rect { pos: dvec2(0.0, 0.0), size: dvec2(100.0, 100.0) })];
let hits = router.route_at(Raw::Down, dvec2(50.0, 50.0), 0.0, &rects);
assert_eq!(hits.len(), 1);
assert_eq!(hits[0].1, Pointer::Down);
```
