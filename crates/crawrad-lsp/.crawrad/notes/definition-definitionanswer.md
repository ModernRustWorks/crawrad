---
id: definition-definitionanswer
kind: doc
target_file: src/definition.rs
line_hint: 40
author: 
created: 
title: "DefinitionAnswer"
tags: []
signature: "pub struct DefinitionAnswer"
---

The answer, tagged with the ticket and the document version it was asked at.

# Examples

```rust
use crawrad_lsp::definition::{DefinitionAnswer, Outcome};
use std::path::PathBuf;

let answer = DefinitionAnswer {
    ticket: 1,
    file: PathBuf::from("src/lib.rs"),
    version: 1,
    outcome: Outcome::Nothing,
};
assert_eq!(answer.ticket, 1);
```
