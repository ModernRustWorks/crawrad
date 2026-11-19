---
id: inlay-inlayworker-request
kind: doc
target_file: src/inlay.rs
line_hint: 173
author: 
created: 
title: "InlayWorker::request"
tags: []
signature: "pub fn request (& self , client : Arc < LspClient > , file : & Path , version : i32 , text : & str)"
---

Asks for the hints of `file` (its `text`) at document `version`.

# Examples

```rust,no_run
// Requesting inlay hints requires an active language server client.
use crawrad_lsp::LspClient;
use crawrad_lsp::inlay::InlayWorker;
use std::path::Path;
use std::sync::Arc;

let client = Arc::new(LspClient::spawn(None, Path::new(".")).unwrap());
let worker = InlayWorker::new();
worker.request(client, Path::new("src/lib.rs"), 1, "fn main() {}");
assert!(worker.take().is_none());
```
