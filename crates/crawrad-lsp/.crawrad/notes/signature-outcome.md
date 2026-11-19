---
id: signature-outcome
kind: doc
target_file: src/signature.rs
line_hint: 90
author: 
created: 
title: "Outcome"
tags: []
signature: "pub enum Outcome"
---

What came back for one request.

# Examples

```rust
use crawrad_lsp::signature::Outcome;

let outcome = Outcome::Nothing;
assert_eq!(outcome, Outcome::Nothing);
```
