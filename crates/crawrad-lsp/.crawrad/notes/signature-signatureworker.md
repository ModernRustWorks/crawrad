---
id: signature-signatureworker
kind: doc
target_file: src/signature.rs
line_hint: 108
author: 
created: 
title: "SignatureWorker"
tags: []
signature: "pub struct SignatureWorker (crate :: asker :: Asker < SignatureAnswer >)"
---

The thread that asks for signatures ([`crate::asker`]).

# Examples

```rust
use crawrad_lsp::signature::SignatureWorker;

let worker = SignatureWorker::new();
assert!(worker.take(1).is_none());
```
