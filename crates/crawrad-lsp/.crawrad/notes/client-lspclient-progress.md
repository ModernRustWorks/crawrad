---
id: client-lspclient-progress
kind: doc
target_file: src/client.rs
line_hint: 645
author: 
created: 
title: "LspClient::progress"
tags: []
signature: "pub fn progress (& self) -> Option < String >"
---

Returns the current progress description for the status bar, if any.

What the server is busy with, for the status bar: `Indexing 120/340
(35%)`; `None` when it is idle.

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::LspClient;
use std::path::Path;

let client = LspClient::spawn(None, Path::new(".")).unwrap();
assert_eq!(client.progress(), None);
```
