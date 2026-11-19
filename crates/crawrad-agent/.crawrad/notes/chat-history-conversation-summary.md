---
id: chat-history-conversation-summary
kind: doc
target_file: src/chat_history.rs
line_hint: 69
author: 
created: 
title: "Conversation::summary"
tags: []
signature: "pub fn summary (& self , today : & str) -> Value"
---

Its row in the history list, dated against `today` (`"YYYY-MM-DD"`).

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
let s = conv.summary("2026-11-05");
assert_eq!(s["id"], "c1");
```
