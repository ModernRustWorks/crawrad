---
id: definition-definitionworker-take
kind: doc
target_file: src/definition.rs
line_hint: 98
author: 
created: 
title: "DefinitionWorker::take"
tags: []
signature: "pub fn take (& self , ticket : u64) -> Option < DefinitionAnswer >"
---

The answer for `ticket`, if it arrived; any other is dropped.

# Examples

```rust
use crawrad_lsp::definition::DefinitionWorker;

let worker = DefinitionWorker::new();
assert!(worker.take(1).is_none());
```
