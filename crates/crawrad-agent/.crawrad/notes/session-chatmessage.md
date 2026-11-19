---
id: session-chatmessage
kind: doc
target_file: src/session.rs
line_hint: 47
author: 
created: 
title: "ChatMessage"
tags: []
signature: "pub struct ChatMessage"
---

A single message entry in a chat session.

# Examples

```rust
use crawrad_agent::{ChatMessage, MessageSender};

let msg = ChatMessage::user("m1", "hello");
assert_eq!(msg.sender, MessageSender::User);
```
