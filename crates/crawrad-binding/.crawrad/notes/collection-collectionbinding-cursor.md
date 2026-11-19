---
id: collection-collectionbinding-cursor
kind: doc
target_file: src/collection.rs
line_hint: 99
author: 
created: 
title: "CollectionBinding::cursor"
tags: []
signature: "pub fn cursor (& self) -> Option < usize >"
---

Current active cursor index, if any.

# Examples

```rust
use crawrad_binding::CollectionBinding;

let binding = CollectionBinding::from_vec(vec![1, 2]);
assert_eq!(binding.cursor(), Some(0));
```

# Panics

Panics if the collection mutex is poisoned.
