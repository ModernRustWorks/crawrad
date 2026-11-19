---
id: check-list-crawradchecklistboxref-set-items
kind: doc
target_file: src/check_list.rs
line_hint: 190
author: 
created: 
title: "CrawradCheckListBoxRef::set_items"
tags: []
signature: "pub fn set_items (& self , cx : & mut Cx , items : & [String])"
---

Shows `items` (only redraws when they changed: the sync runs after every event).

Shows `items` (only redraws when they changed: the sync runs after
every event).

# Examples

```rust,no_run
// Makepad Cx and widget refs require the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::check_list::CrawradCheckListBoxRef;

fn example(cx: &mut Cx, list: &CrawradCheckListBoxRef) {
    list.set_items(cx, &["One".to_string()]);
}
```
