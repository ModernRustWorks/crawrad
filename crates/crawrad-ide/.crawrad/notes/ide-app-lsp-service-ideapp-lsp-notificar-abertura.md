---
id: ide-app-lsp-service-ideapp-lsp-notificar-abertura
kind: doc
target_file: src/ide_app/lsp_service.rs
line_hint: 87
author: 
created: 
title: "IdeApp::lsp_notificar_abertura"
tags: []
signature: "pub fn lsp_notificar_abertura (& mut self , arquivo : & Path , texto : & str)"
---

Notifies the LSP server that a document was opened in the editor.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
app.lsp_notificar_abertura(Path::new("src/main.rs"), "fn main() {}");
assert_eq!(app.lsp_versao, 1);
```
