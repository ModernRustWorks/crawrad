---
id: collection-collectionbinding
kind: doc
target_file: src/collection.rs
line_hint: 34
author: 
created: 
title: "CollectionBinding"
tags: []
signature: "pub struct CollectionBinding < T >"
---

Observable collection with cursor navigation and virtual window slicing.

An observable collection supporting cursor navigation and virtual window slicing.

# Examples

```rust
use crawrad_binding::CollectionBinding;

let binding = CollectionBinding::from_vec(vec![1, 2, 3]);
assert_eq!(binding.len(), 3);
```
