---
id: chips-crawradchipsref-drawn-chips
kind: doc
target_file: src/chips.rs
line_hint: 307
author: 
created: 
title: "CrawradChipsRef::drawn_chips"
tags: []
signature: "pub fn drawn_chips (& self , cx : & Cx) -> Vec < (String , Rect , Option < Rect >) >"
---

The chips on screen and their `×` (#604), in the widget's coordinates.

# Examples

```rust,no_run
// Makepad Cx requires the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::chips::CrawradChipsRef;

fn example(cx: &Cx, chips: &CrawradChipsRef) {
    let _drawn = chips.drawn_chips(cx);
}
```
