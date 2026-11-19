---
id: chat-layout-user-meta-text
kind: doc
target_file: src/chat_layout.rs
line_hint: 322
author: 
created: 
title: "user_meta_text"
tags: []
signature: "pub fn user_meta_text (turn : & ChatTurn) -> String"
---

Formats the user message bubble metadata as plain text.

The user bubble's meta as plain text: `"19:40 ✓✓"`, or the word alone for
a message saved before messages carried a time.

# Examples

```rust
use crawrad_agent::chat_layout::{ChatTurn, user_meta_text};

let turn = ChatTurn {
    user_text: Some("hi".to_string()),
    user_sent_at: Some("12:00".to_string()),
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
assert_eq!(user_meta_text(&turn), "12:00 ✓");
```
