---
id: client-lspclient-signature-triggers
kind: doc
target_file: src/client.rs
line_hint: 726
author: 
created: 
title: "LspClient::signature_triggers"
tags: []
signature: "pub fn signature_triggers (& self) -> Vec < String >"
---

Returns characters that trigger or update signature help.

The characters that open or move the parameter card (LSP
`SignatureHelpOptions`); rust-analyzer: `( , <`.

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::LspClient;
use std::path::Path;

let client = LspClient::spawn(None, Path::new(".")).unwrap();
assert_eq!(client.signature_triggers().len(), 0);
```
