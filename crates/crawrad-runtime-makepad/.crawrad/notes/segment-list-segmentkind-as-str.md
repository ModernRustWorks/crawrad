---
id: segment-list-segmentkind-as-str
kind: doc
target_file: src/segment_list.rs
line_hint: 35
author: 
created: 
title: "SegmentKind::as_str"
tags: []
signature: "pub fn as_str (& self) -> & 'static str"
---

Stable string identifier for serialization / MCP reporting.

# Examples

```rust
use crawrad_runtime_makepad::segment_list::SegmentKind;

assert_eq!(SegmentKind::Text.as_str(), "text");
assert_eq!(SegmentKind::Block.as_str(), "block");
```
