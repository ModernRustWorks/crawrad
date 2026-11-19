---
id: history-view-historysession-json
kind: doc
target_file: src/history_view.rs
line_hint: 375
author: 
created: 
title: "HistorySession::json"
tags: []
signature: "pub fn json (& self , buffer : & str) -> serde_json :: Value"
---

What `view.get_history` reports about the copies against `buffer`.

# Examples

```rust
use crawrad_host::history_view::HistorySession;

let session = HistorySession::default();
let val = session.json("content");
assert_eq!(val["selected"], serde_json::Value::Null);
```
