---
id: history-view-sidebyside
kind: doc
target_file: src/history_view.rs
line_hint: 37
author: 
created: 
title: "SideBySide"
tags: []
signature: "pub struct SideBySide"
---

The two texts aligned row by row, and the change blocks.

# Examples

```rust
use crawrad_host::history_view::SideBySide;

let sbs = SideBySide { rows: vec![], blocks: vec![] };
assert!(sbs.rows.is_empty());
```
