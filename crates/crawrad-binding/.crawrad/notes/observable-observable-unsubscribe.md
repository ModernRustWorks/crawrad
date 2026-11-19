---
id: observable-observable-unsubscribe
kind: doc
target_file: src/observable.rs
line_hint: 88
author: 
created: 
title: "Observable::unsubscribe"
tags: []
signature: "pub fn unsubscribe (& self , id : SubscriptionId) -> bool"
---

Remove an active subscription by its id.

# Examples

```rust
use crawrad_binding::Observable;

let obs = Observable::new(1);
let sub = obs.subscribe(|_| {});
assert!(obs.unsubscribe(sub));
assert_eq!(obs.subscriber_count(), 0);
```

# Panics

Panics if the observable mutex is poisoned.
