---
id: inlay-inlayworker-poll
kind: doc
target_file: src/inlay.rs
line_hint: 184
author: 
created: 
title: "InlayWorker::poll"
tags: []
signature: "pub fn poll (& self , client : Arc < LspClient > , file : & Path , version : i32 , text : & str ,) -> Option < Vec < (usize , usize , String) > >"
---

Polls for fresh inlay hints once per frame for the active editor file.

Once a frame, for the file in the editor: asks again when the server
asked for it (`workspace/inlayHint/refresh`), and returns the hints to
draw — placed on `text` — when an answer for THIS file at THIS version
arrived. An answer for another file or an older version is dropped.

# Examples

```rust,no_run
// Polling inlay hints requires an active language server client.
use crawrad_lsp::LspClient;
use crawrad_lsp::inlay::InlayWorker;
use std::path::Path;
use std::sync::Arc;

let client = Arc::new(LspClient::spawn(None, Path::new(".")).unwrap());
let worker = InlayWorker::new();
let hints = worker.poll(client, Path::new("src/lib.rs"), 1, "fn main() {}");
assert_eq!(hints, None);
```
