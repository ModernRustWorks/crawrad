---
id: ide-app-chat-bar-ideapp-remove-ai-queue-item
kind: doc
target_file: src/ide_app/chat_bar.rs
line_hint: 168
author: 
created: 
title: "IdeApp::remove_ai_queue_item"
tags: []
signature: "pub fn remove_ai_queue_item (& mut self , cx : & mut Cx , index : usize)"
---

Removes a specific item from the sending queue.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let mut app = IdeApp::default();
assert!(app.chat.queue.is_empty());
```
