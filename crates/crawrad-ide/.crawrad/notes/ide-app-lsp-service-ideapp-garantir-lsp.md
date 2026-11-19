---
id: ide-app-lsp-service-ideapp-garantir-lsp
kind: doc
target_file: src/ide_app/lsp_service.rs
line_hint: 42
author: 
created: 
title: "IdeApp::garantir_lsp"
tags: []
signature: "pub fn garantir_lsp (& mut self)"
---

Ensures that `rust-analyzer` is active for the current workspace.

Initializes or ensures that the `rust-analyzer` instance is active for the current workspace.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
app.garantir_lsp();
assert!(!app.lsp_cell.name.is_empty());
```
