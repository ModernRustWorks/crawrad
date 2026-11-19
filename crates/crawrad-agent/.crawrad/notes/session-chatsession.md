---
id: session-chatsession
kind: doc
target_file: src/session.rs
line_hint: 160
author: 
created: 
title: "ChatSession"
tags: []
signature: "pub struct ChatSession"
---

Active conversation session state.

# Examples

```rust
use crawrad_agent::ChatSession;

let session = ChatSession::new("s1", "Session 1");
assert!(session.is_empty());
```
