---
id: ide-app-chat-bar-ideapp-editar-ai-queue-item
kind: doc
target_file: src/ide_app/chat_bar.rs
line_hint: 176
author: 
created: 
title: "IdeApp::editar_ai_queue_item"
tags: []
signature: "pub fn editar_ai_queue_item (& mut self , cx : & mut Cx , index : usize)"
---

Removes an item from the queue and places it in the input field for editing.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let mut app = IdeApp::default();
assert!(app.chat.queue.is_empty());
```
