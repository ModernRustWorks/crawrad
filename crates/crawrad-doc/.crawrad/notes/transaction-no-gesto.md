---
id: transaction-no-gesto
kind: doc
target_file: src/lib.rs
line_hint: 125
author: 
created: 
title: "Transaction::no_gesto"
tags: []
signature: "pub fn no_gesto (mut self , id : u64) -> Self"
---

Marks the transaction as a frame of gesture `id`.

Successive frames of the same gesture become **one** undo step. Callers
that do not invoke this continue stacking per transaction, which is right for
a discrete command: paste, delete, a keyboard arrow.

# Examples

```rust
use crawrad_doc::Transaction;

let tx = Transaction::user("drag").no_gesto(42);
assert_eq!(tx.gesto, Some(42));
```
