---
id: props-props-makepad-slider-look
kind: doc
target_file: src/props.rs
line_hint: 1471
author: 
created: 
title: "Props::makepad_slider_look"
tags: []
signature: "pub fn makepad_slider_look (default : & 'static str) -> PropDesc"
---

Selects Makepad's slider rendering style (`makepad.look`, #491).

`makepad.look` (#491): which of Makepad's three sliders draws the
TrackBar. Makepad-only — an egui project never sees it.

# Examples

```rust
use crawrad_registry::Props;

let p = Props::makepad_slider_look("Minimal");
assert_eq!(p.key, "makepad.look");
```
