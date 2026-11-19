---
id: session-chatsession-reset-if-project-changed
kind: doc
target_file: src/session.rs
line_hint: 236
author: 
created: 
title: "ChatSession::reset_if_project_changed"
tags: []
signature: "pub fn reset_if_project_changed (& mut self , new_project_dir : Option < & std :: path :: Path >) -> bool"
---

Resets the CLI session if the active project directory changed.

Returns true if a reset was triggered.

# Examples

```rust
use crawrad_agent::ChatSession;
use std::path::Path;

let mut session = ChatSession::default();
assert!(session.reset_if_project_changed(Some(Path::new("project_a"))));
assert!(!session.reset_if_project_changed(Some(Path::new("project_a"))));
```
