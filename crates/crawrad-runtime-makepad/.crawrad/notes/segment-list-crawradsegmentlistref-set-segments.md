---
id: segment-list-crawradsegmentlistref-set-segments
kind: doc
target_file: src/segment_list.rs
line_hint: 166
author: 
created: 
title: "CrawradSegmentListRef::set_segments"
tags: []
signature: "pub fn set_segments (& self , cx : & mut Cx , segments : & [Segment])"
---

Shows `segments`, in order: reuses, creates or drops children to match.

# Examples

```rust,no_run
// Makepad Cx requires the Makepad application runtime.
use makepad_widgets::Cx;
use crawrad_runtime_makepad::segment_list::CrawradSegmentListRef;

fn update(cx: &mut Cx, list: &CrawradSegmentListRef) {
    list.set_segments(cx, &[]);
}
```
