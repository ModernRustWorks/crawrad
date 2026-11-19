---
id: list-crawradlistboxref-set-selection
kind: doc
target_file: src/list.rs
line_hint: 430
author: 
created: 
title: "CrawradListBoxRef::set_selection"
tags: []
signature: "pub fn set_selection (& self , cx : & mut Cx , rows : & [usize])"
---

Marks multiple rows for multi-select lists.

Marks every row in `rows` (#439, `MultiSelect`); the row in focus is
still the one [`Self::set_selected`] gave.

# Examples

```rust,no_run
// Makepad Cx requires the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::list::CrawradListBoxRef;

fn select_multi(cx: &mut Cx, r: &CrawradListBoxRef) {
    r.set_selection(cx, &[0, 2]);
}
```
