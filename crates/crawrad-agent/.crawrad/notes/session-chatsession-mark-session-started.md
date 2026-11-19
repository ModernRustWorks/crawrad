---
id: session-chatsession-mark-session-started
kind: doc
target_file: src/session.rs
line_hint: 225
author: 
created: 
title: "ChatSession::mark_session_started"
tags: []
signature: "pub fn mark_session_started (& mut self)"
---

Marks that the CLI session process executed successfully and can be resumed.

# Examples

```rust
use crawrad_agent::ChatSession;

let mut session = ChatSession::default();
session.mark_session_started();
assert!(session.session_started);
```
