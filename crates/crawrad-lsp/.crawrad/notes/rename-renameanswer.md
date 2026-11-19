---
id: rename-renameanswer
kind: doc
target_file: src/rename.rs
line_hint: 46
author: 
created: 
title: "RenameAnswer"
tags: []
signature: "pub struct RenameAnswer"
---

The answer, tagged with the ticket and the document version it was asked at.

# Examples

```rust
use crawrad_lsp::rename::{Outcome, RenameAnswer};
use std::path::PathBuf;

let answer = RenameAnswer {
    ticket: 1,
    file: PathBuf::from("src/main.rs"),
    version: 1,
    outcome: Outcome::NotRenamable,
};
assert_eq!(answer.ticket, 1);
```
