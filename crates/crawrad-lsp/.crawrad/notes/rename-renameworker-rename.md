---
id: rename-renameworker-rename
kind: doc
target_file: src/rename.rs
line_hint: 115
author: 
created: 
title: "RenameWorker::rename"
tags: []
signature: "pub fn rename (& self , client : Arc < LspClient > , ticket : u64 , asked : Asked , new_name : & str ,) -> bool"
---

Asks for the edits that rename the name at `asked` to `new_name`.

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::client::LspClient;
use crawrad_lsp::rename::{Asked, RenameWorker};
use std::path::Path;
use std::sync::Arc;

fn do_rename(worker: &RenameWorker, client: Arc<LspClient>) {
    let asked = Asked {
        file: Path::new("src/main.rs"),
        version: 1,
        text: "fn main() {}\n",
        caret: (0, 3),
    };
    let queued = worker.rename(client, 1, asked, "new_ident");
    assert!(queued);
}
```
