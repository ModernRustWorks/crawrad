---
id: ide-app-chat-ideapp-renderizar-historico-chat
kind: doc
target_file: src/ide_app/chat.rs
line_hint: 233
author: 
created: 
title: "IdeApp::renderizar_historico_chat"
tags: []
signature: "pub fn renderizar_historico_chat (& mut self , cx : & mut Cx)"
---

Renders AI session messages in the chat.

Renders AI session messages across the multiple conversation cards in the chat.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let mut app = IdeApp::default();
assert!(app.chat.session.messages.is_empty());
```
