---
id: segment-list-crawradsegmentlistref-set-items
kind: doc
target_file: src/segment_list.rs
line_hint: 152
author: 
created: 
title: "CrawradSegmentListRef::set_items"
tags: []
signature: "pub fn set_items (& self , cx : & mut Cx , kinds : & [SegmentKind])"
---

Reconfigures the child items of the list to match `kinds`.

# Examples

```rust,no_run
// Makepad Cx requires the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::segment_list::CrawradSegmentListRef;

fn update(cx: &mut Cx, list: &CrawradSegmentListRef) {
    list.set_items(cx, &[]);
}
```
