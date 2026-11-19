---
id: chat-history-historystore-user-default
kind: doc
target_file: src/chat_history.rs
line_hint: 127
author: 
created: 
title: "HistoryStore::user_default"
tags: []
signature: "pub fn user_default () -> Self"
---

`%APPDATA%\CrawRAD\chat-history` (`~/.crawrad/chat-history` elsewhere).

# Examples

```rust
use crawrad_agent::chat_history::HistoryStore;

let store = HistoryStore::user_default();
assert!(store.dir().ends_with("chat-history"));
```
