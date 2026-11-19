---
id: client-lspclient-reload-workspace
kind: doc
target_file: src/client.rs
line_hint: 1089
author: 
created: 
title: "LspClient::reload_workspace"
tags: []
signature: "pub fn reload_workspace (& self) -> Result < () , LspError >"
---

Asks `rust-analyzer` to reload workspace `Cargo.toml` manifests.

Asks rust-analyzer to read the `Cargo.toml`s again
(`rust-analyzer/reloadWorkspace`). The answer comes when it is done,
so nothing waits for it: the status bar shows the progress.

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::LspClient;
use std::path::Path;

let client = LspClient::spawn(None, Path::new(".")).unwrap();
assert!(client.reload_workspace().is_ok());
```

# Errors

Returns [`LspError::ChannelClosed`] if sending the notification fails.
