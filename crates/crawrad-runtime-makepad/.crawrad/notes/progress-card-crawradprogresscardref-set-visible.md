---
id: progress-card-crawradprogresscardref-set-visible
kind: doc
target_file: src/progress_card.rs
line_hint: 162
author: 
created: 
title: "CrawradProgressCardRef::set_visible"
tags: []
signature: "pub fn set_visible (& self , cx : & mut Cx , visible : bool)"
---

Sets the visibility of the whole progress card.

# Examples

```rust,no_run
// Makepad Cx requires the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::progress_card::CrawradProgressCardRef;

fn hide(cx: &mut Cx, card: &CrawradProgressCardRef) {
    card.set_visible(cx, false);
}
```
