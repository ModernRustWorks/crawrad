---
id: list-crawradlistboxref-double-clicked
kind: doc
target_file: src/list.rs
line_hint: 461
author: 
created: 
title: "CrawradListBoxRef::double_clicked"
tags: []
signature: "pub fn double_clicked (& self , actions : & Actions) -> Option < usize >"
---

The row the user double-clicked in `actions`, if any (#439).

# Examples

```rust,no_run
// Makepad Actions require the Makepad application runtime.
use makepad_widgets::Actions;
use crawrad_runtime_makepad::list::CrawradListBoxRef;

fn check(r: &CrawradListBoxRef, actions: &Actions) {
    let _dbl = r.double_clicked(actions);
}
```
