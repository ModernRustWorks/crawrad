---
id: segment-list-itemrect
kind: doc
target_file: src/segment_list.rs
line_hint: 246
author: 
created: 
title: "ItemRect"
tags: []
signature: "pub type ItemRect = (& 'static str , [f64 ; 4] , Option < [f64 ; 4] >)"
---

An item's kind, its rect `[x, y, w, h]` and, for a block, its Copy's rect.

# Examples

```rust
use crawrad_runtime_makepad::segment_list::ItemRect;

let rect: ItemRect = ("text", [0.0, 0.0, 100.0, 20.0], None);
assert_eq!(rect.0, "text");
```
