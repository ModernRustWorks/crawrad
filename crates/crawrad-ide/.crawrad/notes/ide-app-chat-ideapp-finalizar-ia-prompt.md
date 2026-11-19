---
id: ide-app-chat-ideapp-finalizar-ia-prompt
kind: doc
target_file: src/ide_app/chat.rs
line_hint: 839
author: 
created: 
title: "IdeApp::finalizar_ia_prompt"
tags: []
signature: "pub fn finalizar_ia_prompt (& mut self , cx : & mut Cx)"
---

Concludes the AI assistant's asynchronous processing.

Concludes the AI assistant's asynchronous processing when the timer expires or the response arrives.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let mut app = IdeApp::default();
assert!(!app.chat.busy);
```
