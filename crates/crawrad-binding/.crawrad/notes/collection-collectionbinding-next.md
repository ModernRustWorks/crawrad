---
id: collection-collectionbinding-next
kind: doc
target_file: src/collection.rs
line_hint: 179
author: 
created: 
title: "CollectionBinding::next"
tags: []
signature: "pub fn next (& self) -> bool"
---

Advance the cursor to the next element.

Returns true if cursor moved.

# Examples

```rust
use crawrad_binding::CollectionBinding;

let binding = CollectionBinding::from_vec(vec![1, 2]);
assert!(binding.next());
assert_eq!(binding.cursor(), Some(1));
```
