---
id: check-list-crawradchecklistboxref-drawn-rows
kind: doc
target_file: src/check_list.rs
line_hint: 230
author: 
created: 
title: "CrawradCheckListBoxRef::drawn_rows"
tags: []
signature: "pub fn drawn_rows (& self , cx : & Cx) -> Vec < (String , Rect) >"
---

Returns the rows drawn on the last frame in list coordinates.

The rows drawn on the last frame, text and rectangle, in the list's
own coordinates (see [`crate::list::CrawradListBoxRef::drawn_rows`]).

# Examples

```rust,no_run
// Makepad Cx requires the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::check_list::CrawradCheckListBoxRef;

fn example(cx: &Cx, list: &CrawradCheckListBoxRef) {
    let _rows = list.drawn_rows(cx);
}
```
