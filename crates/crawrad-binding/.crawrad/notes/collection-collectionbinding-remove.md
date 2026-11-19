---
id: collection-collectionbinding-remove
kind: doc
target_file: src/collection.rs
line_hint: 240
author: 
created: 
title: "CollectionBinding::remove"
tags: []
signature: "pub fn remove (& self , index : usize) -> Option < T >"
---

Remove an item at `index`.

Returns the removed item if index was valid.

# Examples

```rust
use crawrad_binding::CollectionBinding;

let binding = CollectionBinding::from_vec(vec![10, 20]);
assert_eq!(binding.remove(0), Some(10));
assert_eq!(binding.len(), 1);
```

# Panics

Panics if the collection mutex is poisoned.
