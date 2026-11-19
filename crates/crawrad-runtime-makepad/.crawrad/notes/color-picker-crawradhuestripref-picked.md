---
id: color-picker-crawradhuestripref-picked
kind: doc
target_file: src/color_picker.rs
line_hint: 329
author: 
created: 
title: "CrawradHueStripRef::picked"
tags: []
signature: "pub fn picked (& self , actions : & Actions) -> Option < f32 >"
---

Extracts the picked hue fraction from the actions list, if any.

# Examples

```rust,no_run
// Actions require the Makepad event loop.
use makepad_widgets::Actions;
use crawrad_runtime_makepad::color_picker::CrawradHueStripRef;

fn example(actions: &Actions, strip: &CrawradHueStripRef) {
    let _ = strip.picked(actions);
}
```
