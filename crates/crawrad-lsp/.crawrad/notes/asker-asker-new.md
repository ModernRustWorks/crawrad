---
id: asker-asker-new
kind: doc
target_file: src/asker.rs
line_hint: 23
author: 
created: 
title: "Asker::new"
tags: []
signature: "pub fn new (name : & str) -> Self"
---

A thread named `name` that runs the requests.

# Examples

```rust
use crawrad_lsp::asker::Asker;

let asker = Asker::<i32>::new("worker");
assert!(asker.ask(1, || 42));
```
