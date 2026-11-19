---
id: code-lens-codelensworker-request
kind: doc
target_file: src/code_lens.rs
line_hint: 230
author: 
created: 
title: "CodeLensWorker::request"
tags: []
signature: "pub fn request (& self , client : Arc < LspClient > , file : & Path , version : i32)"
---

Asks for the lenses of `file` at document `version`.

# Examples

```rust,no_run
// Requesting code lenses requires a running language server client.
use crawrad_lsp::LspClient;
use crawrad_lsp::code_lens::CodeLensWorker;
use std::path::Path;
use std::sync::Arc;

let client = Arc::new(LspClient::spawn(None, Path::new(".")).unwrap());
let worker = CodeLensWorker::new();
worker.request(client, Path::new("src/lib.rs"), 1);
assert!(worker.shown().is_empty());
```
