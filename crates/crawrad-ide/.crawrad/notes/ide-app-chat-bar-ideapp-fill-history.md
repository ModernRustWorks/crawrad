---
id: ide-app-chat-bar-ideapp-fill-history
kind: doc
target_file: src/ide_app/chat_bar.rs
line_hint: 75
author: 
created: 
title: "IdeApp::fill_history"
tags: []
signature: "pub fn fill_history (& mut self , cx : & mut Cx)"
---

Draws the stored conversations the search lets through, newest first.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let mut app = IdeApp::default();
assert!(app.chat.conversation.is_none());
```
