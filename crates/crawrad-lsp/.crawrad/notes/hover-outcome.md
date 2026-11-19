---
id: hover-outcome
kind: doc
target_file: src/hover.rs
line_hint: 24
author: 
created: 
title: "Outcome"
tags: []
signature: "pub enum Outcome"
---

What came back for one request.

# Examples

```rust
use crawrad_lsp::hover::Outcome;

let outcome = Outcome::Nothing;
assert_eq!(outcome, Outcome::Nothing);
```
