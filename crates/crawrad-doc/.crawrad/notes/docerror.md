---
id: docerror
kind: doc
target_file: src/lib.rs
line_hint: 458
author: 
created: 
title: "DocError"
tags: []
signature: "pub enum DocError"
---

What the mutation gate rejects.

**The messages here are read by the user**, and that is why they are born in
ENGLISH: `Ide::aplicar` writes `rejected: <mensagem>` on the bar for each
rejected transaction, and `undo`/`redo` without a stack writes the raw phrase.
Comments and docs remain in Portuguese; what enforces the difference is
`crawrad-doc/src/portao_ingles.rs`.

# Examples

```rust
use crawrad_doc::DocError;

let err = DocError::EmptyTransaction;
assert_eq!(err.to_string(), "empty transaction");
```
