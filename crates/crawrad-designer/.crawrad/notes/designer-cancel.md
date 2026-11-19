---
id: designer-cancel
kind: doc
target_file: src/lib.rs
line_hint: 940
author: 
created: 
title: "Designer::cancel"
tags: []
signature: "pub fn cancel (& mut self) -> Outcome"
---

Cancels the drag and returns the transaction that restores initial state.

# Examples

```rust
use crawrad_designer::Designer;

let mut d = Designer::default();
let outcome = d.cancel();
assert!(outcome.tx.is_none());
```
