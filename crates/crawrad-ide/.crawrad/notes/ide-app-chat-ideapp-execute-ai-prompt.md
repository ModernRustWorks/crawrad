---
id: ide-app-chat-ideapp-execute-ai-prompt
kind: doc
target_file: src/ide_app/chat.rs
line_hint: 695
author: 
created: 
title: "IdeApp::execute_ai_prompt"
tags: []
signature: "pub fn execute_ai_prompt (& mut self , cx : & mut Cx , prompt : & str)"
---

Executa o envio efetivo do prompt para o chat de IA.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let mut app = IdeApp::default();
assert!(!app.chat.busy);
```
