---
id: ide-app-chat-ideapp-update-permission-card
kind: doc
target_file: src/ide_app/chat.rs
line_hint: 985
author: 
created: 
title: "IdeApp::update_permission_card"
tags: []
signature: "pub fn update_permission_card (& mut self , cx : & mut Cx)"
---

Updates the permission visual card in the chat panel based on the active queue.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let mut app = IdeApp::default();
assert!(app.chat.permission_queue.is_empty());
```
