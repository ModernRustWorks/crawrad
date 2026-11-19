---
id: collection-collectionbinding-on-collection-changed
kind: doc
target_file: src/collection.rs
line_hint: 338
author: 
created: 
title: "CollectionBinding::on_collection_changed"
tags: []
signature: "pub fn on_collection_changed (& self , callback : impl Fn (& CollectionChange) + Send + Sync + 'static ,) -> SubscriptionId"
---

Subscribe to mutation events in the collection.

# Examples

```rust
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use crawrad_binding::CollectionBinding;

let binding = CollectionBinding::new();
let called = Arc::new(AtomicBool::new(false));
let called_clone = Arc::clone(&called);
binding.on_collection_changed(move |_| called_clone.store(true, Ordering::SeqCst));
binding.push(10);
assert!(called.load(Ordering::SeqCst));
```

# Panics

Panics if the collection mutex is poisoned.
