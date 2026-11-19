---
id: ide-app-chat-bar-ideapp-enfileirar-ai-prompt
kind: doc
target_file: src/ide_app/chat_bar.rs
line_hint: 149
author: 
created: 
title: "IdeApp::enfileirar_ai_prompt"
tags: []
signature: "pub fn enfileirar_ai_prompt (& mut self , cx : & mut Cx , prompt : & str , attachments : Vec < SelectionAttachment > , images : Vec < crawrad_agent :: ImageAttachment > ,)"
---

Enfileira um prompt na fila de envio do assistente de IA.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let mut app = IdeApp::default();
assert!(app.chat.queue.is_empty());
```
