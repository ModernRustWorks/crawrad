---
id: uri-uri-to-path
kind: doc
target_file: src/uri.rs
line_hint: 51
author: 
created: 
title: "uri_to_path"
tags: []
signature: "pub fn uri_to_path (uri : & str) -> Option < PathBuf >"
---

Converts an LSP `file://` URI string back into a local [`PathBuf`].

# Examples

```rust
use crawrad_lsp::uri::uri_to_path;

let path = uri_to_path("file:///path/to/file.rs");
assert!(path.is_some());
```
