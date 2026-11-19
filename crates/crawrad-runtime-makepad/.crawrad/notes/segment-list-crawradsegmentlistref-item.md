---
id: segment-list-crawradsegmentlistref-item
kind: doc
target_file: src/segment_list.rs
line_hint: 159
author: 
created: 
title: "CrawradSegmentListRef::item"
tags: []
signature: "pub fn item (& self , index : usize) -> WidgetRef"
---

Access a child widget by its index.

# Examples

```rust,no_run
// Makepad widgets require the Makepad application runtime.
use crawrad_runtime_makepad::segment_list::CrawradSegmentListRef;

fn get_child(list: &CrawradSegmentListRef) {
    let _child = list.item(0);
}
```
