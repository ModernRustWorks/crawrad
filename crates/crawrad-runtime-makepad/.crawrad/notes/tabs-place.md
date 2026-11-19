---
id: tabs-place
kind: doc
target_file: src/tabs.rs
line_hint: 44
author: 
created: 
title: "place"
tags: []
signature: "pub fn place (cx : & mut Cx , strip : & WidgetRef , pages : & WidgetRef , height : f64 , bottom : bool)"
---

Positions the tab strip and page container at top or bottom.

Places the `strip` and the `pages` of a page control `height` points
tall: the strip on top, or under the pages when `bottom`.

# Examples

```rust,no_run
// Makepad Cx and WidgetRef require the Makepad application runtime.
use makepad_widgets::{Cx, WidgetRef};
use crawrad_runtime_makepad::tabs::place;

fn position(cx: &mut Cx, strip: &WidgetRef, pages: &WidgetRef) {
    place(cx, strip, pages, 300.0, false);
}
```
