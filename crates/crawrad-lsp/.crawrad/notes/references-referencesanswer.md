---
id: references-referencesanswer
kind: doc
target_file: src/references.rs
line_hint: 33
author: 
created: 
title: "ReferencesAnswer"
tags: []
signature: "pub struct ReferencesAnswer"
---

The answer, tagged with the ticket and the document version it was asked at.

# Examples

```rust
use crawrad_lsp::references::{Outcome, ReferencesAnswer};
use std::path::PathBuf;

let answer = ReferencesAnswer {
    ticket: 1,
    file: PathBuf::from("src/main.rs"),
    version: 1,
    outcome: Outcome::Nothing,
};
assert_eq!(answer.ticket, 1);
```
