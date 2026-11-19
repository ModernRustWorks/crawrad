---
id: client-lspclient-shutdown
kind: doc
target_file: src/client.rs
line_hint: 1103
author: 
created: 
title: "LspClient::shutdown"
tags: []
signature: "pub fn shutdown (& self)"
---

Shuts down the server process and terminates its process tree.

Stops the server (LSP 3.17, *Shutdown Request* then *Exit
Notification*): asks, waits for the answer, lets it exit, waits for
the process — and then ends its whole tree, so no `cargo check` it
started is left orphaned holding the build lock (#367). Only the
first call does anything.

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::LspClient;
use std::path::Path;

let client = LspClient::spawn(None, Path::new(".")).unwrap();
client.shutdown();
assert!(!client.is_alive());
```
