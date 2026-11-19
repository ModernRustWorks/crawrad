---
id: client-lspclient
kind: doc
target_file: src/client.rs
line_hint: 51
author: 
created: 
title: "LspClient"
tags: []
signature: "pub struct LspClient"
---

Active LSP client managing a background language server process.

# Examples

```rust,no_run
// Managing a language server process requires a real binary on disk.
use crawrad_lsp::LspClient;
use std::path::Path;

let client = LspClient::spawn(None, Path::new(".")).unwrap();
assert!(client.is_alive());
```
