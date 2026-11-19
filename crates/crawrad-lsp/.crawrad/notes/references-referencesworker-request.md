---
id: references-referencesworker-request
kind: doc
target_file: src/references.rs
line_hint: 61
author: 
created: 
title: "ReferencesWorker::request"
tags: []
signature: "pub fn request (& self , client : Arc < LspClient > , ticket : u64 , file : & Path , version : i32 , text : & str , caret : (usize , usize) ,) -> bool"
---

Asks for all references to the symbol at caret in file.

Asks for every reference to the symbol at `caret` (line, byte) of
`file` — its `text` at `version` — its declaration included.

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::client::LspClient;
use crawrad_lsp::references::ReferencesWorker;
use std::path::Path;
use std::sync::Arc;

fn ask_refs(worker: &ReferencesWorker, client: Arc<LspClient>) {
    let file = Path::new("src/main.rs");
    let queued = worker.request(client, 1, file, 1, "fn main() {}\n", (0, 3));
    assert!(queued);
}
```
