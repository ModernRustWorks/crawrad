---
id: client-lspclient-set-waker
kind: doc
target_file: src/client.rs
line_hint: 656
author: 
created: 
title: "LspClient::set_waker"
tags: []
signature: "pub fn set_waker (& self , wake : impl Fn () + Send + 'static)"
---

What wakes the window when the server reports progress.

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::LspClient;
use std::path::Path;

let client = LspClient::spawn(None, Path::new(".")).unwrap();
client.set_waker(|| {});
assert!(client.is_alive());
```
