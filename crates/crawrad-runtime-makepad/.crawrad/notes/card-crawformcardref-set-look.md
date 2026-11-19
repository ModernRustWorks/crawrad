---
id: card-crawformcardref-set-look
kind: doc
target_file: src/card.rs
line_hint: 143
author: 
created: 
title: "CrawradFormCardRef::set_look"
tags: []
signature: "pub fn set_look (& self , cx : & mut Cx , corner_radius : i64 , elevation : i64)"
---

Radius and elevation (the `CornerRadius` and `Elevation` properties).

# Examples

```rust,no_run
// Makepad Cx and WidgetRef require the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::card::CrawradFormCardRef;

fn example(cx: &mut Cx, card: &CrawradFormCardRef) {
    card.set_look(cx, 4, 1);
}
```
