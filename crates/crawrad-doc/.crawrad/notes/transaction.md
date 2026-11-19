---
id: transaction
kind: doc
target_file: src/lib.rs
line_hint: 93
author: 
created: 
title: "Transaction"
tags: []
signature: "pub struct Transaction"
---

Um lote de comandos que desfaz junto.

# Examples

```rust
use crawrad_doc::Transaction;

let tx = Transaction::user("move button");
assert_eq!(tx.label, "move button");
```
