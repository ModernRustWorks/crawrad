---
id: ide-app-chat-bar-ideapp-alternar-ai-queue-expanded
kind: doc
target_file: src/ide_app/chat_bar.rs
line_hint: 194
author: 
created: 
title: "IdeApp::alternar_ai_queue_expanded"
tags: []
signature: "pub fn alternar_ai_queue_expanded (& mut self , cx : & mut Cx)"
---

Alterna a visibilidade da lista de itens da fila de mensagens.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let mut app = IdeApp::default();
assert!(app.chat.queue.is_empty());
```
