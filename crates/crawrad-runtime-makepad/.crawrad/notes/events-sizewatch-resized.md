---
id: events-sizewatch-resized
kind: doc
target_file: src/events.rs
line_hint: 210
author: 
created: 
title: "SizeWatch::resized"
tags: []
signature: "pub fn resized (& mut self , sizes : & [(usize , DVec2)]) -> Vec < usize >"
---

Returns control indices whose dimensions changed since the previous check.

The controls of `sizes` (`(control, size)`) whose size changed since
they were last seen. The first size a control shows is remembered, not
announced; an empty one is a control not drawn yet.

# Examples

```rust
use makepad_widgets::dvec2;
use crawrad_runtime_makepad::events::SizeWatch;

let mut watch = SizeWatch::default();
assert!(watch.resized(&[(0, dvec2(100.0, 50.0))]).is_empty());
assert_eq!(watch.resized(&[(0, dvec2(120.0, 50.0))]), vec![0]);
```
