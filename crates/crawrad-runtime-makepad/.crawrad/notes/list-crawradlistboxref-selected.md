---
id: list-crawradlistboxref-selected
kind: doc
target_file: src/list.rs
line_hint: 453
author: 
created: 
title: "CrawradListBoxRef::selected"
tags: []
signature: "pub fn selected (& self , actions : & Actions) -> Option < usize >"
---

The row the user clicked in `actions`, if any.

# Examples

```rust,no_run
// Makepad Actions require the Makepad application runtime.
use makepad_widgets::Actions;
use crawrad_runtime_makepad::list::CrawradListBoxRef;

fn check(r: &CrawradListBoxRef, actions: &Actions) {
    let _sel = r.selected(actions);
}
```
