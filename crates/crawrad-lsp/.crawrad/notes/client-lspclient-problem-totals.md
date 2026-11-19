---
id: client-lspclient-problem-totals
kind: doc
target_file: src/client.rs
line_hint: 765
author: 
created: 
title: "LspClient::problem_totals"
tags: []
signature: "pub fn problem_totals (& self) -> Option < (usize , usize) >"
---

Returns total error and warning counts across all files, if any.

Errors and warnings over every file, as last published — or `None`
before the server has published anything: until then a zero would be
a verdict nobody gave.

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::LspClient;
use std::path::Path;

let client = LspClient::spawn(None, Path::new(".")).unwrap();
assert_eq!(client.problem_totals(), None);
```
