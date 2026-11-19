---
id: events-pointerrouter-route
kind: doc
target_file: src/events.rs
line_hint: 176
author: 
created: 
title: "PointerRouter::route"
tags: []
signature: "pub fn route (& mut self , event : & Event , rects : & [(usize , Rect)]) -> Vec < PointerHit >"
---

Computes the pointer events for controls in `rects` from a Makepad event.

The pointer events `event` means for the controls in `rects`
(`(index, rect)`, in drawing order); empty for any other event.

# Examples

```rust
use makepad_widgets::{Event, Rect};
use crawrad_runtime_makepad::events::PointerRouter;

let mut router = PointerRouter::default();
let rects: Vec<(usize, Rect)> = Vec::new();
let hits = router.route(&Event::Pause, &rects);
assert!(hits.is_empty());
```
