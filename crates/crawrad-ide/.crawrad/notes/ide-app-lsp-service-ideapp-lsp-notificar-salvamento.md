---
id: ide-app-lsp-service-ideapp-lsp-notificar-salvamento
kind: doc
target_file: src/ide_app/lsp_service.rs
line_hint: 122
author: 
created: 
title: "IdeApp::lsp_notificar_salvamento"
tags: []
signature: "pub fn lsp_notificar_salvamento (& mut self , arquivo : & Path)"
---

Notifies the LSP server that a document was saved to disk.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let path = Path::new("src/main.rs");
app.lsp_notificar_salvamento(path);
assert_eq!(path.extension().and_then(|e| e.to_str()), Some("rs"));
```
