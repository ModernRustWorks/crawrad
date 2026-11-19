---
id: note-docproblem
kind: doc
target_file: src/note.rs
line_hint: 63
author: 
created: 
title: "DocProblem"
tags: []
signature: "pub struct DocProblem"
---

A problem found in a documentation note's markdown body.

# Examples

```rust
use crawrad_cli::note::DocProblem;

let prob = DocProblem {
    id: "my_doc".to_string(),
    file: "src/lib.rs".to_string(),
    line: 30,
    message: "summary missing period".to_string(),
};
assert_eq!(prob.line, 30);
```
