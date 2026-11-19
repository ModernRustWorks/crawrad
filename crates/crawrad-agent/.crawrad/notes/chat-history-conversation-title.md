---
id: chat-history-conversation-title
kind: doc
target_file: src/chat_history.rs
line_hint: 46
author: 
created: 
title: "Conversation::title"
tags: []
signature: "pub fn title (& self) -> String"
---

The title the history shows: the first user message's first line.

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
    messages: vec![ChatMessage::user("m1", "Hello world")],
};
assert_eq!(conv.title(), "Hello world");
```
