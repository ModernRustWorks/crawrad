---
id: collection-collectionbinding-reset
kind: doc
target_file: src/collection.rs
line_hint: 293
author: 
created: 
title: "CollectionBinding::reset"
tags: []
signature: "pub fn reset (& self , new_items : Vec < T >)"
---

Reset the collection with new items.

# Examples

```rust
use crawrad_binding::CollectionBinding;

let binding = CollectionBinding::from_vec(vec![1, 2]);
binding.reset(vec![3, 4, 5]);
assert_eq!(binding.len(), 3);
```

# Panics

Panics if the collection mutex is poisoned.
