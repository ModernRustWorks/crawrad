---
id: protocol-publishdiagnosticsparams
kind: doc
target_file: src/protocol.rs
line_hint: 133
author: 
created: 
title: "PublishDiagnosticsParams"
tags: []
signature: "pub struct PublishDiagnosticsParams"
---

Parameters for the `textDocument/publishDiagnostics` notification.

# Examples

```rust
use crawrad_lsp::protocol::PublishDiagnosticsParams;

let params = PublishDiagnosticsParams {
    uri: "file:///src/lib.rs".to_string(),
    version: Some(1),
    diagnostics: Vec::new(),
};
assert_eq!(params.version, Some(1));
```
