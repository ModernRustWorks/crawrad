---
id: card-crawformcardref-set-header
kind: doc
target_file: src/card.rs
line_hint: 162
author: 
created: 
title: "CrawradFormCardRef::set_header"
tags: []
signature: "pub fn set_header (& self , cx : & mut Cx , title : & str , subtitle : & str , show : bool)"
---

The header: title, subtitle (hidden when empty), and whether it shows.

# Examples

```rust,no_run
// Makepad Cx and WidgetRef require the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::card::CrawradFormCardRef;

fn example(cx: &mut Cx, card: &CrawradFormCardRef) {
    card.set_header(cx, "Title", "Subtitle", true);
}
```
