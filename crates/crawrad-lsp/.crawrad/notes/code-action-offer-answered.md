---
id: code-action-offer-answered
kind: doc
target_file: src/code_action.rs
line_hint: 85
author: 
created: 
title: "Offer::answered"
tags: []
signature: "pub fn answered (& mut self , answer : CodeActionAnswer) -> Option < bool >"
---

Records an answer for the pending ticket, returning if asked by key.

The answer for the pending ticket: kept as `here`. Returns whether it
was asked by key (then its list opens), once.

# Examples

```rust
use crawrad_lsp::code_action::{CodeActionAnswer, Offer};
use std::path::PathBuf;

let mut offer = Offer::default();
let place = (PathBuf::from("src/lib.rs"), 1, (0, 0));
let ticket = offer.ask(place, true);
let answer = CodeActionAnswer {
    ticket,
    file: PathBuf::from("src/lib.rs"),
    version: 1,
    caret: (0, 0),
    actions: Ok(Vec::new()),
};
assert_eq!(offer.answered(answer), Some(true));
```
