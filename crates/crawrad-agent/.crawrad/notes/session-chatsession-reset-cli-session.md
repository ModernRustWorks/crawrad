---
id: session-chatsession-reset-cli-session
kind: doc
target_file: src/session.rs
line_hint: 230
author: 
created: 
title: "ChatSession::reset_cli_session"
tags: []
signature: "pub fn reset_cli_session (& mut self)"
---

Resets the CLI session state so the next turn starts fresh.

Resets the CLI session (e.g. after a failure to launch so the next turn starts fresh).

# Examples

```rust
use crawrad_agent::ChatSession;

let mut session = ChatSession::default();
session.mark_session_started();
session.reset_cli_session();
assert!(!session.session_started);
```
