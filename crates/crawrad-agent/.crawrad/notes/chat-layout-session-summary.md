---
id: chat-layout-session-summary
kind: doc
target_file: src/chat_layout.rs
line_hint: 514
author: 
created: 
title: "session_summary"
tags: []
signature: "pub fn session_summary (messages : & [crate :: session :: ChatMessage]) -> String"
---

Formats a short summary label for a chat session.

The session row's title: the first user message, on one line and cut at
28 characters, or `🟢 New session` before anything was asked.

# Examples

```rust
use crawrad_agent::chat_layout::session_summary;

assert_eq!(session_summary(&[]), "🟢 New session");
```
