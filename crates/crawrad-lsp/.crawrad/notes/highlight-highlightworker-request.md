---
id: highlight-highlightworker-request
kind: doc
target_file: src/highlight.rs
line_hint: 160
author: 
created: 
title: "HighlightWorker::request"
tags: []
signature: "pub fn request (& self , client : Arc < LspClient > , file : & Path , version : i32 , text : & str , caret : Caret ,) -> bool"
---

Asks for symbol highlights at `caret` unless previously queried.

Asks about the symbol at `caret` of `file` (its `text`, at `version`),
unless that very place was the last one asked. Returns whether it asked.

# Examples

```rust,no_run
// Requesting highlights requires an active language server client.
use crawrad_lsp::LspClient;
use crawrad_lsp::highlight::HighlightWorker;
use std::path::Path;
use std::sync::Arc;

let client = Arc::new(LspClient::spawn(None, Path::new(".")).unwrap());
let worker = HighlightWorker::new();
assert!(worker.request(client, Path::new("src/lib.rs"), 1, "fn main() {}", (0, 0)));
```
