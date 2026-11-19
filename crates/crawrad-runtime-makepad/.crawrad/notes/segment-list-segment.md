---
id: segment-list-segment
kind: doc
target_file: src/segment_list.rs
line_hint: 44
author: 
created: 
title: "Segment"
tags: []
signature: "pub enum Segment < 'a >"
---

One item to show, as [`CrawradSegmentListRef::set_segments`] takes it.

# Examples

```rust
use crawrad_runtime_makepad::segment_list::Segment;

let seg = Segment::Text("prose");
assert!(matches!(seg, Segment::Text(_)));
```
