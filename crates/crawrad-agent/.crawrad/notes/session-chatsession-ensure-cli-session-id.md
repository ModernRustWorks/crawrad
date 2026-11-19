---
id: session-chatsession-ensure-cli-session-id
kind: doc
target_file: src/session.rs
line_hint: 216
author: 
created: 
title: "ChatSession::ensure_cli_session_id"
tags: []
signature: "pub fn ensure_cli_session_id (& mut self) -> & str"
---

Obtains or allocates a UUID for the CLI session.

# Panics

Panics if the CLI session identifier cannot be unwrapped.

# Examples

```rust
use crawrad_agent::ChatSession;

let mut session = ChatSession::default();
let id = session.ensure_cli_session_id();
assert!(!id.is_empty());
```
