---
id: segment-list-crawradsegmentlist-update-items
kind: doc
target_file: src/segment_list.rs
line_hint: 118
author: 
created: 
title: "CrawradSegmentList::update_items"
tags: []
signature: "pub fn update_items (& mut self , cx : & mut Cx , kinds : & [SegmentKind])"
---

Recycle or create child widgets matching `kinds`.

# Examples

```rust,no_run
// Makepad Cx and widgets require the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::segment_list::CrawradSegmentList;

fn update(cx: &mut Cx, list: &mut CrawradSegmentList) {
    list.update_items(cx, &[]);
}
```
