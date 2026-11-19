---
id: color-picker-crawradcolorsquareref-picked
kind: doc
target_file: src/color_picker.rs
line_hint: 239
author: 
created: 
title: "CrawradColorSquareRef::picked"
tags: []
signature: "pub fn picked (& self , actions : & Actions) -> Option < (f32 , f32) >"
---

Extracts the picked `(s, v)` fractions from the actions list, if any.

# Examples

```rust,no_run
// Actions require the Makepad event loop.
use makepad_widgets::Actions;
use crawrad_runtime_makepad::color_picker::CrawradColorSquareRef;

fn example(square: &CrawradColorSquareRef, actions: &Actions) {
    let _picked = square.picked(actions);
}
```
