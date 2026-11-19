---
id: symbols-symbolanswer
kind: doc
target_file: src/symbols.rs
line_hint: 31
author: 
created: 
title: "SymbolAnswer"
tags: []
signature: "pub struct SymbolAnswer"
---

The answer for one query: the items, or why there are none.

# Examples

```rust
use crawrad_lsp::symbols::SymbolAnswer;

let answer = SymbolAnswer {
    ticket: 1,
    outcome: Ok(Vec::new()),
};
assert_eq!(answer.ticket, 1);
```
