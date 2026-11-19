---
id: rename-renameworker-prepare
kind: doc
target_file: src/rename.rs
line_hint: 89
author: 
created: 
title: "RenameWorker::prepare"
tags: []
signature: "pub fn prepare (& self , client : Arc < LspClient > , ticket : u64 , asked : Asked) -> bool"
---

Asks whether the name at `asked` can be renamed.

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::client::LspClient;
use crawrad_lsp::rename::{Asked, RenameWorker};
use std::path::Path;
use std::sync::Arc;

fn prepare_rename(worker: &RenameWorker, client: Arc<LspClient>) {
    let asked = Asked {
        file: Path::new("src/main.rs"),
        version: 1,
        text: "fn main() {}\n",
        caret: (0, 3),
    };
    let queued = worker.prepare(client, 1, asked);
    assert!(queued);
}
```
