---
id: observable-observable-set
kind: doc
target_file: src/observable.rs
line_hint: 114
author: 
created: 
title: "Observable::set"
tags: []
signature: "pub fn set (& self , new_value : T) -> bool"
---

Set a new value and notify listeners only if the value actually changed.

# Examples

```rust
use crawrad_binding::Observable;

let obs = Observable::new(10);
assert!(obs.set(20));
assert!(!obs.set(20));
assert_eq!(obs.get(), 20);
```

# Panics

Panics if the observable mutex is poisoned.
