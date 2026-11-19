---
id: client-lspclient-poll-diagnostics
kind: doc
target_file: src/client.rs
line_hint: 320
author: 
created: 
title: "LspClient::poll_diagnostics"
tags: []
signature: "pub fn poll_diagnostics (& self) -> Vec < PublishDiagnosticsParams >"
---

Drains any pending compiler and linter diagnostics received from the server.

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::LspClient;
use std::path::Path;

let client = LspClient::spawn(None, Path::new(".")).unwrap();
let diags = client.poll_diagnostics();
assert!(diags.is_empty());
```
