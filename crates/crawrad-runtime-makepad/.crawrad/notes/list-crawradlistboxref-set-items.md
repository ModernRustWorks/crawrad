---
id: list-crawradlistboxref-set-items
kind: doc
target_file: src/list.rs
line_hint: 388
author: 
created: 
title: "CrawradListBoxRef::set_items"
tags: []
signature: "pub fn set_items (& self , cx : & mut Cx , items : & [String])"
---

Updates the displayed list items, redrawing only on change.

Shows `items` (only redraws when they changed: the sync runs after
every event).

# Examples

```rust,no_run
// Makepad Cx requires the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::list::CrawradListBoxRef;

fn update(cx: &mut Cx, r: &CrawradListBoxRef) {
    r.set_items(cx, &["Item 1".into()]);
}
```
