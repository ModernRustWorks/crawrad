---
id: history-view-historysession-step
kind: doc
target_file: src/history_view.rs
line_hint: 359
author: 
created: 
title: "HistorySession::step"
tags: []
signature: "pub fn step (& mut self , buffer : & str , forward : bool) -> Option < usize >"
---

Moves the focus to the next or previous change block.

Moves the focus to the next (or previous) block; the row to bring
into view.

# Examples

```rust
use crawrad_host::history_view::HistorySession;

let mut session = HistorySession::default();
// With no copy chosen, the whole buffer is one block, at row 0.
assert_eq!(session.step("test", true), Some(0));
```
