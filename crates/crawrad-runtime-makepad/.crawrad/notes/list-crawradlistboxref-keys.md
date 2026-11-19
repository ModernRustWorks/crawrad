---
id: list-crawradlistboxref-keys
kind: doc
target_file: src/list.rs
line_hint: 469
author: 
created: 
title: "CrawradListBoxRef::keys"
tags: []
signature: "pub fn keys (& self , actions : & Actions) -> Option < (bool , bool) >"
---

The keys held on the click in `actions`: (Ctrl or Cmd, Shift) (#439).

# Examples

```rust,no_run
// Makepad Actions require the Makepad application runtime.
use makepad_widgets::Actions;
use crawrad_runtime_makepad::list::CrawradListBoxRef;

fn check(r: &CrawradListBoxRef, actions: &Actions) {
    let _keys = r.keys(actions);
}
```
