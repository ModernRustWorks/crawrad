---
id: client-lspclient-spawn
kind: doc
target_file: src/client.rs
line_hint: 133
author: 
created: 
title: "LspClient::spawn"
tags: []
signature: "pub fn spawn (server_path : Option < PathBuf > , workspace_root : & Path) -> Result < Self , LspError >"
---

Spawns the language server process and starts background I/O threads.

# Examples

```rust,no_run
// Spawning rust-analyzer requires an external server binary and workspace.
use crawrad_lsp::LspClient;
use std::path::Path;

let client = LspClient::spawn(None, Path::new(".")).unwrap();
assert!(client.is_alive());
```

# Errors

Returns [`LspError::ServerNotFound`] if `rust-analyzer` cannot be located,
or [`LspError::Io`] if spawning or pipe setup fails.
