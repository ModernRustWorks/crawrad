---
id: collection-collectionbinding-len
kind: doc
target_file: src/collection.rs
line_hint: 88
author: 
created: 
title: "CollectionBinding::len"
tags: []
signature: "pub fn len (& self) -> usize"
---

Total number of items in the collection.

# Examples

```rust
use crawrad_binding::CollectionBinding;

let binding = CollectionBinding::from_vec(vec!["a", "b"]);
assert_eq!(binding.len(), 2);
```

# Panics

Panics if the collection mutex is poisoned.
