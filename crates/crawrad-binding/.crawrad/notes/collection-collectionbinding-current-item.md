---
id: collection-collectionbinding-current-item
kind: doc
target_file: src/collection.rs
line_hint: 105
author: 
created: 
title: "CollectionBinding::current_item"
tags: []
signature: "pub fn current_item (& self) -> Option < T >"
---

Retrieve a clone of the currently selected item at the cursor.

# Examples

```rust
use crawrad_binding::CollectionBinding;

let binding = CollectionBinding::from_vec(vec!["first", "second"]);
assert_eq!(binding.current_item(), Some("first"));
```

# Panics

Panics if the collection mutex is poisoned.
