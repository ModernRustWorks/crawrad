---
id: ide-app-lsp-service-ideapp-lsp-process-diagnostics
kind: doc
target_file: src/ide_app/lsp_service.rs
line_hint: 722
author: 
created: 
title: "IdeApp::lsp_process_diagnostics"
tags: []
signature: "pub fn lsp_process_diagnostics (& mut self , cx : & mut Cx)"
---

Processes any pending diagnostics from rust-analyzer and updates squiggles.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
app.lsp_process_diagnostics(cx);
assert_eq!(app.inlay_hints_off, false);
```
