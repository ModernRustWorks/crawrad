---
id: progress-card-crawradprogresscardref-set
kind: doc
target_file: src/progress_card.rs
line_hint: 109
author: 
created: 
title: "CrawradProgressCardRef::set"
tags: []
signature: "pub fn set (& self , cx : & mut Cx , icon : & str , headline : & str , detail : & str , clock : & str , fraction : Option < f64 > , running : bool ,)"
---

Updates all visual fields of the progress card.

# Examples

```rust,no_run
// Makepad Cx requires the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::progress_card::CrawradProgressCardRef;

fn update(cx: &mut Cx, card: &CrawradProgressCardRef) {
    card.set(cx, "i", "Building", "Compiling...", "00:01", Some(0.5), true);
}
```
