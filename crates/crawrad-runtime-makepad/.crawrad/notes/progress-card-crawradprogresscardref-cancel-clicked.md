---
id: progress-card-crawradprogresscardref-cancel-clicked
kind: doc
target_file: src/progress_card.rs
line_hint: 156
author: 
created: 
title: "CrawradProgressCardRef::cancel_clicked"
tags: []
signature: "pub fn cancel_clicked (& self , actions : & Actions) -> bool"
---

Whether the Cancel button was clicked.

# Examples

```rust,no_run
// Makepad Actions require the Makepad application runtime.
use makepad_widgets::Actions;
use crawrad_runtime_makepad::progress_card::CrawradProgressCardRef;

fn check(card: &CrawradProgressCardRef, actions: &Actions) {
    let _clicked = card.cancel_clicked(actions);
}
```
