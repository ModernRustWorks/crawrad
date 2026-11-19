---
id: client-lspclient-position-encoding
kind: doc
target_file: src/client.rs
line_hint: 703
author: 
created: 
title: "LspClient::position_encoding"
tags: []
signature: "pub fn position_encoding (& self) -> crate :: encoding :: PositionEncoding"
---

Returns the column unit negotiated during initialization.

The column unit negotiated in `initialize` (`utf-8` with
rust-analyzer; UTF-16 when the server says nothing).

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::LspClient;
use crawrad_lsp::encoding::PositionEncoding;
use std::path::Path;

let client = LspClient::spawn(None, Path::new(".")).unwrap();
assert_eq!(client.position_encoding(), PositionEncoding::Utf16);
```
