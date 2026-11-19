---
id: chat-layout-chatturn
kind: doc
target_file: src/chat_layout.rs
line_hint: 136
author: 
created: 
title: "ChatTurn"
tags: []
signature: "pub struct ChatTurn"
---

A single conversational turn with optional user prompt and reply.

A single conversational turn consisting of an optional user prompt and an optional reply.

# Examples

```rust
use crawrad_agent::chat_layout::ChatTurn;

let turn = ChatTurn {
    user_text: Some("hi".to_string()),
    user_sent_at: None,
    user_meta: None,
    user_mark: None,
    reply_text: None,
    reply_segments: vec![],
    reply_sent_at: None,
    reply_meta: None,
    sender_name: None,
    tool_calls: vec![],
    notice: None,
    status_line: None,
    has_reply: false,
};
assert_eq!(turn.user_text.as_deref(), Some("hi"));
```
