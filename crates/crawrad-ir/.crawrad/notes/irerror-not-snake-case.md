---
id: irerror-not-snake-case
kind: doc
target_file: src/lib.rs
line_hint: 706
author: 
created: 
title: "IrError::not_snake_case"
tags: []
signature: "pub fn not_snake_case (name : impl Into < String >) -> Self"
---

Rejection of a `Name` outside `snake_case`, with the advice already assembled.

# Examples

```rust
use crawrad_ir::IrError;

let err = IrError::not_snake_case("btnOK");
assert!(matches!(err, IrError::NameNotSnakeCase { .. }));
```

**Constructor, and not a variant assembled by hand**, so that the sentence the
user reads is born in one place. `Command::Rename` is what rejects it today;
the day another path needs to reject the same thing, it will not write
a second wording of the same thing — which is how two messages for the
same defect age, one of them silenced.
