---
id: observable-observable-subscribe
kind: doc
target_file: src/observable.rs
line_hint: 80
author: 
created: 
title: "Observable::subscribe"
tags: []
signature: "pub fn subscribe (& self , callback : impl Fn (& T) + Send + Sync + 'static) -> SubscriptionId"
---

Subscribe a callback to be invoked whenever the value changes.

# Examples

```rust
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;
use crawrad_binding::Observable;

let obs = Observable::new(1);
let current = Arc::new(AtomicI32::new(0));
let cur = Arc::clone(&current);
obs.subscribe(move |v| cur.store(*v, Ordering::SeqCst));
obs.set(42);
assert_eq!(current.load(Ordering::SeqCst), 42);
```

# Panics

Panics if the observable mutex is poisoned.
