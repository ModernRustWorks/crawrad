---
id: semantic-semanticworker
kind: doc
target_file: src/semantic.rs
line_hint: 201
author: 
created: 
title: "SemanticWorker"
tags: []
signature: "pub struct SemanticWorker"
---

Background worker asking for semantic tokens.

The thread that asks for semantic tokens ([`crate::asker`]), and the last
answer of each file, for the next delta.

# Examples

```rust
use crawrad_lsp::semantic::SemanticWorker;

let worker = SemanticWorker::new();
assert!(worker.take(1).is_none());
```
