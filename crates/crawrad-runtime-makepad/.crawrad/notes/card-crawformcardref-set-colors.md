---
id: card-crawformcardref-set-colors
kind: doc
target_file: src/card.rs
line_hint: 180
author: 
created: 
title: "CrawradFormCardRef::set_colors"
tags: []
signature: "pub fn set_colors (& self , cx : & mut Cx , fill : Vec4 , border : Vec4)"
---

Sets the surface fill and border colours.

The surface's fill and border colours — the selection, for a host that
lists cards (the IDE's New Project).

# Examples

```rust,no_run
// Makepad Cx and WidgetRef require the Makepad application runtime.
use makepad_widgets::{vec4, Cx};
use crawrad_runtime_makepad::card::CrawradFormCardRef;

fn example(cx: &mut Cx, card: &CrawradFormCardRef) {
    card.set_colors(cx, vec4(1.0, 1.0, 1.0, 1.0), vec4(0.0, 0.0, 0.0, 1.0));
}
```
