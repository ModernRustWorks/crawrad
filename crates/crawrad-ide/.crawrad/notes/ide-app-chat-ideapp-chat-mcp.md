---
id: ide-app-chat-ideapp-chat-mcp
kind: doc
target_file: src/ide_app/chat.rs
line_hint: 662
author: 
created: 
title: "IdeApp::chat_mcp"
tags: []
signature: "pub fn chat_mcp (& mut self , cx : & mut Cx , name : & str , args : & serde_json :: Value)"
---

Dispatches chat controls as MCP tools.

The chat's controls as MCP tools (#420), each the same path as its click:
`chat.send {text}`, `chat.stop`, `chat.new_session`, `chat.set_context {form?, selection?}`.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let mut app = IdeApp::default();
assert!(!app.chat.busy);
```
