---
id: observable-observable-set-force
kind: doc
target_file: src/observable.rs
line_hint: 144
author: 
created: 
title: "Observable::set_force"
tags: []
signature: "pub fn set_force (& self , new_value : T)"
---

Force setting a new value and notifying listeners regardless of equality.

# Examples

```rust
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;
use crawrad_binding::Observable;

let obs = Observable::new(10);
let count = Arc::new(AtomicI32::new(0));
let count_clone = Arc::clone(&count);
obs.subscribe(move |_| { count_clone.fetch_add(1, Ordering::SeqCst); });
obs.set_force(10);
assert_eq!(count.load(Ordering::SeqCst), 1);
```

# Panics

Panics if the observable mutex is poisoned.
