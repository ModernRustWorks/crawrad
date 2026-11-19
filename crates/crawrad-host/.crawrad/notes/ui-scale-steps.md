---
id: ui-scale-steps
kind: doc
target_file: src/ui_scale.rs
line_hint: 5
author: 
created: 
title: "STEPS"
tags: []
signature: "pub const STEPS : & [f64] = & [0.5 , 0.6 , 0.7 , 0.8 , 0.9 , 1.0 , 1.1 , 1.2 , 1.3 , 1.4 , 1.5 , 1.6 , 1.7 , 1.8 , 1.9 , 2.0 ,]"
---

The steps the interface zoom walks, as multiples of the desktop's own DPI.

A list and not a multiplication, for the reason the canvas zoom already
gives: repeated multiplication drifts, and the user has to land back on
`100%` exactly. These are the steps of the browser and of a modern editor, which is
the hand this key belongs to.

# Examples

```rust
use crawrad_host::ui_scale::STEPS;

assert!(STEPS.contains(&1.0));
```
