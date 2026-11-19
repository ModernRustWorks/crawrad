---
id: chat-history-historystore
kind: doc
target_file: src/chat_history.rs
line_hint: 115
author: 
created: 
title: "HistoryStore"
tags: []
signature: "pub struct HistoryStore"
---

The folder of conversation files.

# Examples

```rust
use crawrad_agent::chat_history::HistoryStore;
use std::path::Path;

let store = HistoryStore::at(Path::new("/tmp/chats"));
assert_eq!(store.dir(), Path::new("/tmp/chats"));
```
