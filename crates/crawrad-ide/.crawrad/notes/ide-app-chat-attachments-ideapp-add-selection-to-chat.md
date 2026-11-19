---
id: ide-app-chat-attachments-ideapp-add-selection-to-chat
kind: doc
target_file: src/ide_app/chat_attachments.rs
line_hint: 22
author: 
created: 
title: "IdeApp::add_selection_to_chat"
tags: []
signature: "pub fn add_selection_to_chat (& mut self , cx : & mut Cx)"
---

Captures the editor's selection NOW, for the next message; sends nothing.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let mut app = IdeApp::default();
assert!(app.chat.attachments.is_empty());
```
