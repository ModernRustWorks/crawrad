---
id: chat-history-historystore-load
kind: doc
target_file: src/chat_history.rs
line_hint: 166
author: 
created: 
title: "HistoryStore::load"
tags: []
signature: "pub fn load (& self , id : & str) -> Option < Conversation >"
---

The conversation `id`, if it is stored and readable.

# Examples

```rust
use crawrad_agent::chat_history::HistoryStore;
use std::path::Path;

let store = HistoryStore::at(Path::new("/tmp/history_empty"));
assert!(store.load("c1").is_none());
```
