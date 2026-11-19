---
id: symbols-symbolworker-request
kind: doc
target_file: src/symbols.rs
line_hint: 53
author: 
created: 
title: "SymbolWorker::request"
tags: []
signature: "pub fn request (& self , client : Arc < LspClient > , ticket : u64 , query : & str) -> bool"
---

Asks for the items whose name matches `query`.

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::client::LspClient;
use crawrad_lsp::symbols::SymbolWorker;
use std::sync::Arc;

fn req_syms(worker: &SymbolWorker, client: Arc<LspClient>) {
    let queued = worker.request(client, 1, "test");
    assert!(queued);
}
```
