---
id: collection-collectionchange
kind: doc
target_file: src/collection.rs
line_hint: 11
author: 
created: 
title: "CollectionChange"
tags: []
signature: "pub enum CollectionChange"
---

Description of mutations that occurred in a collection.

# Examples

```rust
use crawrad_binding::CollectionChange;

let change = CollectionChange::Reset;
assert_eq!(change, CollectionChange::Reset);
```
