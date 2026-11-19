---
id: complete-completionanswer
kind: doc
target_file: src/complete.rs
line_hint: 57
author: 
created: 
title: "CompletionAnswer"
tags: []
signature: "pub struct CompletionAnswer"
---

What the worker answers, tagged with the ticket it was asked under.

# Examples

```rust
use crawrad_lsp::complete::CompletionAnswer;

let answer = CompletionAnswer {
    ticket: 1,
    items: Vec::new(),
    incomplete: false,
};
assert_eq!(answer.ticket, 1);
```
