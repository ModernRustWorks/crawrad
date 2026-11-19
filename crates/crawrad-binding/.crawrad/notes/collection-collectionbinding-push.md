---
id: collection-collectionbinding-push
kind: doc
target_file: src/collection.rs
line_hint: 199
author: 
created: 
title: "CollectionBinding::push"
tags: []
signature: "pub fn push (& self , item : T)"
---

Append an item to the collection.

# Examples

```rust
use crawrad_binding::CollectionBinding;

let binding = CollectionBinding::new();
binding.push(42);
assert_eq!(binding.len(), 1);
```

# Panics

Panics if the collection mutex is poisoned.
