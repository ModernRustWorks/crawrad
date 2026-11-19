---
id: history-view-historysession-restored
kind: doc
target_file: src/history_view.rs
line_hint: 370
author: 
created: 
title: "HistorySession::restored"
tags: []
signature: "pub fn restored (& self , buffer : & str) -> Option < String >"
---

`buffer` with the block in focus (the first when none) as the copy has it.

# Examples

```rust
use crawrad_host::history_view::HistorySession;

let session = HistorySession::default();
assert_eq!(session.restored("code\n"), Some("".into()));
```
