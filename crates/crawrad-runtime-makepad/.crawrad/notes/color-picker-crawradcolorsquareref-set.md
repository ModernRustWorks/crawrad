---
id: color-picker-crawradcolorsquareref-set
kind: doc
target_file: src/color_picker.rs
line_hint: 229
author: 
created: 
title: "CrawradColorSquareRef::set"
tags: []
signature: "pub fn set (& self , cx : & mut Cx , hue : f32 , s : f32 , v : f32)"
---

Sets the hue in degrees, saturation, and value, then requests a redraw.

# Examples

```rust,no_run
// Makepad Cx and widget refs require the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::color_picker::CrawradColorSquareRef;

fn example(cx: &mut Cx, square: &CrawradColorSquareRef) {
    square.set(cx, 180.0, 0.5, 0.8);
}
```
