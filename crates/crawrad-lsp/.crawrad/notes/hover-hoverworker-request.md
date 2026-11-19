---
id: hover-hoverworker-request
kind: doc
target_file: src/hover.rs
line_hint: 67
author: 
created: 
title: "HoverWorker::request"
tags: []
signature: "pub fn request (& self , client : Arc < LspClient > , ticket : u64 , file : & Path , version : i32 , text : & str , caret : (usize , usize) ,) -> bool"
---

Asks about the name at `caret` (line, byte) of `file` — its `text` at
`version`.

# Examples

```rust,no_run
// Requesting hover information requires an active language server client.
use crawrad_lsp::LspClient;
use crawrad_lsp::hover::HoverWorker;
use std::path::Path;
use std::sync::Arc;

let client = Arc::new(LspClient::spawn(None, Path::new(".")).unwrap());
let worker = HoverWorker::new();
assert!(worker.request(client, 1, Path::new("src/lib.rs"), 1, "fn main() {}", (0, 0)));
```
