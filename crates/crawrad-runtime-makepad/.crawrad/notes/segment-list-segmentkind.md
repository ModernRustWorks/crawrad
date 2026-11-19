---
id: segment-list-segmentkind
kind: doc
target_file: src/segment_list.rs
line_hint: 25
author: 
created: 
title: "SegmentKind"
tags: []
signature: "pub enum SegmentKind"
---

The kind of segment displayed at a particular slot.

# Examples

```rust
use crawrad_runtime_makepad::segment_list::SegmentKind;

let kind = SegmentKind::Text;
assert_eq!(kind.as_str(), "text");
```
