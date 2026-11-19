---
id: client-lspclient-resolve-code-lens
kind: doc
target_file: src/client.rs
line_hint: 609
author: 
created: 
title: "LspClient::resolve_code_lens"
tags: []
signature: "pub fn resolve_code_lens (& self , lens : & CodeLens , timeout : Duration , superseded : & mut dyn FnMut () -> bool ,) -> Result < CodeLens , LspError >"
---

Resolves an unresolved code lens, filling its command.

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::LspClient;
use crawrad_lsp::code_lens::CodeLens;
use crawrad_lsp::protocol::{Position, Range};
use std::path::Path;
use std::time::Duration;

let client = LspClient::spawn(None, Path::new(".")).unwrap();
let lens = CodeLens::new(Range::new(Position::new(0, 0), Position::new(0, 0)), None);
let res = client.resolve_code_lens(&lens, Duration::from_secs(1), &mut || false);
assert!(res.is_err());
```

# Errors

Returns [`LspError::RpcError`] if resolution fails, or [`LspError::Cancelled`], [`LspError::Timeout`].
