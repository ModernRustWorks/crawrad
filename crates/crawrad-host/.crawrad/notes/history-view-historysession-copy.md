---
id: history-view-historysession-copy
kind: doc
target_file: src/history_view.rs
line_hint: 337
author: 
created: 
title: "HistorySession::copy"
tags: []
signature: "pub fn copy (& self) -> & str"
---

The selected copy's text ("" when there is none).

# Examples

```rust
use crawrad_host::history_view::HistorySession;

let session = HistorySession::default();
assert_eq!(session.copy(), "");
```
