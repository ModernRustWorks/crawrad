---
id: chat-history-historystore-at
kind: doc
target_file: src/chat_history.rs
line_hint: 122
author: 
created: 
title: "HistoryStore::at"
tags: []
signature: "pub fn at (dir : impl Into < PathBuf >) -> Self"
---

The store in `dir`.

# Examples

```rust
use crawrad_agent::chat_history::HistoryStore;
use std::path::Path;

let store = HistoryStore::at(Path::new("/tmp/history"));
assert_eq!(store.dir(), Path::new("/tmp/history"));
```
