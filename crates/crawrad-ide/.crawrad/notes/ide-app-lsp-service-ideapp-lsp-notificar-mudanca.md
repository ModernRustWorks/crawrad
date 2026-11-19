---
id: ide-app-lsp-service-ideapp-lsp-notificar-mudanca
kind: doc
target_file: src/ide_app/lsp_service.rs
line_hint: 105
author: 
created: 
title: "IdeApp::lsp_notificar_mudanca"
tags: []
signature: "pub fn lsp_notificar_mudanca (& mut self , arquivo : & Path , texto : & str)"
---

Notifies the LSP server that a document was modified.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
app.lsp_notificar_mudanca(Path::new("src/main.rs"), "fn main() {}");
assert!(app.lsp_versao >= 1);
```
