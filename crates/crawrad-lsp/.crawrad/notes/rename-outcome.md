---
id: rename-outcome
kind: doc
target_file: src/rename.rs
line_hint: 23
author: 
created: 
title: "Outcome"
tags: []
signature: "pub enum Outcome"
---

What came back for one request.

# Examples

```rust
use crawrad_lsp::rename::Outcome;

let outcome = Outcome::NotRenamable;
assert_eq!(outcome, Outcome::NotRenamable);
```
