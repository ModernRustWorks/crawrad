---
id: format-outcome
kind: doc
target_file: src/format.rs
line_hint: 19
author: 
created: 
title: "Outcome"
tags: []
signature: "pub enum Outcome"
---

What came back.

# Examples

```rust
use crawrad_lsp::format::Outcome;

let outcome = Outcome::Edits(Vec::new());
assert_eq!(outcome, Outcome::Edits(Vec::new()));
```
