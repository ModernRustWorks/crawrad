---
id: code-action-codeactionworker-request
kind: doc
target_file: src/code_action.rs
line_hint: 149
author: 
created: 
title: "CodeActionWorker::request"
tags: []
signature: "pub fn request (& self , client : Arc < LspClient > , ticket : u64 , asked : & Asked < '_ >) -> bool"
---

Asks what code actions can be performed at the caret.

Asks what can be done at the caret. The context carries the
diagnostics the client holds on that line (LSP: *"known on the client
side overlapping the range"*).

# Examples

```rust,no_run
// Running code action requests requires an active language server.
use crawrad_lsp::LspClient;
use crawrad_lsp::code_action::{Asked, CodeActionWorker};
use std::path::Path;
use std::sync::Arc;

let client = Arc::new(LspClient::spawn(None, Path::new(".")).unwrap());
let worker = CodeActionWorker::new();
let asked = Asked {
    file: Path::new("src/lib.rs"),
    version: 1,
    text: "fn main() {}",
    caret: (0, 0),
};
assert!(worker.request(client, 1, &asked));
```
