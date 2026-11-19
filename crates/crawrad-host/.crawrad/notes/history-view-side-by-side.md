---
id: history-view-side-by-side
kind: doc
target_file: src/history_view.rs
line_hint: 46
author: 
created: 
title: "side_by_side"
tags: []
signature: "pub fn side_by_side (copy : & str , buffer : & str) -> SideBySide"
---

Computes the side-by-side alignment and change blocks between copy and buffer.

# Examples

```rust
use crawrad_host::history_view::side_by_side;

let sbs = side_by_side("hello\n", "world\n");
assert_eq!(sbs.blocks.len(), 1);
```
