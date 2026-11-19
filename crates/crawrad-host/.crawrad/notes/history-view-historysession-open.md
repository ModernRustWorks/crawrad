---
id: history-view-historysession-open
kind: doc
target_file: src/history_view.rs
line_hint: 314
author: 
created: 
title: "HistorySession::open"
tags: []
signature: "pub fn open (history : Option < & crate :: history :: LocalHistory > , file : std :: path :: PathBuf , now : u64 , offset : i64 ,) -> Self"
---

Opens a history session for `file` with the newest copy selected.

The copies of `file`, the newest selected; none when there is no
history (no workspace, history off).

# Examples

```rust
use std::path::PathBuf;
use crawrad_host::history_view::HistorySession;

let session = HistorySession::open(None, PathBuf::from("src/main.rs"), 0, 0);
assert_eq!(session.selected, None);
```
