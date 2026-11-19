---
id: ide-app-chat-ideapp-new-chat-session
kind: doc
target_file: src/ide_app/chat.rs
line_hint: 586
author: 
created: 
title: "IdeApp::new_chat_session"
tags: []
signature: "pub fn new_chat_session (& mut self , cx : & mut Cx)"
---

New Session / Clear: a new conversation, a new ACP session (#429).

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let mut app = IdeApp::default();
assert!(app.chat.session.messages.is_empty());
```

# Panics

Panics if the session permissions mutex is poisoned.
