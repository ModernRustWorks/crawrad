---
id: definition-outcome
kind: doc
target_file: src/definition.rs
line_hint: 29
author: 
created: 
title: "Outcome"
tags: []
signature: "pub enum Outcome"
---

What came back for one request.

# Examples

```rust
use crawrad_lsp::definition::Outcome;

let outcome = Outcome::Nothing;
assert_eq!(outcome, Outcome::Nothing);
```
