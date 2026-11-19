---
id: observable-observable-subscriber-count
kind: doc
target_file: src/observable.rs
line_hint: 99
author: 
created: 
title: "Observable::subscriber_count"
tags: []
signature: "pub fn subscriber_count (& self) -> usize"
---

Returns the number of active subscribers.

# Examples

```rust
use crawrad_binding::Observable;

let obs = Observable::new(1);
assert_eq!(obs.subscriber_count(), 0);
let _sub = obs.subscribe(|_| {});
assert_eq!(obs.subscriber_count(), 1);
```

# Panics

Panics if the observable mutex is poisoned.
