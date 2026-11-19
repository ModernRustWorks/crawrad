---
id: chat-history-conversation-turns
kind: doc
target_file: src/chat_history.rs
line_hint: 61
author: 
created: 
title: "Conversation::turns"
tags: []
signature: "pub fn turns (& self) -> usize"
---

How many times the user spoke.

# Examples

```rust
use crawrad_agent::chat_history::Conversation;
use crawrad_agent::session::ChatMessage;

let conv = Conversation {
    id: "c1".to_string(),
    agent: "Claude".to_string(),
    agent_key: "claude".to_string(),
    acp_session: None,
    can_resume: false,
    updated: "2026-11-05 10:00".to_string(),
    messages: vec![ChatMessage::user("m1", "Hello")],
};
assert_eq!(conv.turns(), 1);
```
