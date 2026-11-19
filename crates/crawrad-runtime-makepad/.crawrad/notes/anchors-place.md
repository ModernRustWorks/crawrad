---
id: anchors-place
kind: doc
target_file: src/anchors.rs
line_hint: 43
author: 
created: 
title: "place"
tags: []
signature: "pub fn place (cx : & mut Cx , widget : & WidgetRef , design : (f64 , f64 , f64 , f64) , a : Anchors , grown : DVec2 ,)"
---

Moves `widget`, whose design rect is `design`, where its anchors take it.

# Examples

```rust,no_run
// Makepad Cx and WidgetRef require the Makepad application runtime.
use makepad_widgets::*;
use crawrad_runtime_makepad::anchors::{place, Anchors};

fn example(cx: &mut Cx, widget: &WidgetRef) {
    place(cx, widget, (10.0, 10.0, 100.0, 30.0), Anchors::default(), dvec2(0.0, 0.0));
}
```
