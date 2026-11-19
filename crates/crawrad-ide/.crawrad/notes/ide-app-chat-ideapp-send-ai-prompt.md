---
id: ide-app-chat-ideapp-send-ai-prompt
kind: doc
target_file: src/ide_app/chat.rs
line_hint: 208
author: 
created: 
title: "IdeApp::send_ai_prompt"
tags: []
signature: "pub fn send_ai_prompt (& mut self , cx : & mut Cx)"
---

Triggered when requesting send via button or Enter in the prompt.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let mut app = IdeApp::default();
assert!(app.chat.session.messages.is_empty());
```
