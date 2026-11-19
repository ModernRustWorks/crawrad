---
id: ide-app-chat-ideapp-chat-messages-json
kind: doc
target_file: src/ide_app/chat.rs
line_hint: 408
author: 
created: 
title: "IdeApp::chat_messages_json"
tags: []
signature: "pub fn chat_messages_json (& self , cx : & Cx) -> serde_json :: Value"
---

Returns JSON representation of chat turns and buttons for MCP (view.get_chat).

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let app = IdeApp::default();
assert!(app.chat.session.messages.is_empty());
```
