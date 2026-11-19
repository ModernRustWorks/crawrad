---
id: explorer-model-explanation
kind: doc
target_file: src/explorer_model.rs
line_hint: 209
author: 
created: 
title: "explanation"
tags: []
signature: "pub fn explanation (dono : Dono) -> & 'static str"
---

Returns the tooltip explanation for the given file owner.

The full sentence, for the tooltip. The badge fits in the column; the consequence does not.

# Examples

```rust
use crawrad_codegen::Dono;
use crawrad_project::explorer_model::explanation;

assert!(!explanation(Dono::Gerador).is_empty());
```
