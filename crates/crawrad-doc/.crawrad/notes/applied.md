---
id: applied
kind: doc
target_file: src/lib.rs
line_hint: 149
author: 
created: 
title: "Applied"
tags: []
signature: "pub struct Applied"
---

Represents the result of an applied transaction.

What a transaction did — it is what observers consume to update the
runtime, inspector, and tree without reloading the world.

# Examples

```rust
use std::collections::BTreeSet;
use crawrad_doc::{Applied, Author};

let applied = Applied {
    label: "init".into(),
    author: Author::User,
    touched: BTreeSet::new(),
    revision: 1,
};
assert_eq!(applied.revision, 1);
```
