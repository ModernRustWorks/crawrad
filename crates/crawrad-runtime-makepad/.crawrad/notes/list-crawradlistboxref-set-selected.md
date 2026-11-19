---
id: list-crawradlistboxref-set-selected
kind: doc
target_file: src/list.rs
line_hint: 420
author: 
created: 
title: "CrawradListBoxRef::set_selected"
tags: []
signature: "pub fn set_selected (& self , cx : & mut Cx , selected : Option < usize >)"
---

Marks row `selected` (`None`: no row).

# Examples

```rust,no_run
// Makepad Cx requires the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::list::CrawradListBoxRef;

fn select_first(cx: &mut Cx, r: &CrawradListBoxRef) {
    r.set_selected(cx, Some(0));
}
```
