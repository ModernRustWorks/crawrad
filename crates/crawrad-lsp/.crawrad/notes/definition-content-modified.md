---
id: definition-content-modified
kind: doc
target_file: src/definition.rs
line_hint: 123
author: 
created: 
title: "CONTENT_MODIFIED"
tags: []
signature: "pub const CONTENT_MODIFIED : i64 = - 32801"
---

LSP 3.17 error code indicating server state changed during request.

LSP 3.17's `ContentModified`: the server dropped the request because
its state changed under it (rust-analyzer while it loads the
workspace). Not an error to show: the request is asked again.

# Examples

```rust
use crawrad_lsp::definition::CONTENT_MODIFIED;

assert_eq!(CONTENT_MODIFIED, -32801);
```
