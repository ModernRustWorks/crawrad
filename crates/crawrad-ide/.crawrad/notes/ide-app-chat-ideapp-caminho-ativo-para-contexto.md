---
id: ide-app-chat-ideapp-caminho-ativo-para-contexto
kind: doc
target_file: src/ide_app/chat.rs
line_hint: 480
author: 
created: 
title: "IdeApp::caminho_ativo_para_contexto"
tags: []
signature: "pub fn caminho_ativo_para_contexto (& self) -> Option < std :: path :: PathBuf >"
---

Returns the path of the active file in the editor or form in the designer.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let app = IdeApp::default();
assert_eq!(app.caminho_ativo_para_contexto(), None);
```
