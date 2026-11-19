---
id: progress-card-crawradprogresscardref-set-button-texts
kind: doc
target_file: src/progress_card.rs
line_hint: 141
author: 
created: 
title: "CrawradProgressCardRef::set_button_texts"
tags: []
signature: "pub fn set_button_texts (& self , cx : & mut Cx , show : & str , cancel : & str)"
---

Sets the text of the Show and Cancel action buttons.

# Examples

```rust,no_run
// Makepad Cx requires the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::progress_card::CrawradProgressCardRef;

fn update(cx: &mut Cx, card: &CrawradProgressCardRef) {
    card.set_button_texts(cx, "Show", "Cancel");
}
```
