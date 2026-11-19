---
id: irerror
kind: doc
target_file: src/lib.rs
line_hint: 630
author: 
created: 
title: "IrError"
tags: []
signature: "pub enum IrError"
---

What the document rejects.

# Examples

```rust
use crawrad_ir::IrError;

let err = IrError::not_snake_case("btnOK");
assert!(matches!(err, IrError::NameNotSnakeCase { .. }));
```

**The messages here are read by the user**, and therefore are born in
ENGLISH: the shell writes them in full to the status bar —
`Document::load` and `save` via the file path, and `Ide::apply`
via `rejected: <message>`, since every `IrError` arrives there wrapped in a
`DocError::Ir`. Comments and docs remain in Portuguese; what enforces the
difference is `crawrad-doc/src/portao_ingles.rs`.

A good message says **what failed and what to do** — see
`NameWithoutAlnum`, which is not satisfied with merely rejecting.
