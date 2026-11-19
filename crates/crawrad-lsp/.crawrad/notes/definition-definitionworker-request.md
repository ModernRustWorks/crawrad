---
id: definition-definitionworker-request
kind: doc
target_file: src/definition.rs
line_hint: 68
author: 
created: 
title: "DefinitionWorker::request"
tags: []
signature: "pub fn request (& self , client : Arc < LspClient > , ticket : u64 , file : & Path , version : i32 , text : & str , caret : (usize , usize) ,) -> bool"
---

Asks where the symbol at `caret` of `file` is defined.

Asks where the symbol at `caret` (line, byte) of `file` — its `text`
at `version` — is defined.

# Examples

```rust,no_run
// Requesting definitions requires an active language server client.
use crawrad_lsp::LspClient;
use crawrad_lsp::definition::DefinitionWorker;
use std::path::Path;
use std::sync::Arc;

let client = Arc::new(LspClient::spawn(None, Path::new(".")).unwrap());
let worker = DefinitionWorker::new();
assert!(worker.request(client, 1, Path::new("src/lib.rs"), 1, "fn main() {}", (0, 0)));
```
