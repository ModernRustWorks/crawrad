---
id: signature-signatureworker-take
kind: doc
target_file: src/signature.rs
line_hint: 170
author: 
created: 
title: "SignatureWorker::take"
tags: []
signature: "pub fn take (& self , ticket : u64) -> Option < SignatureAnswer >"
---

The answer for `ticket`, if it arrived; any other is dropped.

# Examples

```rust
use crawrad_lsp::signature::SignatureWorker;

let worker = SignatureWorker::new();
assert!(worker.take(42).is_none());
```
