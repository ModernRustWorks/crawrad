---
id: check-list-crawradchecklistboxref-set-details
kind: doc
target_file: src/check_list.rs
line_hint: 201
author: 
created: 
title: "CrawradCheckListBoxRef::set_details"
tags: []
signature: "pub fn set_details (& self , cx : & mut Cx , details : & [String])"
---

Shows `details` at the right end of the rows, one per item.

# Examples

```rust,no_run
// Makepad Cx and widget refs require the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::check_list::CrawradCheckListBoxRef;

fn example(cx: &mut Cx, list: &CrawradCheckListBoxRef) {
    list.set_details(cx, &["Detail".to_string()]);
}
```
