---
id: client-lspclient-semantic-legend
kind: doc
target_file: src/client.rs
line_hint: 718
author: 
created: 
title: "LspClient::semantic_legend"
tags: []
signature: "pub fn semantic_legend (& self) -> crate :: semantic :: Legend"
---

The server's semantic token legend; empty when it has none.

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::LspClient;
use std::path::Path;

let client = LspClient::spawn(None, Path::new(".")).unwrap();
let legend = client.semantic_legend();
assert!(legend.types.is_empty());
```
