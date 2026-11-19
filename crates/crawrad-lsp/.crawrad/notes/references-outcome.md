---
id: references-outcome
kind: doc
target_file: src/references.rs
line_hint: 22
author: 
created: 
title: "Outcome"
tags: []
signature: "pub enum Outcome"
---

What came back for one request.

# Examples

```rust
use crawrad_lsp::references::Outcome;

let outcome = Outcome::Nothing;
assert_eq!(outcome, Outcome::Nothing);
```
