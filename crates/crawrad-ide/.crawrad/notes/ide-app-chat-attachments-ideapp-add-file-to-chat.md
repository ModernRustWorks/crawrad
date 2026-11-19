---
id: ide-app-chat-attachments-ideapp-add-file-to-chat
kind: doc
target_file: src/ide_app/chat_attachments.rs
line_hint: 42
author: 
created: 
title: "IdeApp::add_file_to_chat"
tags: []
signature: "pub fn add_file_to_chat (& mut self , cx : & mut Cx)"
---

Inserts the active file reference (@file) into prompt input; sends nothing.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let mut app = IdeApp::default();
assert!(app.chat.attachments.is_empty());
```
