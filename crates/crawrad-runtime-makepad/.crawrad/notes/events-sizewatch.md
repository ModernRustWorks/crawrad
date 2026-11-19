---
id: events-sizewatch
kind: doc
target_file: src/events.rs
line_hint: 203
author: 
created: 
title: "SizeWatch"
tags: []
signature: "pub struct SizeWatch"
---

Tracks control dimensions to trigger resize events.

**Classic RAD's `OnResize` for the form's controls**: the size each one
was last drawn with, and which changed.

# Examples

```rust
use makepad_widgets::dvec2;
use crawrad_runtime_makepad::events::SizeWatch;

let mut watch = SizeWatch::default();
let changed = watch.resized(&[(0, dvec2(100.0, 50.0))]);
assert!(changed.is_empty());
```
