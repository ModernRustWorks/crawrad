---
id: definition-definitionworker
kind: doc
target_file: src/definition.rs
line_hint: 49
author: 
created: 
title: "DefinitionWorker"
tags: []
signature: "pub struct DefinitionWorker (crate :: asker :: Asker < DefinitionAnswer >)"
---

The thread that asks where things are defined ([`crate::asker`]).

# Examples

```rust
use crawrad_lsp::definition::DefinitionWorker;

let worker = DefinitionWorker::new();
assert!(worker.take(1).is_none());
```
