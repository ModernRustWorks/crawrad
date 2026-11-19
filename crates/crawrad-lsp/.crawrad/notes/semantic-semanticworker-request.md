---
id: semantic-semanticworker-request
kind: doc
target_file: src/semantic.rs
line_hint: 226
author: 
created: 
title: "SemanticWorker::request"
tags: []
signature: "pub fn request (& self , client : Arc < LspClient > , ticket : u64 , file : & Path , version : i32) -> bool"
---

Asks for semantic tokens for a file at a specific version.

Asks for `file` at `version`: a delta when an answer is kept, else
the whole file. A failed delta forgets the kept one, so the next
request is whole.

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::client::LspClient;
use crawrad_lsp::semantic::SemanticWorker;
use std::path::Path;
use std::sync::Arc;

fn req_tokens(worker: &SemanticWorker, client: Arc<LspClient>) {
    let file = Path::new("src/main.rs");
    let queued = worker.request(client, 1, file, 1);
    assert!(queued);
}
```
