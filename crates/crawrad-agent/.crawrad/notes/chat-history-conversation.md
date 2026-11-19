---
id: chat-history-conversation
kind: doc
target_file: src/chat_history.rs
line_hint: 15
author: 
created: 
title: "Conversation"
tags: []
signature: "pub struct Conversation"
---

One stored conversation.

# Examples

```rust
use crawrad_agent::chat_history::Conversation;

let conv = Conversation {
    id: "c1".to_string(),
    agent: "Claude".to_string(),
    agent_key: "claude".to_string(),
    acp_session: None,
    can_resume: false,
    updated: "2026-11-05 10:00".to_string(),
    messages: vec![],
};
assert_eq!(conv.id, "c1");
```
