---
id: collection-collectionbinding-first
kind: doc
target_file: src/collection.rs
line_hint: 159
author: 
created: 
title: "CollectionBinding::first"
tags: []
signature: "pub fn first (& self) -> bool"
---

Move the cursor to the first element.

# Examples

```rust
use crawrad_binding::CollectionBinding;

let binding = CollectionBinding::from_vec(vec![1, 2]);
binding.set_cursor(Some(1));
assert!(binding.first());
assert_eq!(binding.cursor(), Some(0));
```
