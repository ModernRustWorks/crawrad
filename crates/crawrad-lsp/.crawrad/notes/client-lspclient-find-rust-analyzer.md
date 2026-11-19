---
id: client-lspclient-find-rust-analyzer
kind: doc
target_file: src/client.rs
line_hint: 127
author: 
created: 
title: "LspClient::find_rust_analyzer"
tags: []
signature: "pub fn find_rust_analyzer () -> Option < PathBuf >"
---

Locates the `rust-analyzer` binary used by the IDE.

The `rust-analyzer` the IDE uses — its own first, the machine's as
the last resort. See [`crate::locate`].

# Examples

```rust,no_run
// Locating the binary inspects system environment and directories.
use crawrad_lsp::LspClient;

let path = LspClient::find_rust_analyzer().unwrap();
assert!(path.is_file());
```
