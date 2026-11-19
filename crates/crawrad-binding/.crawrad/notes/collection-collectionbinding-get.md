---
id: collection-collectionbinding-get
kind: doc
target_file: src/collection.rs
line_hint: 111
author: 
created: 
title: "CollectionBinding::get"
tags: []
signature: "pub fn get (& self , index : usize) -> Option < T >"
---

Retrieve an item at an arbitrary index.

# Examples

```rust
use crawrad_binding::CollectionBinding;

let binding = CollectionBinding::from_vec(vec![10, 20]);
assert_eq!(binding.get(1), Some(20));
```

# Panics

Panics if the collection mutex is poisoned.
