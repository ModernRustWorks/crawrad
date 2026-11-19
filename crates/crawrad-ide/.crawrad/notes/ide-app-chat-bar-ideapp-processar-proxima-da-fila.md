---
id: ide-app-chat-bar-ideapp-processar-proxima-da-fila
kind: doc
target_file: src/ide_app/chat_bar.rs
line_hint: 223
author: 
created: 
title: "IdeApp::processar_proxima_da_fila"
tags: []
signature: "pub fn processar_proxima_da_fila (& mut self , cx : & mut Cx)"
---

Processes and sends the next message from the AI queue.

Processes and sends the next message from the queue if one exists and the agent is free.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let mut app = IdeApp::default();
assert!(app.chat.queue.is_empty());
```
