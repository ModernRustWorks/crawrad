---
id: ide-app-chat-attachments-ideapp-ask-about-selection
kind: doc
target_file: src/ide_app/chat_attachments.rs
line_hint: 62
author: 
created: 
title: "IdeApp::ask_about_selection"
tags: []
signature: "pub fn ask_about_selection (& mut self , cx : & mut Cx , command : & str)"
---

Asks the AI assistant about the selection using the specified slash command.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let mut app = IdeApp::default();
assert!(app.chat.attachments.is_empty());
```
