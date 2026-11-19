---
id: ide-app-chat-bar-ideapp-despachar-cliques-fila-chat
kind: doc
target_file: src/ide_app/chat_bar.rs
line_hint: 200
author: 
created: 
title: "IdeApp::despachar_cliques_fila_chat"
tags: []
signature: "pub fn despachar_cliques_fila_chat (& mut self , cx : & mut Cx , actions : & Actions) -> bool"
---

Dispatches clicks to send, edit, or remove items in the queue's visible slots.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let mut app = IdeApp::default();
assert!(app.chat.queue.is_empty());
```
