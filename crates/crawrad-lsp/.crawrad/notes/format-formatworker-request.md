---
id: format-formatworker-request
kind: doc
target_file: src/format.rs
line_hint: 54
author: 
created: 
title: "FormatWorker::request"
tags: []
signature: "pub fn request (& self , client : Arc < LspClient > , ticket : u64 , file : & Path , version : i32 , tab_size : u32 ,) -> bool"
---

Asks for edits formatting `file` with `tab_size` indentation.

Asks for the edits that format `file` at `version`, indenting with
`tab_size` spaces.

# Examples

```rust,no_run
// Requesting formatting requires an active language server client.
use crawrad_lsp::LspClient;
use crawrad_lsp::format::FormatWorker;
use std::path::Path;
use std::sync::Arc;

let client = Arc::new(LspClient::spawn(None, Path::new(".")).unwrap());
let worker = FormatWorker::new();
assert!(worker.request(client, 1, Path::new("src/lib.rs"), 1, 4));
```
