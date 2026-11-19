---
id: uri-path-to-uri
kind: doc
target_file: src/uri.rs
line_hint: 8
author: 
created: 
title: "path_to_uri"
tags: []
signature: "pub fn path_to_uri (path : & Path) -> String"
---

Converts a local filesystem path into an LSP file URI string (`file://...`).

# Examples

```rust
use crawrad_lsp::uri::path_to_uri;
use std::path::Path;

let uri = path_to_uri(Path::new("src/main.rs"));
assert!(uri.starts_with("file://"));
```
