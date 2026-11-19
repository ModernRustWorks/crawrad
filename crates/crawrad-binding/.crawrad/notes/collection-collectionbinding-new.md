---
id: collection-collectionbinding-new
kind: doc
target_file: src/collection.rs
line_hint: 63
author: 
created: 
title: "CollectionBinding::new"
tags: []
signature: "pub fn new () -> Self"
---

Create an empty collection binding.

# Examples

```rust
use crawrad_binding::CollectionBinding;

let binding: CollectionBinding<i32> = CollectionBinding::new();
assert_eq!(binding.len(), 0);
```
