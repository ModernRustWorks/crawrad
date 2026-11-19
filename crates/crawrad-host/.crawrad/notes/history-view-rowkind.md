---
id: history-view-rowkind
kind: doc
target_file: src/history_view.rs
line_hint: 13
author: 
created: 
title: "RowKind"
tags: []
signature: "pub enum RowKind"
---

What a side-by-side row shows.

# Examples

```rust
use crawrad_host::history_view::RowKind;

let kind = RowKind::Same;
assert_eq!(kind, RowKind::Same);
```
