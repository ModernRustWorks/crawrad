---
id: collection-collectionbinding-is-empty
kind: doc
target_file: src/collection.rs
line_hint: 94
author: 
created: 
title: "CollectionBinding::is_empty"
tags: []
signature: "pub fn is_empty (& self) -> bool"
---

Returns true if the collection has no items.

# Examples

```rust
use crawrad_binding::CollectionBinding;

let binding: CollectionBinding<i32> = CollectionBinding::new();
assert!(binding.is_empty());
```
