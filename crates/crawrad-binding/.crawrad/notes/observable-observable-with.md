---
id: observable-observable-with
kind: doc
target_file: src/observable.rs
line_hint: 68
author: 
created: 
title: "Observable::with"
tags: []
signature: "pub fn with < R > (& self , f : impl FnOnce (& T) -> R) -> R"
---

Read the current value via a borrowed closure.

# Examples

```rust
use crawrad_binding::Observable;

let obs = Observable::new(vec![1, 2, 3]);
let len = obs.with(|v| v.len());
assert_eq!(len, 3);
```

# Panics

Panics if the observable mutex is poisoned.
