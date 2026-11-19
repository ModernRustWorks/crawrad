---
id: card-apply
kind: doc
target_file: src/card.rs
line_hint: 206
author: 
created: 
title: "apply"
tags: []
signature: "pub fn apply (cx : & mut Cx , card : & WidgetRef , corner_radius : i64 , elevation : i64)"
---

Radius and elevation of a Card — the call generated projects make (#500).

# Examples

```rust,no_run
// Makepad Cx and WidgetRef require the Makepad application runtime.
use makepad_widgets::{Cx, WidgetRef};
use crawrad_runtime_makepad::card::apply;

fn example(cx: &mut Cx, card: &WidgetRef) {
    apply(cx, card, 4, 1);
}
```
