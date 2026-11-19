---
id: ide-app-chat-bar-ideapp-close-history
kind: doc
target_file: src/ide_app/chat_bar.rs
line_hint: 66
author: 
created: 
title: "IdeApp::close_history"
tags: []
signature: "pub fn close_history (& mut self , cx : & mut Cx) -> bool"
---

Closes the history list; whether it was open (Esc, a click outside).

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let mut app = IdeApp::default();
assert!(app.chat.conversation.is_none());
```
