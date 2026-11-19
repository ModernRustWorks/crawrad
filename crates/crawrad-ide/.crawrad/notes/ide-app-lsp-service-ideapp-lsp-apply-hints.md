---
id: ide-app-lsp-service-ideapp-lsp-apply-hints
kind: doc
target_file: src/ide_app/lsp_service.rs
line_hint: 139
author: 
created: 
title: "IdeApp::lsp_apply_hints"
tags: []
signature: "pub fn lsp_apply_hints (& mut self , cx : & mut Cx)"
---

The types rust-analyzer infers, in grey inside the lines (issue #255).

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
app.lsp_apply_hints(cx);
assert_eq!(app.inlay_hints_off, false);
```
