---
id: ide-app-chat-bar-ideapp-update-ai-queue-visual
kind: doc
target_file: src/ide_app/chat_bar.rs
line_hint: 331
author: 
created: 
title: "IdeApp::update_ai_queue_visual"
tags: []
signature: "pub fn update_ai_queue_visual (& mut self , cx : & mut Cx)"
---

Atualiza todos os elementos visuais do card da fila de envio (Queued Messages).

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let mut app = IdeApp::default();
assert!(app.chat.queue.is_empty());
```
