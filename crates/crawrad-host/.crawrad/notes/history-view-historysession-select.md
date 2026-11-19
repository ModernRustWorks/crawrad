---
id: history-view-historysession-select
kind: doc
target_file: src/history_view.rs
line_hint: 351
author: 
created: 
title: "HistorySession::select"
tags: []
signature: "pub fn select (& mut self , row : usize)"
---

Compares row `row` from now on; out of range changes nothing.

# Examples

```rust
use crawrad_host::history_view::HistorySession;

let mut session = HistorySession::default();
session.select(0);
assert_eq!(session.selected, None);
```
