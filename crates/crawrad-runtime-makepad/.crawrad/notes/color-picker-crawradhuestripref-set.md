---
id: color-picker-crawradhuestripref-set
kind: doc
target_file: src/color_picker.rs
line_hint: 321
author: 
created: 
title: "CrawradHueStripRef::set"
tags: []
signature: "pub fn set (& self , cx : & mut Cx , fraction : f32)"
---

Sets the hue fraction in `0.0..=1.0`, then requests a redraw.

# Examples

```rust,no_run
// Makepad Cx requires the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::color_picker::CrawradHueStripRef;

fn example(cx: &mut Cx, strip: &CrawradHueStripRef) {
    strip.set(cx, 0.5);
}
```
