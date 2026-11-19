---
id: chat-history-historystore-save
kind: doc
target_file: src/chat_history.rs
line_hint: 148
author: 
created: 
title: "HistoryStore::save"
tags: []
signature: "pub fn save (& self , conversation : & Conversation) -> Result < () , String >"
---

Writes the conversation to storage.

Writes `conversation` (replacing its earlier copy). An empty one is not kept.

# Errors

Returns an error if the conversation ID is invalid or writing to disk fails.

# Examples

```rust,no_run
// Writes conversation files to disk.
use crawrad_agent::chat_history::{Conversation, HistoryStore};
use std::path::Path;

let store = HistoryStore::at(Path::new("/tmp/history"));
let conv = Conversation {
    id: "c1".to_string(),
    agent: "Claude".to_string(),
    agent_key: "claude".to_string(),
    acp_session: None,
    can_resume: false,
    updated: "2026-11-05 10:00".to_string(),
    messages: vec![],
};
assert!(store.save(&conv).is_ok());
```
