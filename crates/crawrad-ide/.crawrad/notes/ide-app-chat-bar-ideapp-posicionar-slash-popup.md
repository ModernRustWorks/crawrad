---
id: ide-app-chat-bar-ideapp-posicionar-slash-popup
kind: doc
target_file: src/ide_app/chat_bar.rs
line_hint: 303
author: 
created: 
title: "IdeApp::posicionar_slash_popup"
tags: []
signature: "pub fn posicionar_slash_popup (& mut self , cx : & mut Cx)"
---

Posiciona o popup de comandos de barra (/).

Posiciona o popup de comandos de barra (/) ancorado logo acima da barra de prompt.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let mut app = IdeApp::default();
assert!(!app.chat.busy);
```
