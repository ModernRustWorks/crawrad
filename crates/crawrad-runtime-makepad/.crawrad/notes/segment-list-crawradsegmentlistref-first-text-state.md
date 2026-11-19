---
id: segment-list-crawradsegmentlistref-first-text-state
kind: doc
target_file: src/segment_list.rs
line_hint: 216
author: 
created: 
title: "CrawradSegmentListRef::first_text_state"
tags: []
signature: "pub fn first_text_state (& self , cx : & Cx) -> ([f64 ; 4] , String)"
---

The first text item's rect `[x, y, w, h]` and its selected text.

# Examples

```rust,no_run
// Makepad Cx requires the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::segment_list::CrawradSegmentListRef;

fn check(cx: &Cx, list: &CrawradSegmentListRef) {
    let (_rect, _sel) = list.first_text_state(cx);
}
```
