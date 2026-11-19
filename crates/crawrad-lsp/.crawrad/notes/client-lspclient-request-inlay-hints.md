---
id: client-lspclient-request-inlay-hints
kind: doc
target_file: src/client.rs
line_hint: 574
author: 
created: 
title: "LspClient::request_inlay_hints"
tags: []
signature: "pub fn request_inlay_hints (& self , file_path : & Path , end : Position , timeout : Duration , superseded : & mut dyn FnMut () -> bool ,) -> Result < Vec < InlayHint > , LspError >"
---

Requests inlay hints for `file_path` from start up to `end`.

The inlay hints of `file` from its start to `end` (its last line and
that line's length in UTF-16) — the types and parameter names the code
does not write (issue #255). Measured on 01/10/2026: an end one line
past the text is refused (`Invalid offset LineCol { line: 31, col: 0 }`).

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::LspClient;
use crawrad_lsp::protocol::Position;
use std::path::Path;
use std::time::Duration;

let client = LspClient::spawn(None, Path::new(".")).unwrap();
let hints = client.request_inlay_hints(Path::new("src/lib.rs"), Position::new(10, 0), Duration::from_secs(1), &mut || false).unwrap();
assert!(hints.is_empty());
```

# Errors

Returns [`LspError::Cancelled`], [`LspError::Timeout`], or [`LspError::RpcError`].
