---
id: chat-history-historystore-list
kind: doc
target_file: src/chat_history.rs
line_hint: 172
author: 
created: 
title: "HistoryStore::list"
tags: []
signature: "pub fn list (& self) -> Vec < Conversation >"
---

Every stored conversation, newest first (an unreadable file is skipped).

# Examples

```rust
use crawrad_agent::chat_history::HistoryStore;
use std::path::Path;

let store = HistoryStore::at(Path::new("/tmp/history_empty"));
assert_eq!(store.list(), vec![]);
```
