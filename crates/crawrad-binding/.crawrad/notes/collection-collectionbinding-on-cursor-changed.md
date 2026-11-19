---
id: collection-collectionbinding-on-cursor-changed
kind: doc
target_file: src/collection.rs
line_hint: 327
author: 
created: 
title: "CollectionBinding::on_cursor_changed"
tags: []
signature: "pub fn on_cursor_changed (& self , callback : impl Fn (Option < usize >) + Send + Sync + 'static ,) -> SubscriptionId"
---

Subscribe to changes in the active cursor index.

# Examples

```rust
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use crawrad_binding::CollectionBinding;

let binding = CollectionBinding::from_vec(vec![1, 2]);
let called = Arc::new(AtomicBool::new(false));
let called_clone = Arc::clone(&called);
binding.on_cursor_changed(move |_| called_clone.store(true, Ordering::SeqCst));
binding.next();
assert!(called.load(Ordering::SeqCst));
```

# Panics

Panics if the collection mutex is poisoned.
