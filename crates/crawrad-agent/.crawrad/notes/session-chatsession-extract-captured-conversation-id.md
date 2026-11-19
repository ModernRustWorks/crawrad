---
id: session-chatsession-extract-captured-conversation-id
kind: doc
target_file: src/session.rs
line_hint: 246
author: 
created: 
title: "ChatSession::extract_captured_conversation_id"
tags: []
signature: "pub fn extract_captured_conversation_id (output : & str) -> Option < String >"
---

Extracts a conversation ID from captured CLI output (e.g. agy init events).

# Examples

```rust
use crawrad_agent::ChatSession;

let line = r#"{"event":"init","conversation_id":"c123"}"#;
assert_eq!(ChatSession::extract_captured_conversation_id(line), Some("c123".to_string()));
```
