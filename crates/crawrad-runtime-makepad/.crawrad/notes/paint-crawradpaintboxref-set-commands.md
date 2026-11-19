---
id: paint-crawradpaintboxref-set-commands
kind: doc
target_file: src/paint.rs
line_hint: 251
author: 
created: 
title: "CrawradPaintBoxRef::set_commands"
tags: []
signature: "pub fn set_commands (& self , cx : & mut Cx , commands : Vec < Command >)"
---

Updates the list of paint commands, redrawing only when changed.

What `OnPaint` drew this time; the box redraws only when it changed
(the sync runs after every event).

# Examples

```rust,no_run
// Makepad Cx requires the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::paint::CrawradPaintBoxRef;

fn update(cx: &mut Cx, r: &CrawradPaintBoxRef) {
    r.set_commands(cx, Vec::new());
}
```
