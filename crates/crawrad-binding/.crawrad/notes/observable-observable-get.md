---
id: observable-observable-get
kind: doc
target_file: src/observable.rs
line_hint: 107
author: 
created: 
title: "Observable::get"
tags: []
signature: "pub fn get (& self) -> T"
---

Get a clone of the current value.

# Examples

```rust
use crawrad_binding::Observable;

let obs = Observable::new(123);
assert_eq!(obs.get(), 123);
```
