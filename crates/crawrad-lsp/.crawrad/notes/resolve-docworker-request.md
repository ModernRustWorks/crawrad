---
id: resolve-docworker-request
kind: doc
target_file: src/resolve.rs
line_hint: 51
author: 
created: 
title: "DocWorker::request"
tags: []
signature: "pub fn request (& self , client : Arc < LspClient > , ticket : u64 , label : & str , data : Value) -> bool"
---

Resolves the item `label` whose `data` the server sent with the list.

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::client::LspClient;
use crawrad_lsp::resolve::DocWorker;
use serde_json::json;
use std::sync::Arc;

fn resolve_item(worker: &DocWorker, client: Arc<LspClient>) {
    let queued = worker.request(client, 1, "len", json!({"index": 0}));
    assert!(queued);
}
```
