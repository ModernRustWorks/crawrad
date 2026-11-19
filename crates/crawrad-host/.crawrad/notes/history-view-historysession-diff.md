---
id: history-view-historysession-diff
kind: doc
target_file: src/history_view.rs
line_hint: 346
author: 
created: 
title: "HistorySession::diff"
tags: []
signature: "pub fn diff (& self , buffer : & str) -> SideBySide"
---

The selected copy against `buffer`.

# Examples

```rust
use crawrad_host::history_view::HistorySession;

let session = HistorySession::default();
let sbs = session.diff("code\n");
assert_eq!(sbs.rows.len(), 1);
```
