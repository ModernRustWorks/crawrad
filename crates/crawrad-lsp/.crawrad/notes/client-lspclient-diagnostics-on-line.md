---
id: client-lspclient-diagnostics-on-line
kind: doc
target_file: src/client.rs
line_hint: 734
author: 
created: 
title: "LspClient::diagnostics_on_line"
tags: []
signature: "pub fn diagnostics_on_line (& self , file : & Path , line : u32) -> Vec < serde_json :: Value >"
---

Returns published diagnostics for `file` that intersect `line`.

The diagnostics the server last published for `file` that touch
`line` (0-based), as it sent them.

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::LspClient;
use std::path::Path;

let client = LspClient::spawn(None, Path::new(".")).unwrap();
let diags = client.diagnostics_on_line(Path::new("src/lib.rs"), 0);
assert!(diags.is_empty());
```
