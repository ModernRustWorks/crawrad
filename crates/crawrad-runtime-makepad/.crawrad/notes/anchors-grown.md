---
id: anchors-grown
kind: doc
target_file: src/anchors.rs
line_hint: 22
author: 
created: 
title: "grown"
tags: []
signature: "pub fn grown (inner : DVec2 , caption : bool , design : DVec2) -> DVec2"
---

Calculates how much the form grew since design time.

How much the form grew since design time: the window's `inner` size,
less the caption bar when it `caption` shows, against the `design` size.

# Examples

```rust
use makepad_widgets::dvec2;
use crawrad_runtime_makepad::anchors::grown;

let delta = grown(dvec2(640.0, 420.0), false, dvec2(640.0, 420.0));
assert_eq!(delta, dvec2(0.0, 0.0));
```
