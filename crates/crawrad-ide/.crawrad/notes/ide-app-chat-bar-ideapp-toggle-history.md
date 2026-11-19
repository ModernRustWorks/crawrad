---
id: ide-app-chat-bar-ideapp-toggle-history
kind: doc
target_file: src/ide_app/chat_bar.rs
line_hint: 44
author: 
created: 
title: "IdeApp::toggle_history"
tags: []
signature: "pub fn toggle_history (& mut self , cx : & mut Cx)"
---

Toggles the conversation history popup list.

The clock button (#478): the history list under it, inside the panel — or closed.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let mut app = IdeApp::default();
assert!(app.chat.conversation.is_none());
```
