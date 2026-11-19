---
id: progress-card-crawradprogresscardref-show-clicked
kind: doc
target_file: src/progress_card.rs
line_hint: 150
author: 
created: 
title: "CrawradProgressCardRef::show_clicked"
tags: []
signature: "pub fn show_clicked (& self , actions : & Actions) -> bool"
---

Whether the Show output button was clicked.

# Examples

```rust,no_run
// Makepad Actions require the Makepad application runtime.
use makepad_widgets::Actions;
use crawrad_runtime_makepad::progress_card::CrawradProgressCardRef;

fn check(card: &CrawradProgressCardRef, actions: &Actions) {
    let _clicked = card.show_clicked(actions);
}
```
