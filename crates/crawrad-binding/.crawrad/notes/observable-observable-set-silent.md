---
id: observable-observable-set-silent
kind: doc
target_file: src/observable.rs
line_hint: 74
author: 
created: 
title: "Observable::set_silent"
tags: []
signature: "pub fn set_silent (& self , value : T)"
---

Sets a new value silently without notifying listeners.

Set a new value silently without notifying any listeners.
Useful to avoid cycle echoes in two-way bindings.

# Examples

```rust
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use crawrad_binding::Observable;

let obs = Observable::new(10);
let notified = Arc::new(AtomicBool::new(false));
let notified_clone = Arc::clone(&notified);
obs.subscribe(move |_| notified_clone.store(true, Ordering::SeqCst));
obs.set_silent(20);
assert_eq!(obs.get(), 20);
assert!(!notified.load(Ordering::SeqCst));
```

# Panics

Panics if the observable mutex is poisoned.
