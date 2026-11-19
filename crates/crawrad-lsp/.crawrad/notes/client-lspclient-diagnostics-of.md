---
id: client-lspclient-diagnostics-of
kind: doc
target_file: src/client.rs
line_hint: 746
author: 
created: 
title: "LspClient::diagnostics_of"
tags: []
signature: "pub fn diagnostics_of (& self , file : & Path) -> Option < Vec < crate :: protocol :: Diagnostic > >"
---

Returns all published diagnostics previously received for `file`.

Every diagnostic the server last published for `file`, read back from
what was kept raw (#361): a tab opened later is decorated from it.

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::LspClient;
use std::path::Path;

let client = LspClient::spawn(None, Path::new(".")).unwrap();
assert_eq!(client.diagnostics_of(Path::new("src/lib.rs")), None);
```
