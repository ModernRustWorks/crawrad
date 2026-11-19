---
id: client-file-key
kind: doc
target_file: src/client.rs
line_hint: 1209
author: 
created: 
title: "file_key"
tags: []
signature: "pub fn file_key (file : & Path) -> String"
---

Formats a file path as a normalized diagnostic storage key.

A file as the published diagnostics are kept: the server writes the drive
in lower case (`file:///c:/…`), this client in upper case, so on Windows
the key is folded.

# Examples

```rust
use crawrad_lsp::client::file_key;
use std::path::Path;

let key = file_key(Path::new("src/lib.rs"));
assert_eq!(key, "src/lib.rs");
```
