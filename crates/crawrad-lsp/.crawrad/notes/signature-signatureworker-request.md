---
id: signature-signatureworker-request
kind: doc
target_file: src/signature.rs
line_hint: 138
author: 
created: 
title: "SignatureWorker::request"
tags: []
signature: "pub fn request (& self , client : Arc < LspClient > , ticket : u64 , asked : & Asked < '_ >) -> bool"
---

Asks which signature the call around the caret is to.

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::client::LspClient;
use crawrad_lsp::signature::{Asked, SignatureWorker, Trigger};
use std::path::Path;
use std::sync::Arc;

fn req_sig(worker: &SignatureWorker, client: Arc<LspClient>) {
    let file = Path::new("src/main.rs");
    let asked = Asked {
        file,
        version: 1,
        text: "foo()\n",
        caret: (0, 4),
        trigger: Trigger::Invoked,
        retrigger: false,
    };
    let queued = worker.request(client, 1, &asked);
    assert!(queued);
}
```
