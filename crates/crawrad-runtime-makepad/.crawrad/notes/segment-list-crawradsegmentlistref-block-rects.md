---
id: segment-list-crawradsegmentlistref-block-rects
kind: doc
target_file: src/segment_list.rs
line_hint: 231
author: 
created: 
title: "CrawradSegmentListRef::block_rects"
tags: []
signature: "pub fn block_rects (& self , cx : & Cx) -> Vec < ItemRect >"
---

Every item in order: its kind, its rect and, for a block, its Copy's rect.

# Examples

```rust,no_run
// Makepad Cx requires the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::segment_list::CrawradSegmentListRef;

fn check(cx: &Cx, list: &CrawradSegmentListRef) {
    let _rects = list.block_rects(cx);
}
```
