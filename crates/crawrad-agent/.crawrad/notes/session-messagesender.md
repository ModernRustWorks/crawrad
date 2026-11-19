---
id: session-messagesender
kind: doc
target_file: src/session.rs
line_hint: 6
author: 
created: 
title: "MessageSender"
tags: []
signature: "pub enum MessageSender"
---

Role of who sent a message in the chat conversation.

# Examples

```rust
use crawrad_agent::MessageSender;

assert_eq!(MessageSender::User, MessageSender::User);
```
