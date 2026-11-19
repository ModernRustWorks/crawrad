---
id: ide-app-chat-attachments-ideapp-dispatch-attachment-clicks
kind: doc
target_file: src/ide_app/chat_attachments.rs
line_hint: 126
author: 
created: 
title: "IdeApp::dispatch_attachment_clicks"
tags: []
signature: "pub fn dispatch_attachment_clicks (& mut self , cx : & mut Cx , actions : & Actions) -> bool"
---

A ✕ removes its attachment.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let mut app = IdeApp::default();
assert!(app.chat.attachments.is_empty());
```
