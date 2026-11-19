---
id: complete-completionworker-request
kind: doc
target_file: src/complete.rs
line_hint: 139
author: 
created: 
title: "CompletionWorker::request"
tags: []
signature: "pub fn request (& self , client : Arc < LspClient > , file : & Path , at : Position , ticket : u64 , trigger : Trigger ,)"
---

Asks for the completions at `at` (UTF-16 column) in `file`.

# Examples

```rust,no_run
// Requesting completions requires an active language server client.
use crawrad_lsp::LspClient;
use crawrad_lsp::complete::{CompletionWorker, Trigger};
use crawrad_lsp::protocol::Position;
use std::path::Path;
use std::sync::Arc;

let client = Arc::new(LspClient::spawn(None, Path::new(".")).unwrap());
let worker = CompletionWorker::new();
worker.request(client, Path::new("src/lib.rs"), Position::new(0, 0), 1, Trigger::Invoked);
assert!(worker.take(1).is_none());
```
