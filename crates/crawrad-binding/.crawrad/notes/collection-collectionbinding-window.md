---
id: collection-collectionbinding-window
kind: doc
target_file: src/collection.rs
line_hint: 117
author: 
created: 
title: "CollectionBinding::window"
tags: []
signature: "pub fn window (& self , offset : usize , limit : usize) -> Vec < T >"
---

Extracts a virtual window of items for viewports.

Extract a virtual window of items (e.g. for GPU grid viewports).
Safe against out-of-bounds offsets.

# Examples

```rust
use crawrad_binding::CollectionBinding;

let binding = CollectionBinding::from_vec(vec![1, 2, 3, 4, 5]);
assert_eq!(binding.window(1, 2), vec![2, 3]);
```

# Panics

Panics if the collection mutex is poisoned.
