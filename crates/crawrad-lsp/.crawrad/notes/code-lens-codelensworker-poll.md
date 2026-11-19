---
id: code-lens-codelensworker-poll
kind: doc
target_file: src/code_lens.rs
line_hint: 239
author: 
created: 
title: "CodeLensWorker::poll"
tags: []
signature: "pub fn poll (& self , client : Arc < LspClient > , file : & Path , version : i32) -> Option < Vec < CodeLens > >"
---

Polls for lens updates once per frame for the active editor file.

Once a frame, for the file in the editor: asks again when the server
asked for it (`workspace/codeLens/refresh`), and returns the lenses
when an answer for THIS file at THIS version arrived.

# Examples

```rust,no_run
// Polling code lenses requires a running language server client.
use crawrad_lsp::LspClient;
use crawrad_lsp::code_lens::CodeLensWorker;
use std::path::Path;
use std::sync::Arc;

let client = Arc::new(LspClient::spawn(None, Path::new(".")).unwrap());
let worker = CodeLensWorker::new();
let lenses = worker.poll(client, Path::new("src/lib.rs"), 1);
assert_eq!(lenses, None);
```
