---
id: client-lspclient-completion-triggers
kind: doc
target_file: src/client.rs
line_hint: 710
author: 
created: 
title: "LspClient::completion_triggers"
tags: []
signature: "pub fn completion_triggers (& self) -> Vec < String >"
---

Returns the characters that trigger completion automatically.

The characters the server says open the completion list by itself
(LSP `CompletionOptions.triggerCharacters`); rust-analyzer: `: . ' (`.

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::LspClient;
use std::path::Path;

let client = LspClient::spawn(None, Path::new(".")).unwrap();
assert_eq!(client.completion_triggers().len(), 0);
```
