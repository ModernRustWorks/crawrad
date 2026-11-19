---
id: chat-history-conversation-new-id
kind: doc
target_file: src/chat_history.rs
line_hint: 36
author: 
created: 
title: "Conversation::new_id"
tags: []
signature: "pub fn new_id () -> String"
---

A new conversation's id: unique and sortable by creation.

# Examples

```rust
use crawrad_agent::chat_history::Conversation;

let id = Conversation::new_id();
assert!(id.starts_with('c'));
```
