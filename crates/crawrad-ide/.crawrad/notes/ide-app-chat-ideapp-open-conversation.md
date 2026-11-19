---
id: ide-app-chat-ideapp-open-conversation
kind: doc
target_file: src/ide_app/chat.rs
line_hint: 574
author: 
created: 
title: "IdeApp::open_conversation"
tags: []
signature: "pub fn open_conversation (& mut self , cx : & mut Cx , id : & str)"
---

Opens a saved conversation from history.

Opens conversation `id` from the history (#478): its turns, and its ACP
session when the same agent can load it. An unknown id changes nothing.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let mut app = IdeApp::default();
assert!(app.chat.conversation.is_none());
```
