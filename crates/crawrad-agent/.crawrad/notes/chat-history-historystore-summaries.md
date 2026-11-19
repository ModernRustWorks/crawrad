---
id: chat-history-historystore-summaries
kind: doc
target_file: src/chat_history.rs
line_hint: 186
author: 
created: 
title: "HistoryStore::summaries"
tags: []
signature: "pub fn summaries (& self , today : & str , query : & str) -> Vec < Value >"
---

Returns the history list of conversation summaries filtered by query.

The history list: each row's summary, newest first, those whose title or
agent contains `query` (any case; empty keeps all).

# Examples

```rust
use crawrad_agent::chat_history::HistoryStore;
use std::path::Path;

let store = HistoryStore::at(Path::new("/tmp/history_empty"));
let list = store.summaries("2026-11-05", "");
assert!(list.is_empty());
```
