---
id: ide-app-lsp-service-ideapp-lsp-command
kind: doc
target_file: src/ide_app/lsp_service.rs
line_hint: 648
author: 
created: 
title: "IdeApp::lsp_command"
tags: []
signature: "pub fn lsp_command (& mut self , command : crawrad_actions :: LspCommand)"
---

Dispatches an LSP lifecycle command like reload, restart, or stop.

The status bar's rust-analyzer menu: reload, restart or stop. The
pill follows at the end of the event (`lsp_apply_hints`).

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
app.lsp_command(crawrad_actions::LspCommand::Stop);
assert!(app.lsp.is_none());
```
