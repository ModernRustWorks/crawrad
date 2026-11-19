---
id: client-lspclient-take-code-lens-refresh
kind: doc
target_file: src/client.rs
line_hint: 668
author: 
created: 
title: "LspClient::take_code_lens_refresh"
tags: []
signature: "pub fn take_code_lens_refresh (& self) -> bool"
---

Whether the server asked for the code lenses again since the last call.

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::LspClient;
use std::path::Path;

let client = LspClient::spawn(None, Path::new(".")).unwrap();
assert!(!client.take_code_lens_refresh());
```
