---
id: collection-collectionbinding-from-vec
kind: doc
target_file: src/collection.rs
line_hint: 75
author: 
created: 
title: "CollectionBinding::from_vec"
tags: []
signature: "pub fn from_vec (items : Vec < T >) -> Self"
---

Create a collection binding initialized from an existing `Vec<T>`.

# Examples

```rust
use crawrad_binding::CollectionBinding;

let binding = CollectionBinding::from_vec(vec![10, 20]);
assert_eq!(binding.cursor(), Some(0));
```
