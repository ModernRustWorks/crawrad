---
id: collection-collectionbinding-previous
kind: doc
target_file: src/collection.rs
line_hint: 190
author: 
created: 
title: "CollectionBinding::previous"
tags: []
signature: "pub fn previous (& self) -> bool"
---

Move the cursor to the previous element.

Returns true if cursor moved.

# Examples

```rust
use crawrad_binding::CollectionBinding;

let binding = CollectionBinding::from_vec(vec![1, 2]);
binding.last();
assert!(binding.previous());
assert_eq!(binding.cursor(), Some(0));
```
