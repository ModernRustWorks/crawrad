---
id: client-lspclient-status-name
kind: doc
target_file: src/client.rs
line_hint: 1079
author: 
created: 
title: "LspClient::status_name"
tags: []
signature: "pub fn status_name (& self) -> & 'static str"
---

Returns the server's descriptive name formatted for the status bar.

The server's name on the status bar: `rust-analyzer`, or
`rust-analyzer (system copy)` when it is the machine's and not the
IDE's own (ADR 0050) — idle or indexing, the warning stays.

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::LspClient;
use std::path::Path;

let client = LspClient::spawn(None, Path::new(".")).unwrap();
assert_eq!(client.status_name(), "rust-analyzer");
```
