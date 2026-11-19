---
id: collection-collectionbinding-set-cursor
kind: doc
target_file: src/collection.rs
line_hint: 128
author: 
created: 
title: "CollectionBinding::set_cursor"
tags: []
signature: "pub fn set_cursor (& self , new_cursor : Option < usize >) -> bool"
---

Move the cursor to a specific index.

Returns true if cursor was changed.

# Examples

```rust
use crawrad_binding::CollectionBinding;

let binding = CollectionBinding::from_vec(vec![1, 2, 3]);
assert!(binding.set_cursor(Some(2)));
assert_eq!(binding.cursor(), Some(2));
```

# Panics

Panics if the collection mutex is poisoned.
