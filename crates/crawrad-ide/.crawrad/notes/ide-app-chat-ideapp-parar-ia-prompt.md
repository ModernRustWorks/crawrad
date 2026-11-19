---
id: ide-app-chat-ideapp-parar-ia-prompt
kind: doc
target_file: src/ide_app/chat.rs
line_hint: 1041
author: 
created: 
title: "IdeApp::parar_ia_prompt"
tags: []
signature: "pub fn parar_ia_prompt (& mut self , cx : & mut Cx)"
---

Interrompe imediatamente o processamento ativo do agente de IA.

Interrompe imediatamente o processamento ativo do agente de IA e cancela o turno.
Preserva a fila de envio (`ai_queue`) intacta.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let mut app = IdeApp::default();
assert!(!app.chat.busy);
```
