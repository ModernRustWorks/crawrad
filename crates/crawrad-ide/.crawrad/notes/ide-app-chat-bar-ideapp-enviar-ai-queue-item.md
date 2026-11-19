---
id: ide-app-chat-bar-ideapp-enviar-ai-queue-item
kind: doc
target_file: src/ide_app/chat_bar.rs
line_hint: 185
author: 
created: 
title: "IdeApp::enviar_ai_queue_item"
tags: []
signature: "pub fn enviar_ai_queue_item (& mut self , cx : & mut Cx , index : usize)"
---

Immediately sends a specific message from the queue (jumping the queue).

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let mut app = IdeApp::default();
assert!(app.chat.queue.is_empty());
```
