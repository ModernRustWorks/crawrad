---
id: sync-incremental-change
kind: doc
target_file: src/sync.rs
line_hint: 49
author: 
created: 
title: "incremental_change"
tags: []
signature: "pub fn incremental_change (old : & str , new : & str , enc : crate :: encoding :: PositionEncoding ,) -> Option < ContentChange >"
---

The single change that turns `old` into `new`; `None` when they are equal.

# Examples

```rust
use crawrad_lsp::encoding::PositionEncoding;
use crawrad_lsp::sync::incremental_change;

let change = incremental_change("foo", "bar", PositionEncoding::Utf8);
assert!(change.is_some());
```
