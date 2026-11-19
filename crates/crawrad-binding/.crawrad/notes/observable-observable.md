---
id: observable-observable
kind: doc
target_file: src/observable.rs
line_hint: 28
author: 
created: 
title: "Observable"
tags: []
signature: "pub struct Observable < T >"
---

Thread-safe observable cell notifying subscribers on changes.

A thread-safe, cloneable observable cell that notifies subscribers when its value changes.

# Examples

```rust
use crawrad_binding::Observable;

let obs = Observable::new(10);
assert_eq!(obs.get(), 10);
```
