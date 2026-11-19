---
id: definition-definitionworker-set-waker
kind: doc
target_file: src/definition.rs
line_hint: 63
author: 
created: 
title: "DefinitionWorker::set_waker"
tags: []
signature: "pub fn set_waker (& self , wake : impl Fn () + Send + 'static)"
---

What wakes the window when an answer arrives.

# Examples

```rust
use crawrad_lsp::definition::DefinitionWorker;

let worker = DefinitionWorker::new();
worker.set_waker(|| {});
assert!(worker.take(1).is_none());
```
