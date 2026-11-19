---
id: client-lspclient-sync-kind
kind: doc
target_file: src/client.rs
line_hint: 775
author: 
created: 
title: "LspClient::sync_kind"
tags: []
signature: "pub fn sync_kind (& self) -> crate :: sync :: SyncKind"
---

What the server accepts in `didChange`, as it answered `initialize`.

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::LspClient;
use crawrad_lsp::sync::SyncKind;
use std::path::Path;

let client = LspClient::spawn(None, Path::new(".")).unwrap();
assert_eq!(client.sync_kind(), SyncKind::Full);
```
