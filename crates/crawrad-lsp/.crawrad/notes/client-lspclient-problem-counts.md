---
id: client-lspclient-problem-counts
kind: doc
target_file: src/client.rs
line_hint: 757
author: 
created: 
title: "LspClient::problem_counts"
tags: []
signature: "pub fn problem_counts (& self) -> Vec < (String , usize , usize) >"
---

Returns error and warning counts per file as last published.

Errors and warnings per file, as the server last published them —
the Explorer colours its rows by them (#412): (file key, errors,
warnings), files with neither left out.

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::LspClient;
use std::path::Path;

let client = LspClient::spawn(None, Path::new(".")).unwrap();
assert!(client.problem_counts().is_empty());
```
