---
id: collection-collectionbinding-last
kind: doc
target_file: src/collection.rs
line_hint: 169
author: 
created: 
title: "CollectionBinding::last"
tags: []
signature: "pub fn last (& self) -> bool"
---

Move the cursor to the last element.

# Examples

```rust
use crawrad_binding::CollectionBinding;

let binding = CollectionBinding::from_vec(vec![1, 2]);
assert!(binding.last());
assert_eq!(binding.cursor(), Some(1));
```
